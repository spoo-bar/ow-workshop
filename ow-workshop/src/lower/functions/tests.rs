use ow_workshop_data::{ActionId, ValueId};

use super::*;
use crate::ir::{Cond, ModifyOp, Place, VarId};
use crate::lower::testutil::*;
use crate::options::OptLevel;

fn cond(e: Expr) -> Cond {
    Cond {
        expr: e,
        comment: None,
    }
}

fn var(id: u32) -> Expr {
    expr(ExprKind::GlobalVar(VarId(id)))
}

fn lowered_with(mut p: Program, opt: OptLevel) -> Program {
    run(lower, &mut p, &options(opt)).expect("lowers");
    p
}

fn lowered(body: Vec<StmtKind>) -> Program {
    lowered_with(program(body), OptLevel::Size)
}

fn set_value(p: &Program) -> String {
    let StmtKind::Set { value, .. } = &p.rules[0].body.stmts[0].kind else {
        panic!("not a set")
    };
    show(value)
}

fn assign(value: Expr) -> StmtKind {
    StmtKind::Set {
        target: Place::Global(VarId(0)),
        value,
    }
}

fn rule_condition() -> Expr {
    expr(ExprKind::RuleCondition)
}

#[test]
fn rule_condition_expands_to_the_rules_conditions() {
    let mut p = program(vec![assign(rule_condition())]);
    p.rules[0].conditions = vec![cond(var(1)), cond(var(2)), cond(var(3))];
    assert_eq!(
        set_value(&lowered_with(p, OptLevel::Size)),
        "(g1 && (g2 && g3))"
    );

    let mut p = program(vec![assign(rule_condition())]);
    p.rules[0].conditions = vec![cond(var(1))];
    assert_eq!(set_value(&lowered_with(p, OptLevel::Size)), "g1");

    assert_eq!(set_value(&lowered(vec![assign(rule_condition())])), "true");
}

#[test]
fn conditional_loop_and_abort_on_the_rule_condition() {
    for (cond, thing, want) in [
        (
            rule_condition(),
            StmtKind::Loop,
            StmtKind::LoopIfConditionTrue,
        ),
        (
            expr(ExprKind::Not(rule_condition())),
            StmtKind::Loop,
            StmtKind::LoopIfConditionFalse,
        ),
        (
            rule_condition(),
            StmtKind::Abort,
            StmtKind::AbortIfConditionTrue,
        ),
        (
            expr(ExprKind::Not(rule_condition())),
            StmtKind::Abort,
            StmtKind::AbortIfConditionFalse,
        ),
    ] {
        // Even with the optimizer off.
        let p = lowered_with(program(vec![if_(cond, vec![thing])]), OptLevel::Off);
        assert_eq!(kinds(&p), [&want]);
    }
}

#[test]
fn lone_conditional_actions_fuse_when_optimizing() {
    let c = var(1);
    let p = lowered(vec![
        if_(c.clone(), vec![StmtKind::Abort]),
        if_(c.clone(), vec![StmtKind::Loop]),
        if_(c.clone(), vec![skip_to(0)]),
        place(0),
    ]);
    assert_eq!(
        kinds(&p),
        [
            &StmtKind::AbortIf(c.clone()),
            &StmtKind::LoopIf(c.clone()),
            &StmtKind::SkipIf(c, SkipTarget::Label(crate::ir::LabelId(0))),
            &place(0)
        ]
    );
}

#[test]
fn lone_conditional_actions_need_the_optimizer_and_a_static_rule() {
    let body = vec![if_(var(1), vec![StmtKind::Abort])];
    let off = lowered_with(program(body.clone()), OptLevel::Off);
    assert!(matches!(kinds(&off)[0], StmtKind::If { .. }));

    // A dynamic skip anywhere in the rule keeps the line counts as written.
    let dynamic = StmtKind::Skip(SkipTarget::Dynamic(var(2)));
    let p = lowered(vec![dynamic, body[0].clone()]);
    assert!(matches!(kinds(&p)[1], StmtKind::If { .. }));
}

#[test]
fn only_a_single_child_without_elif_or_else_fuses() {
    let two = lowered(vec![if_(var(1), vec![StmtKind::Abort, StmtKind::Abort])]);
    assert!(matches!(kinds(&two)[0], StmtKind::If { .. }));
    let with_else = StmtKind::If {
        cond: var(1),
        then: block(vec![StmtKind::Abort]),
        elifs: vec![],
        else_: Some(block(vec![StmtKind::Loop])),
    };
    let p = lowered(vec![with_else]);
    assert!(matches!(kinds(&p)[0], StmtKind::If { .. }));
}

fn current_map() -> Expr {
    call(ValueId::GetCurrentMap, vec![])
}

fn eq(a: Expr, b: Expr) -> Expr {
    expr(ExprKind::Compare {
        op: CmpOp::Eq,
        lhs: a,
        rhs: b,
    })
}

#[test]
fn comparing_the_current_map_with_a_duplicated_map() {
    for key in ["colosseo", "esperanca", "samoa"] {
        let p = lowered(vec![assign(eq(current_map(), konst("Map", key)))]);
        assert_eq!(
            set_value(&p),
            format!("(\"{{0}}\"(GetCurrentMap()) Eq \"{{0}}\"(Map.{key}))")
        );
    }
}

