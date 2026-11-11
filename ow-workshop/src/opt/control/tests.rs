use ow_workshop_data::ActionId;

use crate::ir::{Expr, ExprKind, ModifyOp, Place, Program, SkipTarget, StmtKind, VarId};
use crate::lower::testutil::*;
use crate::opt::testutil::{add, cstr, g, mul, sh, sub, val};
use crate::options::OptLevel;

fn optimized_with(p: Program, level: OptLevel) -> Program {
    let mut p = p;
    run(crate::opt::run, &mut p, &options(level)).expect("optimizes");
    p
}

fn optimized(body: Vec<StmtKind>) -> Program {
    optimized_with(program(body), OptLevel::Speed)
}

fn var(n: u32) -> Place {
    Place::Global(VarId(n))
}

fn set(n: u32, value: Expr) -> StmtKind {
    StmtKind::Set {
        target: var(n),
        value,
    }
}

fn shape(p: &Program) -> Vec<String> {
    p.rules
        .first()
        .map(|r| {
            r.body
                .stmts
                .iter()
                .map(|s| match &s.kind {
                    StmtKind::Set { value, .. } => format!("set {}", sh(value)),
                    StmtKind::Modify { op, value, .. } => format!("{op:?} {}", sh(value)),
                    StmtKind::If { cond, .. } => format!("if {}", sh(cond)),
                    StmtKind::Label(l) => format!("label{}", l.0),
                    StmtKind::Skip(_) => "skip".to_owned(),
                    StmtKind::Abort => "abort".to_owned(),
                    StmtKind::Action { id, .. } => format!("{id:?}"),
                    other => format!("{other:?}"),
                })
                .collect()
        })
        .unwrap_or_default()
}

/// An action that is not a pause.
fn real() -> StmtKind {
    act(ActionId::Kill, vec![])
}

fn unknown() -> Expr {
    g(5)
}

// ---- assignments ---------------------------------------------------------------------------

#[test]
fn self_updates_become_modify() {
    let cases = [
        (add(g(0), unknown()), ModifyOp::Add),
        (sub(g(0), unknown()), ModifyOp::Subtract),
        (mul(g(0), unknown()), ModifyOp::Multiply),
        (expr(ExprKind::Div(g(0), unknown())), ModifyOp::Divide),
        (expr(ExprKind::Mod(g(0), unknown())), ModifyOp::Modulo),
        (expr(ExprKind::Pow(g(0), unknown())), ModifyOp::RaiseToPower),
        (
            val(ow_workshop_data::ValueId::Min, vec![g(0), unknown()]),
            ModifyOp::Min,
        ),
        (
            val(ow_workshop_data::ValueId::Max, vec![g(0), unknown()]),
            ModifyOp::Max,
        ),
        (
            val(ow_workshop_data::ValueId::Concat, vec![g(0), unknown()]),
            ModifyOp::AppendToArray,
        ),
        (
            val(ow_workshop_data::ValueId::Exclude, vec![g(0), unknown()]),
            ModifyOp::RemoveFromArrayByValue,
        ),
    ];
    for (value, expected) in cases {
        let p = optimized(vec![set(0, value)]);
        assert_eq!(shape(&p), [format!("{expected:?} g5")]);
    }
}

#[test]
fn only_the_left_operand_counts() {
    let p = optimized(vec![set(0, sub(unknown(), g(0)))]);
    assert_eq!(shape(&p), ["set (g5 - g0)"]);
    let other = optimized(vec![set(1, add(g(0), unknown()))]);
    assert_eq!(shape(&other), ["set (g0 + g5)"]);
}

#[test]
fn indexed_places_update_in_place() {
    let target = Place::Index {
        base: Box::new(var(0)),
        index: g(1),
    };
    let current = expr(ExprKind::Index {
        array: g(0),
        index: g(1),
    });
    let p = optimized(vec![StmtKind::Set {
        target: target.clone(),
        value: add(current, unknown()),
    }]);
    let StmtKind::Modify { target: got, .. } = &p.rules[0].body.stmts[0].kind else {
        panic!("not modified")
    };
    assert_eq!(got, &target);
}

