use ow_workshop_data::{ActionId, ValueId as V};

use super::declare_variables;
use crate::ir::CmpOp::Eq;
use crate::ir::{Expr, Place, Program, StmtKind, VarId};
use crate::lower::testutil::{act, konst, num, program, run};
use crate::opt::testutil::*;
use crate::options::{CompileOptions, EmptyString, OptLevel, Replacements};

fn options(level: OptLevel, replacements: Replacements) -> CompileOptions {
    CompileOptions {
        optimize: level,
        replacements,
        ..CompileOptions::default()
    }
}

fn size_with(replacements: Replacements) -> CompileOptions {
    options(OptLevel::Size, replacements)
}

/// The value assigned by a program with one `Set`, after the optimizer ran.
fn assigned(value: Expr, options: &CompileOptions) -> String {
    let mut p = program(vec![
        StmtKind::Set {
            target: Place::Global(VarId(0)),
            value,
        },
        act(ActionId::Kill, vec![]),
    ]);
    run(crate::opt::run, &mut p, options).expect("optimizes");
    match &p.rules[0].body.stmts[0].kind {
        StmtKind::Set { value, .. } => sh(value),
        other => panic!("not a set: {other:?}"),
    }
}

/// An argument no signature flag covers: the end position of `Distance`.
fn distance_to(arg: Expr) -> Expr {
    val(V::Distance, vec![g(1), arg])
}

#[test]
fn nothing_is_replaced_by_default() {
    let o = size_with(Replacements::default());
    assert_eq!(assigned(distance_to(num(0.0)), &o), "Distance(g1, 0)");
    assert_eq!(assigned(distance_to(num(1.0)), &o), "Distance(g1, 1)");
    assert_eq!(
        assigned(val(V::GetPlayers, vec![konst("Team", "1")]), &o),
        "GetPlayers(Team.1)"
    );
}

#[test]
fn team_one_by_control_scoring_team() {
    let o = size_with(Replacements {
        team1_by_control_scoring_team: true,
        ..Replacements::default()
    });
    let players = |team| val(V::GetPlayers, vec![konst("Team", team)]);
    assert_eq!(
        assigned(players("1"), &o),
        "GetPlayers(GetControlScoringTeam())"
    );
    assert_eq!(assigned(players("2"), &o), "GetPlayers(Team.2)");
    assert_eq!(assigned(players("ALL"), &o), "GetPlayers(Team.ALL)");
}

#[test]
fn zero_by_capture_percentage() {
    let o = size_with(Replacements {
        zero_by_capture_percentage: true,
        ..Replacements::default()
    });
    assert_eq!(
        assigned(distance_to(num(0.0)), &o),
        "Distance(g1, GetCapturePercentage())"
    );
    assert_eq!(assigned(distance_to(num(2.0)), &o), "Distance(g1, 2)");
}

#[test]
fn zero_by_payload_progress() {
    let o = size_with(Replacements {
        zero_by_payload_progress: true,
        ..Replacements::default()
    });
    assert_eq!(
        assigned(distance_to(num(0.0)), &o),
        "Distance(g1, GetPayloadProgressPercentage())"
    );
}

#[test]
fn zero_by_is_match_complete() {
    let o = size_with(Replacements {
        zero_by_is_match_complete: true,
        ..Replacements::default()
    });
    assert_eq!(
        assigned(distance_to(num(0.0)), &o),
        "Distance(g1, IsMatchComplete())"
    );
}

#[test]
fn the_first_configured_zero_wins() {
    let o = size_with(Replacements {
        zero_by_payload_progress: true,
        zero_by_is_match_complete: true,
        ..Replacements::default()
    });
    assert_eq!(
        assigned(distance_to(num(0.0)), &o),
        "Distance(g1, GetPayloadProgressPercentage())"
    );
}

#[test]
fn one_by_match_round() {
    let o = size_with(Replacements {
        one_by_match_round: true,
        ..Replacements::default()
    });
    assert_eq!(
        assigned(distance_to(num(1.0)), &o),
        "Distance(g1, GetMatchRound())"
    );
    assert_eq!(assigned(distance_to(num(2.0)), &o), "Distance(g1, 2)");
}

#[test]
fn signature_flags_win_over_the_options() {
    let o = size_with(Replacements {
        zero_by_capture_percentage: true,
        one_by_match_round: true,
        ..Replacements::default()
    });
    // `Slice` takes `False` and `True` for 0 and 1.
    assert_eq!(
        assigned(val(V::Slice, vec![g(1), num(0.0), num(1.0)]), &o),
        "Slice(g1, false, true)"
    );
    // Arrays take `Null` for 0.
    assert_eq!(
        assigned(arr(vec![num(0.0), num(1.0)]), &o),
        "[Null, GetMatchRound()]"
    );
    // So does an assignment.
    assert_eq!(assigned(num(0.0), &o), "Null");
}

#[test]
fn conditions_compare_with_null_or_the_replacement() {
    let o = size_with(Replacements {
        zero_by_capture_percentage: true,
        one_by_match_round: true,
        team1_by_control_scoring_team: true,
        ..Replacements::default()
    });
    assert_eq!(assigned(cmp(Eq, g(1), num(0.0)), &o), "!g1");
    assert_eq!(
        assigned(cmp(crate::ir::CmpOp::Gt, g(1), num(0.0)), &o),
        "(g1 > Null)"
    );
    assert_eq!(
        assigned(cmp(crate::ir::CmpOp::Gt, g(1), num(1.0)), &o),
        "(g1 > GetMatchRound())"
    );
    assert_eq!(
        assigned(cmp(Eq, g(1), konst("Team", "1")), &o),
        "(g1 == GetControlScoringTeam())"
    );
}