#[test]
fn other_uses_of_the_current_map_when_bugged_maps_are_referenced() {
    // Throne of Anubis cannot be compared directly, and it counts as referenced.
    let p = lowered(vec![
        assign(eq(current_map(), konst("Map", "throneOfAnubis"))),
        assign(current_map()),
    ]);
    let want = "FirstOf(FilteredArray(Concat([Map.throneOfAnubis], GetCurrentMap()), (\"{0}\"(GetCurrentMap()) Eq Split(@, EmptyArray()))))";
    // The comparison itself is "any other use" too, so both sides are rewritten.
    assert_eq!(set_value(&p), format!("({want} Eq Map.throneOfAnubis)"));
    let StmtKind::Set { value, .. } = &p.rules[0].body.stmts[1].kind else {
        panic!("not a set")
    };
    assert_eq!(show(value), want);
}

#[test]
fn the_current_map_stays_plain_without_bugged_maps() {
    let p = lowered(vec![assign(eq(current_map(), konst("Map", "busan")))]);
    assert_eq!(set_value(&p), "(GetCurrentMap() Eq Map.busan)");
}

fn hud(given: Vec<(&str, Expr)>) -> StmtKind {
    act(ActionId::HudText, given)
}

fn args_of(p: &Program) -> Vec<String> {
    let StmtKind::Action { args, .. } = &p.rules[0].body.stmts[0].kind else {
        panic!("not an action")
    };
    args.iter().map(show).collect()
}

fn left() -> Expr {
    konst("HudPosition", "ACTUALLY_LEFT")
}

#[test]
fn hud_alignment_pads_the_subheader_first() {
    let pad = "\"<170em>\"()";
    // (given arguments, subheader, text)
    let cases = [
        (
            vec![("subheader", num(1.0)), ("text", num(2.0))],
            format!("\"{{0}}{{1}}\"(1, {pad})"),
            "2".to_owned(),
        ),
        (
            vec![("text", num(2.0))],
            "Null".to_owned(),
            format!("\"{{0}}{{1}}\"(2, {pad})"),
        ),
        (vec![], "Null".to_owned(), pad.to_owned()),
    ];
    for (mut given, subheader, text) in cases {
        given.push(("location", left()));
        let args = args_of(&lowered(vec![hud(given)]));
        assert_eq!(args[2], subheader);
        assert_eq!(args[3], text);
        assert_eq!(args[4], "HudPosition.LEFT");
    }
}

#[test]
fn other_hud_positions_are_untouched() {
    let kind = hud(vec![
        ("text", num(2.0)),
        ("location", konst("HudPosition", "TOP")),
    ]);
    let p = lowered(vec![kind.clone()]);
    assert_eq!(kinds(&p), [&kind]);
}

#[test]
fn progress_bar_text_is_padded_too() {
    let p = lowered(vec![act(
        ActionId::ProgressBarHud,
        vec![("text", num(2.0)), ("location", left())],
    )]);
    let args = args_of(&p);
    let sig = ActionId::ProgressBarHud.signature();
    let text = arg_index(sig, "text").expect("text");
    let location = arg_index(sig, "location").expect("location");
    assert_eq!(args[text], "\"{0}{1}\"(2, \"<170em>\"())");
    assert_eq!(args[location], "HudPosition.LEFT");
}

#[test]
fn fractional_ultimate_charge_literals_get_an_absolute_value() {
    let frac = lowered(vec![act(
        ActionId::SetUltCharge,
        vec![("chargePercent", num(12.5))],
    )]);
    assert_eq!(args_of(&frac)[1], "Abs(12.5)");
    let whole = lowered(vec![act(
        ActionId::SetUltCharge,
        vec![("chargePercent", num(12.0))],
    )]);
    assert_eq!(args_of(&whole)[1], "12");
}

#[test]
fn n_ary_min_and_max_become_nested_binary_calls() {
    let args = |n: u32| (1..=n).map(|i| num(f64::from(i))).collect::<Vec<_>>();
    let p = lowered(vec![
        assign(call(ValueId::Min, args(3))),
        assign(call(ValueId::Max, args(4))),
        assign(call(ValueId::Min, args(2))),
    ]);
    let shown: Vec<String> = p.rules[0]
        .body
        .stmts
        .iter()
        .map(|s| match &s.kind {
            StmtKind::Set { value, .. } => show(value),
            _ => unreachable!(),
        })
        .collect();
    assert_eq!(shown[0], "Min(Min(1, 2), 3)");
    assert_eq!(shown[1], "Max(Max(Max(1, 2), 3), 4)");
    assert_eq!(shown[2], "Min(1, 2)");
}

#[test]
fn nested_blocks_and_conditions_are_rewritten() {
    let mut p = program(vec![if_(
        boolean(true),
        vec![StmtKind::Modify {
            target: Place::Global(VarId(0)),
            op: ModifyOp::Add,
            value: call(ValueId::Max, vec![num(1.0), num(2.0), num(3.0)]),
        }],
    )]);
    p.rules[0].conditions = vec![cond(call(ValueId::Min, vec![num(1.0), num(2.0), num(3.0)]))];
    let p = lowered_with(p, OptLevel::Size);
    assert_eq!(show(&p.rules[0].conditions[0].expr), "Min(Min(1, 2), 3)");
    let StmtKind::If { then, .. } = &p.rules[0].body.stmts[0].kind else {
        panic!("not an if")
    };
    let StmtKind::Modify { value, .. } = &then.stmts[0].kind else {
        panic!("not a modify")
    };
    assert_eq!(show(value), "Max(Max(1, 2), 3)");
}
