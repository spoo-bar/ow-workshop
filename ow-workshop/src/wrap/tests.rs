use ow_workshop_data::{ActionId, Lang, Ty, ValueId};

use super::run;
use crate::count::count_rule;
use crate::emit::emit;
use crate::ir::{Cond, Expr, ExprKind, Program, StmtKind};
use crate::lower::testutil::{act, call, expr, if_, null, num, options, program};
use crate::options::OptLevel;

/// An action with a limited literal argument: the action, the argument's name and its maximum.
fn limited_action() -> (ActionId, &'static str, f64) {
    ActionId::ALL
        .iter()
        .filter(|id| id.signature().flags.has_literal_limit())
        .find_map(|&id| {
            id.signature()
                .args
                .iter()
                .find_map(|a| a.literal_max.filter(|&m| m > 0.0).map(|m| (id, a.name, m)))
        })
        .expect("some action limits a literal")
}

/// A zero-argument value the game rejects where a boolean is expected.
fn up() -> Expr {
    let id = ValueId::ALL
        .iter()
        .copied()
        .find(|id| {
            let signature = id.signature();
            !signature.flags.can_be_put_in_boolean()
                && !signature.flags.internal()
                && signature.args.is_empty()
        })
        .expect("some value is rejected in boolean slots");
    call(id, vec![])
}

fn wrapped(mut program: Program) -> Program {
    run_pass(&mut program);
    program
}

fn run_pass(program: &mut Program) {
    crate::lower::testutil::run(run, program, &options(OptLevel::Speed)).unwrap();
}

fn first_action_args(program: &Program) -> Vec<Expr> {
    match &program.rules[0].body.stmts[0].kind {
        StmtKind::Action { args, .. } => args.clone(),
        other => panic!("not an action: {other:?}"),
    }
}

fn arg_of(id: ActionId, name: &str, value: Expr) -> StmtKind {
    act(id, vec![(name, value)])
}

fn is_call(e: &Expr, id: ValueId) -> bool {
    matches!(e.kind(), ExprKind::Value { id: found, .. } if *found == id)
}

fn total(program: &Program) -> usize {
    count_rule(&program.rules[0], &program.options, Lang::EnUs).total
}

#[test]
fn literal_above_the_limit_is_wrapped_in_absolute_value() {
    let (id, name, max) = limited_action();
    let before = program(vec![arg_of(id, name, num(max + 1.0))]);
    let after = wrapped(before.clone());
    let position = id
        .signature()
        .args
        .iter()
        .position(|a| a.name == name)
        .unwrap();
    let arg = &first_action_args(&after)[position];
    assert!(is_call(arg, ValueId::Abs), "{arg:?}");
    // The wrapper is one more element.
    assert_eq!(total(&after), total(&before) + 1);
}

#[test]
fn literal_at_or_below_the_limit_is_left_alone() {
    let (id, name, max) = limited_action();
    for n in [max, max - 1.0, 0.0] {
        let before = program(vec![arg_of(id, name, num(n))]);
        assert_eq!(wrapped(before.clone()), before, "{n}");
    }
}

#[test]
fn only_number_literals_are_wrapped() {
    let (id, name, max) = limited_action();
    let value = expr(ExprKind::Add(num(max + 1.0), num(1.0)));
    let before = program(vec![arg_of(id, name, value)]);
    let after = wrapped(before.clone());
    // The nested literal sits in an `Add`, whose slots have no limit.
    assert_eq!(after, before);
}

#[test]
fn rejected_values_in_conditions_get_first_of() {
    let before = program(vec![if_(up(), vec![StmtKind::Pass])]);
    let after = wrapped(before.clone());
    let StmtKind::If { cond, .. } = &after.rules[0].body.stmts[0].kind else {
        panic!("not an if");
    };
    assert!(is_call(cond, ValueId::FirstOf), "{cond:?}");
    // `First Of(x)` costs one more element than `x`.
    assert_eq!(total(&after), total(&before) + 1);
}

#[test]
fn rejected_values_under_logic_operators_get_first_of() {
    let and = expr(ExprKind::And(up(), expr(ExprKind::Not(up()))));
    let after = wrapped(program(vec![StmtKind::AbortIf(and)]));
    let StmtKind::AbortIf(cond) = &after.rules[0].body.stmts[0].kind else {
        panic!("not an abort if");
    };
    let ExprKind::And(left, right) = cond.kind() else {
        panic!("not an and");
    };
    assert!(is_call(left, ValueId::FirstOf));
    let ExprKind::Not(inner) = right.kind() else {
        panic!("not a not");
    };
    assert!(is_call(inner, ValueId::FirstOf));
}

#[test]
fn accepted_values_are_left_alone() {
    let id = ValueId::ALL
        .iter()
        .copied()
        .find(|id| {
            let s = id.signature();
            s.flags.can_be_put_in_boolean() && !s.flags.internal() && s.ret == Ty::Bool
        })
        .expect("some boolean value is accepted");
    let before = program(vec![if_(call(id, vec![]), vec![StmtKind::Pass])]);
    assert_eq!(wrapped(before.clone()), before);
}

#[test]
fn comparisons_and_literals_are_left_alone() {
    let before = program(vec![
        if_(expr(ExprKind::Bool(true)), vec![]),
        StmtKind::LoopIf(null()),
    ]);
    assert_eq!(wrapped(before.clone()), before);
}

#[test]
fn rejected_values_in_nested_blocks_are_wrapped() {
    let inner = StmtKind::While {
        cond: up(),
        body: crate::lower::testutil::block(vec![]),
    };
    let after = wrapped(program(vec![if_(expr(ExprKind::Bool(true)), vec![inner])]));
    let StmtKind::If { then, .. } = &after.rules[0].body.stmts[0].kind else {
        panic!("not an if");
    };
    let StmtKind::While { cond, .. } = &then.stmts[0].kind else {
        panic!("not a while");
    };
    assert!(is_call(cond, ValueId::FirstOf));
}

#[test]
fn a_wrapped_boolean_condition_keeps_the_true_form() {
    // No value is both boolean and rejected today, so build the wrapped condition directly.
    let alive = call(ValueId::IsAlive, vec![call(ValueId::EventPlayer, vec![])]);
    let mut program = program(vec![]);
    program.rules[0].conditions = vec![Cond {
        expr: call(ValueId::FirstOf, vec![alive]),
        comment: None,
    }];
    let text = emit(&program, Lang::EnUs, None).unwrap();
    assert!(
        text.contains("First Of(Is Alive(Event Player)) == True;"),
        "{text}"
    );
}

#[test]
fn wrapped_output_prints_and_counts() {
    let program = wrapped(program(vec![if_(up(), vec![])]));
    let text = emit(&program, Lang::EnUs, None).unwrap();
    assert!(text.contains("If(First Of("), "{text}");
}