#[test]
fn assigning_a_variable_to_itself_is_removed() {
    let p = optimized(vec![set(0, g(0)), real()]);
    assert_eq!(shape(&p), ["Kill"]);
}

#[test]
fn random_values_are_not_the_same_variable() {
    let roll = val(
        ow_workshop_data::ValueId::RandomRandint,
        vec![num(1.0), num(2.0)],
    );
    let target = Place::Index {
        base: Box::new(var(0)),
        index: roll.clone(),
    };
    let current = expr(ExprKind::Index {
        array: g(0),
        index: roll,
    });
    let p = optimized(vec![StmtKind::Set {
        target,
        value: current,
    }]);
    assert!(matches!(
        p.rules[0].body.stmts[0].kind,
        StmtKind::Set { .. }
    ));
}

// ---- dead code -----------------------------------------------------------------------------

fn if_else(
    cond: Expr,
    then: Vec<StmtKind>,
    elifs: Vec<(Expr, Vec<StmtKind>)>,
    else_: Option<Vec<StmtKind>>,
) -> StmtKind {
    StmtKind::If {
        cond,
        then: block(then),
        elifs: elifs.into_iter().map(|(c, b)| (c, block(b))).collect(),
        else_: else_.map(block),
    }
}

fn marker(n: u32) -> StmtKind {
    set(n, num(1.0))
}

#[test]
fn a_false_if_is_removed() {
    let p = optimized(vec![if_(boolean(false), vec![marker(0)]), marker(1)]);
    assert_eq!(shape(&p), ["set 1"]);
}

#[test]
fn a_false_if_keeps_a_body_with_a_targeted_label() {
    let body = vec![
        skip_to(1),
        if_(boolean(false), vec![place(1), marker(0)]),
        marker(1),
    ];
    let p = optimized(body);
    assert_eq!(shape(&p), ["skip", "if false", "set 1"]);
}

#[test]
fn a_false_if_with_else_runs_the_else() {
    let p = optimized(vec![if_else(
        boolean(false),
        vec![marker(0)],
        vec![],
        Some(vec![marker(1), marker(2)]),
    )]);
    assert_eq!(shape(&p), ["set 1", "set 1"]);
}

#[test]
fn a_false_if_promotes_the_first_else_if() {
    let p = optimized(vec![if_else(
        boolean(false),
        vec![marker(0)],
        vec![(g(7), vec![marker(1)]), (g(8), vec![marker(2)])],
        Some(vec![marker(3)]),
    )]);
    let StmtKind::If {
        cond, elifs, else_, ..
    } = &p.rules[0].body.stmts[0].kind
    else {
        panic!("not an if")
    };
    assert_eq!(sh(cond), "g7");
    assert_eq!(elifs.len(), 1);
    assert!(else_.is_some());
}

#[test]
fn a_false_else_if_leaves_the_chain() {
    let p = optimized(vec![if_else(
        g(7),
        vec![marker(0)],
        vec![(boolean(false), vec![marker(1)]), (g(8), vec![marker(2)])],
        None,
    )]);
    let StmtKind::If { elifs, .. } = &p.rules[0].body.stmts[0].kind else {
        panic!("not an if")
    };
    assert_eq!(elifs.len(), 1);
    assert_eq!(sh(&elifs[0].0), "g8");
}

#[test]
fn adding_or_subtracting_zero_is_dropped() {
    let modify = |op, value| StmtKind::Modify {
        target: var(0),
        op,
        value,
    };
    let p = optimized(vec![
        modify(ModifyOp::Add, num(0.0)),
        modify(ModifyOp::Subtract, num(0.0)),
        modify(ModifyOp::Add, num(1.0)),
        modify(ModifyOp::Multiply, num(0.0)),
    ]);
    assert_eq!(shape(&p), ["Add 1", "Multiply 0"]);
}

#[test]
fn a_chain_of_dead_branches_folds_completely() {
    let p = optimized(vec![if_else(
        boolean(false),
        vec![marker(0)],
        vec![
            (boolean(false), vec![marker(1)]),
            (boolean(true), vec![marker(2)]),
        ],
        None,
    )]);
    assert_eq!(shape(&p), ["set 1"]);
}

