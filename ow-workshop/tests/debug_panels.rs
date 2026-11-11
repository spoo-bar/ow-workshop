//! The on-screen debug panels of a debug build, and how they give way to the element limit.
// `insta` reads and writes files, which WASM does not have.
#![cfg(not(target_arch = "wasm32"))]

use ow_workshop::prelude::*;
use ow_workshop::{ErrorKind, Kind, Warning};

#[derive(GlobalVars)]
struct Globals {
    wave: Global<Num>,
    #[ow(debug_label = "Phase")]
    state: Global<Str>,
    #[ow(debug = false)]
    #[allow(dead_code)]
    internal: Global<Num>,
    spawns: Global<Array<Num>>,
}

#[derive(PlayerVars)]
struct Players {
    kills: PlayerVar<Num>,
    alive: PlayerVar<Bool>,
}

/// A small mod that reads every variable once, with `filler` extra rules of one action each.
struct Mode {
    filler: usize,
}

impl Workshop for Mode {
    const NAME: &'static str = "Test";
    const DESCRIPTION: &'static str = "Test mode";

    fn settings(&self) -> ow_workshop::Settings {
        ow_workshop::Settings::unchecked()
    }

    type Globals = Globals;
    type Players = Players;

    fn rules<'a>(&'a self, g: &'a Globals, p: &'a Players) -> Vec<RuleDef<'a>> {
        mode_rules(self.filler, g, p)
    }
}

fn mode_rules<'a>(filler: usize, g: &'a Globals, p: &'a Players) -> Vec<RuleDef<'a>> {
    {
        let mut defs = Vec::new();

        defs.push(rule("Run", OngoingGlobal, |r, _| {
            r.actions(|b| {
                b.set(g.wave, 1);
                b.set(g.state, "playing");
                b.set(g.spawns, [1, 2, 3]);
            });
        }));
        defs.push(rule("Score", OngoingEachPlayer, |r, ev| {
            r.actions(|b| {
                b.set(ev.event_player().var(p.kills), 0);
                b.set(ev.event_player().var(p.alive), true);
            });
        }));
        for i in 0..filler {
            defs.push(rule(format!("Filler {i}"), OngoingGlobal, move |r, _| {
                r.actions(|b| b.big_message(u32::try_from(i).expect("small")));
            }));
        }
        defs
    }
}

fn compile(filler: usize, debug: DebugMode) -> Result<CompileOutput, Diagnostics> {
    Mode { filler }.compile_with_debug(Lang::EnUs, Some(debug))
}

fn on() -> DebugOptions {
    DebugOptions::default()
}

/// `DebugOptions` is non-exhaustive, so a test changes the defaults instead of listing fields.
fn on_with(change: impl FnOnce(&mut DebugOptions)) -> DebugOptions {
    let mut options = on();
    change(&mut options);
    options
}

#[test]
fn both_panels_of_a_small_mod() {
    let out = compile(0, DebugMode::On(on())).expect("compiles");
    assert_eq!(out.warnings, vec![]);
    insta::assert_snapshot!(out.code);
}

#[test]
fn the_panels_can_be_held_behind_a_button() {
    let options = on_with(|o| o.toggle = Some(Button::Interact));
    let out = compile(0, DebugMode::On(options)).expect("compiles");
    insta::assert_snapshot!(out.code);
}

#[test]
fn each_panel_can_be_turned_off() {
    let rules = |options: DebugOptions| {
        let code = compile(0, DebugMode::On(options)).expect("compiles").code;
        (
            code.contains("rule (\"Debug: globals\")"),
            code.contains("rule (\"Debug: players\")"),
        )
    };
    assert_eq!(rules(on()), (true, true));
    assert_eq!(rules(on_with(|o| o.hud = false)), (false, true));
    assert_eq!(rules(on_with(|o| o.player_hud = false)), (true, false));
}

#[test]
fn the_panels_come_before_the_users_rules() {
    let code = compile(0, DebugMode::On(on())).expect("compiles").code;
    let at = |name: &str| code.find(&format!("rule (\"{name}\")")).expect(name);
    assert!(at("Debug: globals") < at("Debug: players"));
    assert!(at("Debug: players") < at("Run"));
}

#[test]
fn off_adds_no_rules_and_no_elements() {
    let off = compile(0, DebugMode::Off).expect("compiles");
    assert!(!off.code.contains("Debug:"));
    let on = compile(0, DebugMode::On(on())).expect("compiles");
    assert!(on.element_count > off.element_count);
}

/// Elements of the mod with `filler` rules, and with each panel plan, found by measuring.
struct Costs {
    per_filler: usize,
    base: usize,
    both: usize,
    globals: usize,
}

