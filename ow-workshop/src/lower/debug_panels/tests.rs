use super::*;
use crate::prelude::*;
use crate::vars::VarRegistry;
use crate::workshop_debug::WorkshopDebug as _;

#[derive(GlobalVars)]
struct Globals {
    a: Global<Num>,
    b: Global<Num>,
    c: Global<Num>,
}

#[derive(PlayerVars)]
struct Players {
    p: PlayerVar<Num>,
}

fn program() -> Program {
    let globals = Globals::declare(&mut VarRegistry::default());
    let players = Players::declare(&mut VarRegistry::default());
    let mut debug_fields = globals.debug_fields();
    debug_fields.extend(players.debug_fields());
    Program {
        debug_fields,
        ..Program::default()
    }
}

fn names(rules: &[crate::ir::Rule]) -> Vec<&str> {
    rules.iter().map(|r| r.name.as_str()).collect()
}

/// The rule's `Create HUD Text` statements.
fn huds(rule: &crate::ir::Rule) -> usize {
    rule.body.stmts.len()
}

#[test]
fn sizes_count_the_fields_the_options_show() {
    let program = program();
    let mut options = DebugOptions::default();
    assert_eq!(sizes(&program, options), (3, 1));
    options.hud = false;
    assert_eq!(sizes(&program, options), (0, 1));
    options.player_hud = false;
    assert_eq!(sizes(&program, options), (0, 0));
}

#[test]
fn every_panel_has_a_heading_and_a_text_per_field() {
    let program = program();
    let rules = rules(&program, DebugOptions::default(), PanelPlan::default());
    assert_eq!(names(&rules), ["Debug: globals", "Debug: players"]);
    assert_eq!(huds(&rules[0]), 1 + 3);
    assert_eq!(huds(&rules[1]), 1 + 1);
    assert!(rules.iter().all(|r| r.location.is_none()));
}

#[test]
fn a_plan_keeps_the_first_global_fields() {
    let program = program();
    let plan = PanelPlan {
        players: false,
        globals: 2,
    };
    let rules = rules(&program, DebugOptions::default(), plan);
    assert_eq!(names(&rules), ["Debug: globals"]);
    assert_eq!(huds(&rules[0]), 1 + 2);
}

#[test]
fn no_plan_and_no_fields_mean_no_rules() {
    let none = Vec::<&str>::new();
    let program = program();
    let nothing = rules(&program, DebugOptions::default(), PanelPlan::NONE);
    assert_eq!(names(&nothing), none);
    let no_fields = rules(
        &Program::default(),
        DebugOptions::default(),
        PanelPlan::default(),
    );
    assert_eq!(names(&no_fields), none);
}
