//! Turns a [`Workshop`] into plain data, applies the build profile, and prints it.

use crate::{
    Action, ActionId, Condition, Event, Extension, Global, Globals, PlayerVar, Players,
    SettingsConflict, Value, ValueId, Workshop, emit, settings,
};

/// Which kind of script to produce.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Profile {
    /// Every rule, comments, and debug helpers.
    Debug,
    /// Only what the mod needs to run.
    Release,
}

pub(crate) enum Trigger {
    Event(Event),
    Subroutine(String),
}

pub(crate) struct RuleData {
    pub name: String,
    pub enabled: bool,
    pub trigger: Trigger,
    pub conditions: Vec<Condition>,
    pub actions: Vec<Action>,
}

pub(crate) struct Script {
    pub settings: settings::Resolved,
    pub extensions: Vec<Extension>,
    pub globals: Vec<Global>,
    pub players: Vec<PlayerVar>,
    pub subroutines: Vec<String>,
    pub rules: Vec<RuleData>,
}

/// The workshop script for `workshop` under `profile`.
///
/// # Panics
///
/// Panics with the [`SettingsConflict`] message when two settings sources disagree,
/// which makes it a build error when called from a build script.
pub fn script<M: Workshop>(workshop: &M, profile: Profile) -> String {
    try_script(workshop, profile).unwrap_or_else(|conflict| panic!("{conflict}"))
}

/// Like [`script`], returning a conflict between settings instead of panicking.
#[allow(clippy::result_large_err)]
pub fn try_script<M: Workshop>(workshop: &M, profile: Profile) -> Result<String, SettingsConflict> {
    let mut script = collect::<M>(workshop, profile)?;
    match profile {
        Profile::Debug => script.rules.extend(variable_panels(&script)),
        Profile::Release => {
            script.rules.retain(|rule| rule.enabled);
            for rule in &mut script.rules {
                rule.actions = strip_comments(std::mem::take(&mut rule.actions));
            }
        }
    }
    Ok(emit::write(&script))
}

#[allow(clippy::result_large_err)]
fn collect<M: Workshop>(workshop: &M, profile: Profile) -> Result<Script, SettingsConflict> {
    let globals = M::Globals::declare();
    let players = M::Players::declare();
    let log = |name: &str, opts: Option<crate::DebugOpts>, actions: &mut Vec<Action>| {
        if profile == Profile::Debug && opts.is_some_and(|opts| opts.log_to_inspector) {
            let line = Value::string(name, []);
            actions.insert(0, Action::call(ActionId::LogToInspector, [line]));
        }
    };

    let mut rules = Vec::new();
    for rule in workshop.rules(&globals, &players) {
        let mut actions = rule.actions();
        log(&rule.name(), rule.debug_opts(), &mut actions);
        for event in rule.events() {
            rules.push(RuleData {
                name: rule.name(),
                enabled: rule.enabled(),
                trigger: Trigger::Event(event),
                conditions: rule.conditions(),
                actions: actions.clone(),
            });
        }
    }
    let mut subroutines = Vec::new();
    for subroutine in workshop.subroutines(&globals, &players) {
        let name = subroutine.name();
        let mut actions = subroutine.actions();
        log(&name, subroutine.debug_opts(), &mut actions);
        rules.push(RuleData {
            name: format!("Subroutine: {name}"),
            enabled: true,
            trigger: Trigger::Subroutine(name.clone()),
            conditions: Vec::new(),
            actions,
        });
        subroutines.push(name);
    }

    let mut extensions: Vec<Extension> = Vec::new();
    for extension in M::extensions() {
        if !extensions.contains(&extension) {
            extensions.push(extension);
        }
    }
    let settings = settings::resolve(
        M::NAME,
        M::DESCRIPTION,
        &[M::lobby()],
        &M::modes(),
        &M::heroes(),
    )?;

    Ok(Script {
        settings,
        extensions,
        globals: globals.vars(),
        players: players.vars(),
        subroutines,
        rules,
    })
}

fn strip_comments(actions: Vec<Action>) -> Vec<Action> {
    actions
        .into_iter()
        .map(|action| match action {
            Action::Commented { action, .. } => strip_comments(vec![*action]).remove(0),
            Action::If {
                cond,
                then,
                otherwise,
            } => Action::If {
                cond,
                then: strip_comments(then),
                otherwise: strip_comments(otherwise),
            },
            other => other,
        })
        .collect()
}

/// One HUD line per variable that declares debug options.
fn variable_panels(script: &Script) -> Vec<RuleData> {
    let all_players = Value::call(ValueId::GetPlayers, [Value::Constant("All Teams")]);
    let global_lines: Vec<Action> = script
        .globals
        .iter()
        .filter_map(|var| {
            Some(hud_line(
                all_players.clone(),
                var.name,
                var.debug.as_ref()?,
                var.into(),
            ))
        })
        .collect();
    let player_lines: Vec<Action> = script
        .players
        .iter()
        .filter_map(|var| {
            let value = var.of(Value::event_player());
            Some(hud_line(
                Value::event_player(),
                var.name,
                var.debug.as_ref()?,
                value,
            ))
        })
        .collect();

    let panel = |name: &str, event, actions: Vec<Action>| {
        (!actions.is_empty()).then(|| RuleData {
            name: name.to_owned(),
            enabled: true,
            trigger: Trigger::Event(event),
            conditions: Vec::new(),
            actions,
        })
    };
    [
        panel(
            "Debug: global variables",
            Event::OngoingGlobal,
            global_lines,
        ),
        panel(
            "Debug: player variables",
            Event::each_player(),
            player_lines,
        ),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn hud_line(visible_to: Value, name: &str, opts: &crate::DebugOpts, value: Value) -> Action {
    let label = opts.label.as_deref().unwrap_or(name);
    let white = || Value::Constant("Color(White)");
    Action::call(
        ActionId::HudText,
        [
            visible_to,
            Value::null(),
            Value::null(),
            Value::string(format!("{label}: {{0}}"), [value]),
            Value::Constant(opts.hud.name()),
            opts.sort_order.into(),
            white(),
            white(),
            white(),
            Value::Constant("Visible To and String"),
            Value::Constant("Default Visibility"),
        ],
    )
}