#[test]
fn a_true_if_keeps_its_body() {
    let p = optimized(vec![if_else(
        boolean(true),
        vec![marker(0), marker(1)],
        vec![(g(7), vec![marker(2)])],
        Some(vec![marker(3)]),
    )]);
    assert_eq!(shape(&p), ["set 1", "set 1"]);
}

#[test]
fn a_true_if_stays_when_the_rule_has_skips() {
    let p = optimized(vec![
        skip_to(1),
        if_(boolean(true), vec![marker(0)]),
        place(1),
        real(),
    ]);
    assert_eq!(shape(&p), ["skip", "if true", "label1", "Kill"]);
}

#[test]
fn a_true_else_if_becomes_the_else() {
    let p = optimized(vec![if_else(
        g(7),
        vec![marker(0)],
        vec![
            (g(8), vec![marker(1)]),
            (boolean(true), vec![marker(2)]),
            (g(9), vec![marker(3)]),
        ],
        Some(vec![marker(4)]),
    )]);
    let StmtKind::If { elifs, else_, .. } = &p.rules[0].body.stmts[0].kind else {
        panic!("not an if")
    };
    assert_eq!(elifs.len(), 1);
    let else_ = else_.as_ref().expect("an else");
    assert_eq!(else_.stmts.len(), 1);
    assert!(matches!(
        &else_.stmts[0].kind,
        StmtKind::Set {
            target: Place::Global(VarId(2)),
            ..
        }
    ));
}

#[test]
fn an_empty_if_is_removed() {
    let p = optimized(vec![if_(g(7), vec![]), marker(0)]);
    assert_eq!(shape(&p), ["set 1"]);
    let with_else = optimized(vec![if_else(g(7), vec![], vec![], Some(vec![marker(0)]))]);
    assert_eq!(shape(&with_else), ["if g7"]);
}

#[test]
fn a_false_while_is_removed() {
    let body = block(vec![marker(0)]);
    let p = optimized(vec![
        StmtKind::While {
            cond: boolean(false),
            body,
        },
        marker(1),
    ]);
    assert_eq!(shape(&p), ["set 1"]);
}

#[test]
fn dead_code_after_an_exit_stops_at_a_targeted_label() {
    let p = optimized(vec![
        skip_to(1),
        StmtKind::Abort,
        marker(0),
        marker(1),
        place(1),
        marker(2),
        place(2),
        marker(3),
    ]);
    // Label 2 is not targeted but sits after the first targeted label, so it stays.
    assert_eq!(
        shape(&p),
        ["skip", "abort", "label1", "set 1", "label2", "set 1"]
    );
}

#[test]
fn dead_code_after_an_exit_goes_when_nothing_jumps_in() {
    let p = optimized(vec![real(), StmtKind::Abort, marker(0), place(9)]);
    assert_eq!(shape(&p), ["Kill", "abort"]);
}

#[test]
fn nothing_is_removed_in_a_rule_with_a_dynamic_skip() {
    let dynamic = StmtKind::Skip(SkipTarget::Dynamic(g(3)));
    let p = optimized(vec![
        dynamic,
        StmtKind::Pass,
        if_(boolean(false), vec![marker(0)]),
        set(0, g(0)),
        StmtKind::Abort,
        marker(1),
    ]);
    assert_eq!(p.rules[0].body.stmts.len(), 6);
}

#[test]
fn nothing_changes_with_the_optimizer_off() {
    let before = program(vec![
        StmtKind::Pass,
        if_(boolean(false), vec![marker(0)]),
        set(0, g(0)),
    ]);
    assert_eq!(optimized_with(before.clone(), OptLevel::Off), before);
}

// ---- workshop no-ops -----------------------------------------------------------------------

#[test]
fn pass_is_removed() {
    let p = optimized(vec![StmtKind::Pass, real(), StmtKind::Pass]);
    assert_eq!(shape(&p), ["Kill"]);
}

#[test]
fn empty_hud_text_is_removed() {
    let p = optimized(vec![
        act(
            ActionId::HudText,
            vec![("header", cstr("")), ("text", null())],
        ),
        act(ActionId::HudText, vec![("header", cstr("hi"))]),
        real(),
    ]);
    assert_eq!(shape(&p), ["HudText", "Kill"]);
}