#[test]
fn rule_conditions_are_replaced_too() {
    let o = size_with(Replacements {
        one_by_match_round: true,
        ..Replacements::default()
    });
    let mut p = program(vec![act(ActionId::Kill, vec![])]);
    p.rules[0].conditions = vec![crate::ir::Cond {
        expr: cmp(crate::ir::CmpOp::Gt, g(1), num(1.0)),
        comment: None,
    }];
    run(crate::opt::run, &mut p, &o).expect("optimizes");
    assert_eq!(sh(&p.rules[0].conditions[0].expr), "(g1 > GetMatchRound())");
}

#[test]
fn nothing_is_replaced_outside_size_mode() {
    let replacements = Replacements {
        zero_by_capture_percentage: true,
        one_by_match_round: true,
        team1_by_control_scoring_team: true,
        empty_string: Some(EmptyString::EmptyArray),
        ..Replacements::default()
    };
    for level in [OptLevel::Speed, OptLevel::Off] {
        let o = options(level, replacements.clone());
        assert_eq!(assigned(distance_to(num(0.0)), &o), "Distance(g1, 0)");
        assert_eq!(assigned(distance_to(num(1.0)), &o), "Distance(g1, 1)");
    }
}

#[test]
fn workshop_settings_keep_their_literals() {
    let o = size_with(Replacements {
        zero_by_capture_percentage: true,
        one_by_match_round: true,
        ..Replacements::default()
    });
    let setting = val(
        V::CreateWorkshopSettingInt,
        vec![cstr("c"), cstr("n"), num(1.0), num(0.0), num(1.0), num(0.0)],
    );
    let before = assigned(
        setting.clone(),
        &options(OptLevel::Speed, Replacements::default()),
    );
    assert_eq!(assigned(setting, &o), before);
}

// ---- the empty string ---------------------------------------------------------------------

#[test]
fn empty_string_by_empty_array() {
    let o = size_with(Replacements {
        empty_string: Some(EmptyString::EmptyArray),
        ..Replacements::default()
    });
    assert_eq!(assigned(cstr(""), &o), "[]");
    assert_eq!(assigned(distance_to(cstr("")), &o), "Distance(g1, [])");
}

#[test]
fn empty_string_defaults_to_the_first_character_of_nothing() {
    let o = size_with(Replacements::default());
    assert_eq!(assigned(cstr(""), &o), "CharAt([], false)");
}

fn compiled(value: Expr, options: &CompileOptions) -> Program {
    let mut p = program(vec![
        StmtKind::Set {
            target: Place::Global(VarId(0)),
            value,
        },
        act(ActionId::Kill, vec![]),
    ]);
    let passes: &[(&str, crate::lower::Pass)] = &[
        ("declare", declare_variables),
        ("optimize", crate::opt::run),
    ];
    let mut diags = crate::ir::Diagnostics::default();
    let mut ctx = crate::lower::Ctx::new(options, ow_workshop_data::Lang::EnUs, &mut diags, &p);
    crate::lower::run_passes(passes, &mut p, &mut ctx).expect("compiles");
    p
}

#[test]
fn empty_string_by_variable_declares_the_global() {
    let o = size_with(Replacements {
        empty_string: Some(EmptyString::Variable),
        ..Replacements::default()
    });
    let p = compiled(cstr(""), &o);
    let decl = p
        .globals
        .entries
        .iter()
        .find(|d| d.name == "__emptyString__")
        .expect("declared");
    assert!(decl.internal);
    assert_eq!(
        decl.init.as_ref().map(sh).as_deref(),
        Some("CharAt([], Null)")
    );
    let StmtKind::Set { value, .. } = &p.rules[0].body.stmts[0].kind else {
        panic!("not a set")
    };
    let index = p.globals.entries.len() - 1;
    assert_eq!(sh(value), format!("g{index}"));
}

#[test]
fn the_variable_is_only_declared_when_needed() {
    let o = size_with(Replacements {
        empty_string: Some(EmptyString::Variable),
        ..Replacements::default()
    });
    assert_eq!(compiled(num(3.0), &o).globals.entries.len(), 0);
    let off = options(
        OptLevel::Off,
        Replacements {
            empty_string: Some(EmptyString::Variable),
            ..Replacements::default()
        },
    );
    assert_eq!(compiled(cstr(""), &off).globals.entries.len(), 0);
    let unset = size_with(Replacements::default());
    assert_eq!(compiled(cstr(""), &unset).globals.entries.len(), 0);
}

#[test]
fn the_variable_is_declared_once() {
    let o = size_with(Replacements {
        empty_string: Some(EmptyString::Variable),
        ..Replacements::default()
    });
    let mut p = compiled(cstr(""), &o);
    let mut diags = crate::ir::Diagnostics::default();
    let mut ctx = crate::lower::Ctx::new(&o, ow_workshop_data::Lang::EnUs, &mut diags, &p);
    declare_variables(&mut p, &mut ctx).expect("declares");
    assert_eq!(p.globals.entries.len(), 1);
}

#[test]
fn signature_flags_win_for_empty_strings_too() {
    let o = size_with(Replacements {
        empty_string: Some(EmptyString::Variable),
        ..Replacements::default()
    });
    // `String Contains` takes an empty array.
    let p = compiled(val(V::StrContains, vec![g(1), cstr("")]), &o);
    let StmtKind::Set { value, .. } = &p.rules[0].body.stmts[0].kind else {
        panic!("not a set")
    };
    assert_eq!(sh(value), "StrContains(g1, [])");
}
