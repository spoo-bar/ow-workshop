//! The on-screen debug panels of a debug build.
//!
//! The rules are recorded through the same typed API a mod uses and added after the other
//! generated rules, before the user's rules. How many of them are kept is decided by the
//! pipeline, which drops fields until the mod fits the element limit.

use crate::builder::Builder;
use crate::event::{OngoingEachPlayer, OngoingGlobal};
use crate::expr::{Expr, null};
use crate::fmt::{Piece, template};
use crate::generated::api::{HudTextArgs, PlayerExt as _, all_players};
use crate::ir::{Program, Rule};
use crate::options::DebugOptions;
use crate::ty::{Any, Array, Button, Color, HudPosition, HudReeval, Player, SpecVisibility, Str};
use crate::workshop_debug::{DebugField, Scope};

/// Sort order of the global panel's fields: `GLOBAL_BASE + index`.
const GLOBAL_BASE: i32 = -1000;
/// Sort order of the player panel's fields: `PLAYER_BASE + index`.
const PLAYER_BASE: i32 = -500;
/// Sort order of each panel's heading.
const HEADING: i32 = -2000;

/// How much of the panels to keep.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PanelPlan {
    /// Whether the player panel is generated.
    pub(crate) players: bool,
    /// How many of the global fields, from the first declared, are shown.
    pub(crate) globals: usize,
}

impl Default for PanelPlan {
    /// Everything.
    fn default() -> Self {
        Self {
            players: true,
            globals: usize::MAX,
        }
    }
}

impl PanelPlan {
    /// No panels at all: the release output.
    pub(crate) const NONE: Self = Self {
        players: false,
        globals: 0,
    };
}

/// The fields of one scope that the options show.
fn fields(program: &Program, scope: Scope) -> impl Iterator<Item = &DebugField> {
    program
        .debug_fields
        .iter()
        .filter(move |f| f.scope == scope)
}

/// How many global and player fields a debug build with `options` shows in full.
pub(crate) fn sizes(program: &Program, options: DebugOptions) -> (usize, usize) {
    let globals = if options.hud {
        fields(program, Scope::Global).count()
    } else {
        0
    };
    let players = if options.player_hud {
        fields(program, Scope::Player).count()
    } else {
        0
    };
    (globals, players)
}

/// `{0}: {1}` with the label and the rendered value.
#[track_caller]
fn line(label: &'static str, value: Expr<Str>) -> Expr<Str> {
    template(
        &[Piece::Arg(0), Piece::Text(": "), Piece::Arg(1)],
        vec![Expr::<Str>::from(label).erase(), value.erase()],
    )
}

/// Who sees the global panel: everyone, or whoever holds the toggle button.
fn global_audience(toggle: Option<Button>) -> Expr<Array<Player>> {
    match toggle {
        None => all_players(),
        Some(button) => all_players().filter(move |p, _| p.is_holding_button(button)),
    }
}

/// A HUD text on the right; the heading puts `shown` in the header slot, a field in the body.
fn hud(
    visible_to: Expr<Array<Player>>,
    shown: Expr<Any>,
    heading: bool,
    sort_order: i32,
    color: Color,
) -> HudTextArgs {
    let (header, text) = if heading {
        (shown, null())
    } else {
        (null(), shown)
    };
    let colored = |on: bool| if on { color.into() } else { null() };
    HudTextArgs {
        visible_to: visible_to.into(),
        header,
        subheader: null(),
        text,
        location: HudPosition::Right.into(),
        sort_order: sort_order.into(),
        header_color: colored(heading),
        subheader_color: null(),
        text_color: colored(!heading),
        reevaluation: HudReeval::VisibilitySortOrderAndString.into(),
        spec_visibility: SpecVisibility::Default.into(),
    }
}

/// The panel rules for `plan`, in the order they are added.
pub(crate) fn rules(program: &Program, options: DebugOptions, plan: PanelPlan) -> Vec<Rule> {
    let mut ws = Builder::new();
    let globals: Vec<&DebugField> = if options.hud {
        fields(program, Scope::Global).take(plan.globals).collect()
    } else {
        Vec::new()
    };
    if !globals.is_empty() {
        ws.rule("Debug: globals", OngoingGlobal, |r, _| {
            r.actions(|b| {
                b.hud_text_with(hud(
                    global_audience(options.toggle),
                    Expr::<Str>::from("debug build").erase(),
                    true,
                    HEADING,
                    Color::Orange,
                ));
                for (i, field) in (0_i32..).zip(&globals) {
                    let value = (field.value)(null());
                    b.hud_text_with(hud(
                        global_audience(options.toggle),
                        line(field.label, value).erase(),
                        false,
                        GLOBAL_BASE + i,
                        Color::Orange,
                    ));
                }
            });
        });
    }
    let players: Vec<&DebugField> = if options.player_hud && plan.players {
        fields(program, Scope::Player).collect()
    } else {
        Vec::new()
    };
    if !players.is_empty() {
        ws.rule("Debug: players", OngoingEachPlayer, |r, ev| {
            let me = ev.event_player();
            let audience = || match options.toggle {
                None => Expr::<Array<Player>>::from_ir(me.ir().clone()),
                Some(button) => Expr::<Array<Player>>::from([me.clone()])
                    .filter(move |p, _| p.is_holding_button(button)),
            };
            r.actions(|b| {
                b.hud_text_with(hud(
                    audience(),
                    Expr::<Str>::from("debug build").erase(),
                    true,
                    HEADING,
                    Color::SkyBlue,
                ));
                for (i, field) in (0_i32..).zip(&players) {
                    let value = (field.value)(me.clone());
                    b.hud_text_with(hud(
                        audience(),
                        line(field.label, value).erase(),
                        false,
                        PLAYER_BASE + i,
                        Color::SkyBlue,
                    ));
                }
            });
        });
    }
    let (mut built, _) = ws.finish(Program::default());
    for rule in &mut built.rules {
        rule.location = None;
    }
    built.rules
}

#[cfg(test)]
mod tests;