#[test]
fn empty_strings_in_hud_text_become_null() {
    let p = optimized(vec![act(
        ActionId::HudText,
        vec![
            ("header", cstr("hi")),
            ("subheader", cstr("")),
            ("text", cstr("")),
        ],
    )]);
    let StmtKind::Action { args, .. } = &p.rules[0].body.stmts[0].kind else {
        panic!("not an action")
    };
    assert!(matches!(args[2].kind(), ExprKind::Null));
    assert!(matches!(args[3].kind(), ExprKind::Null));
}

// ---- rule removal --------------------------------------------------------------------------

fn rule_program(body: Vec<StmtKind>) -> Program {
    program(body)
}

#[test]
fn a_rule_with_nothing_meaningful_is_removed() {
    let bodies = [
        vec![StmtKind::Pass],
        vec![action_kind(ActionId::Wait)],
        vec![StmtKind::Abort, StmtKind::Loop],
        vec![skip_to(1), place(1)],
        vec![if_(g(7), vec![StmtKind::Pass])],
        vec![],
    ];
    for body in bodies {
        let p = optimized_with(rule_program(body), OptLevel::Speed);
        assert_eq!(p.rules.len(), 0);
    }
}

fn action_kind(id: ActionId) -> StmtKind {
    act(id, vec![])
}

#[test]
fn a_rule_with_a_real_action_stays() {
    for body in [
        vec![marker(0)],
        vec![if_(g(7), vec![marker(0)])],
        vec![action_kind(ActionId::Kill)],
        vec![StmtKind::CallSubroutine(crate::ir::SubId(0))],
    ] {
        let p = optimized_with(rule_program(body), OptLevel::Speed);
        assert_eq!(p.rules.len(), 1);
    }
}

#[test]
fn a_delimiter_rule_stays_even_when_empty() {
    let mut p = rule_program(vec![]);
    p.rules[0].delimiter = true;
    assert_eq!(optimized_with(p, OptLevel::Speed).rules.len(), 1);
}

#[test]
fn an_empty_rule_stays_with_the_optimizer_off() {
    let p = optimized_with(rule_program(vec![]), OptLevel::Off);
    assert_eq!(p.rules.len(), 1);
}

#[test]
fn waits_count_in_subroutines_only() {
    use crate::ir::{Event, Rule, SubId};
    let sub_rule = Rule::new("s", Event::Subroutine(SubId(0)), block(vec![real()]));
    let mut p = Program {
        rules: vec![sub_rule],
        ..Program::default()
    };
    p.rules[0].conditions.clear();
    // Called, so it stays whatever it does.
    let caller = Rule::new(
        "c",
        Event::OngoingGlobal,
        block(vec![StmtKind::CallSubroutine(SubId(0))]),
    );
    p.rules.push(caller);
    assert_eq!(optimized_with(p.clone(), OptLevel::Speed).rules.len(), 2);
    // Not called, but a subroutine that waits is kept: the pause is its effect.
    p.rules.pop();
    assert_eq!(optimized_with(p, OptLevel::Speed).rules.len(), 1);
}

#[test]
fn an_uncalled_subroutine_that_does_nothing_is_removed() {
    use crate::ir::{Event, Rule, SubId};
    let sub_rule = Rule::new(
        "s",
        Event::Subroutine(SubId(0)),
        block(vec![StmtKind::Pass]),
    );
    let p = Program {
        rules: vec![sub_rule],
        ..Program::default()
    };
    assert_eq!(optimized_with(p, OptLevel::Speed).rules.len(), 0);
}

#[test]
fn blocks_keep_their_own_settings() {
    let mut inner = block(vec![set(0, g(0))]);
    inner.opt = Some(crate::ir::OptOverride {
        enabled: false,
        size: false,
        strict: None,
    });
    let outer = StmtKind::If {
        cond: g(7),
        then: inner,
        elifs: vec![],
        else_: None,
    };
    let p = optimized(vec![outer, marker(1)]);
    let StmtKind::If { then, .. } = &p.rules[0].body.stmts[0].kind else {
        panic!("not an if")
    };
    assert_eq!(then.stmts.len(), 1, "the disabled block is untouched");
}