fn costs() -> Costs {
    let count = |filler, debug| compile(filler, debug).expect("fits").element_count;
    let base = count(0, DebugMode::Off);
    let per_filler = count(10, DebugMode::Off) - base;
    assert!(per_filler % 10 == 0 && per_filler > 0);
    let globals_only = on_with(|o| o.player_hud = false);
    Costs {
        per_filler: per_filler / 10,
        base,
        both: count(0, DebugMode::On(on())) - base,
        globals: count(0, DebugMode::On(globals_only)) - base,
    }
}

/// The most filler that still fits, leaving less than one filler rule of room.
fn filler_to_the_limit(costs: &Costs) -> usize {
    (32_768 - costs.base) / costs.per_filler
}

/// The filler that leaves at most `room` elements (and more than `room` minus one filler rule's
/// worth) under the limit when no panels are added.
fn filler_for_room(costs: &Costs, room: usize) -> usize {
    (32_768 - room - costs.base).div_ceil(costs.per_filler)
}

fn budget_warning(out: &CompileOutput) -> Option<String> {
    out.warnings
        .iter()
        .find(|d| d.kind == Kind::Warning(Warning::DebugBudget))
        .map(|d| d.message.clone())
}

#[test]
fn a_mod_with_room_keeps_every_panel() {
    let costs = costs();
    let filler = filler_for_room(&costs, costs.both + 10);
    let out = compile(filler, DebugMode::On(on())).expect("compiles");
    assert_eq!(budget_warning(&out), None);
    assert!(out.code.contains("Debug: players"));
}

#[test]
fn the_player_panel_goes_first() {
    let costs = costs();
    assert!(costs.globals < costs.both);
    let filler = filler_for_room(&costs, costs.both - 1);
    // The globals still fit, or this would test something else.
    assert!(costs.both > costs.globals + costs.per_filler);
    let out = compile(filler, DebugMode::On(on())).expect("compiles");
    assert!(out.element_count <= 32_768);
    assert!(out.code.contains("Debug: globals"));
    assert!(!out.code.contains("Debug: players"));
    let message = budget_warning(&out).expect("a warning");
    assert!(
        message.contains("the player panel (kills, alive)"),
        "{message}"
    );
    assert!(!message.contains("global fields"), "{message}");
}

#[test]
fn then_global_fields_from_the_last_declared() {
    let costs = costs();
    // Room for the heading and the first field of the global panel, nothing more.
    let filler = filler_for_room(&costs, costs.globals / 3);
    let out = compile(filler, DebugMode::On(on())).expect("compiles");
    assert!(out.element_count <= 32_768);
    let message = budget_warning(&out).expect("a warning");
    assert!(message.contains("the player panel"), "{message}");
    assert!(message.contains("global fields"), "{message}");
    assert!(message.contains("spawns"), "{message}");
    assert!(!out.code.contains("Debug: players"));
    // Fields go from the last declared: `spawns` and `Phase` before `wave`.
    assert!(!message.contains("wave"), "{message}");
    assert!(out.code.contains("wave"));
}

#[test]
fn a_mod_that_barely_fits_gets_no_panels() {
    let costs = costs();
    let filler = filler_to_the_limit(&costs);
    let release = compile(filler, DebugMode::Off).expect("fits");
    let debug = compile(filler, DebugMode::On(on())).expect("compiles");
    assert!(!debug.code.contains("Debug:"));
    assert!(budget_warning(&debug).is_some());
    // What is left is the release mod, plus the header comment.
    assert_eq!(
        debug.code.strip_prefix("// ow-workshop debug build\n"),
        Some(release.code.as_str())
    );
}

#[test]
fn a_mod_that_does_not_fit_without_panels_fails_as_in_release() {
    let costs = costs();
    let filler = filler_to_the_limit(&costs) + 1;
    for debug in [DebugMode::Off, DebugMode::On(on())] {
        let err = compile(filler, debug).expect_err("too many elements");
        assert!(
            err.0
                .iter()
                .any(|d| d.kind == Kind::Error(ErrorKind::TooManyElements)),
            "{err}"
        );
    }
}

#[test]
fn the_warning_can_be_suppressed() {
    struct Quiet(usize);

    impl Workshop for Quiet {
        const NAME: &'static str = "Test";
        const DESCRIPTION: &'static str = "Test mode";

        fn settings(&self) -> ow_workshop::Settings {
            ow_workshop::Settings::unchecked()
        }

        type Globals = Globals;
        type Players = Players;

        fn rules<'a>(&'a self, g: &'a Globals, p: &'a Players) -> Vec<RuleDef<'a>> {
            mode_rules(self.0, g, p)
        }

        fn options(&self) -> CompileOptions {
            let mut options = CompileOptions::default();
            options.suppress.insert(Warning::DebugBudget);
            options
        }
    }

    let costs = costs();
    let filler = filler_for_room(&costs, costs.both - 1);
    let out = Quiet(filler)
        .compile_with_debug(Lang::EnUs, Some(DebugMode::On(on())))
        .expect("compiles");
    assert_eq!(budget_warning(&out), None);
    assert!(!out.code.contains("Debug: players"));
}
