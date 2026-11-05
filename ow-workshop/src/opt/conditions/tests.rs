use super::*;
use crate::ir::Cond;
use crate::lower::testutil::*;
use crate::opt::testutil::*;
use crate::options::OptLevel;

fn cond(e: crate::ir::Expr) -> Cond {
    Cond {
        expr: e,
        comment: None,
    }
}

fn shown(conds: &[Cond]) -> Vec<String> {
    conds.iter().map(|c| sh(&c.expr)).collect()
}

#[test]
fn and_splits_into_two_conditions() {
    let both = simplify(&[cond(and(g(0), g(1)))]).expect("runs");
    assert_eq!(shown(&both), ["g0", "g1"]);
    let nested = simplify(&[cond(and(and(g(0), g(1)), g(2)))]).expect("runs");
    assert_eq!(shown(&nested), ["g0", "g1", "g2"]);
}

#[test]
fn the_first_part_keeps_the_comment() {
    let commented = Cond {
        expr: and(g(0), g(1)),
        comment: Some("why".to_owned()),
    };
    let out = simplify(&[commented]).expect("runs");
    assert_eq!(out[0].comment.as_deref(), Some("why"));
    assert_eq!(out[1].comment, None);
}

#[test]
fn a_truthy_condition_is_dropped() {
    let out = simplify(&[cond(tru()), cond(g(0)), cond(num(3.0))]).expect("runs");
    assert_eq!(shown(&out), ["g0"]);
}

#[test]
fn a_falsy_condition_stops_the_rule() {
    assert!(simplify(&[cond(g(0)), cond(fals())]).is_none());
    assert!(simplify(&[cond(and(g(0), null()))]).is_none());
}

fn rule_with(conditions: Vec<Cond>, delimiter: bool) -> crate::ir::Program {
    let mut p = program(vec![action()]);
    p.rules[0].conditions = conditions;
    p.rules[0].delimiter = delimiter;
    p
}

fn optimized(mut p: crate::ir::Program, level: OptLevel) -> crate::ir::Program {
    run(
        crate::opt::run,
        &mut p,
        &crate::lower::testutil::options(level),
    )
    .expect("optimizes");
    p
}

#[test]
fn a_falsy_condition_removes_the_rule() {
    let p = optimized(
        rule_with(vec![cond(boolean(false))], false),
        OptLevel::Speed,
    );
    assert_eq!(p.rules.len(), 0);
}

#[test]
fn a_delimiter_is_never_removed() {
    let p = optimized(rule_with(vec![cond(boolean(false))], true), OptLevel::Speed);
    assert_eq!(p.rules.len(), 1);
}

#[test]
fn conditions_are_optimized_and_simplified_in_a_program() {
    let c = cond(and(
        boolean(true),
        and(expr(crate::ir::ExprKind::Not(g(0))), g(1)),
    ));
    let p = optimized(rule_with(vec![c], false), OptLevel::Speed);
    assert_eq!(shown(&p.rules[0].conditions), ["!g0", "g1"]);
}

#[test]
fn nothing_changes_with_the_optimizer_off() {
    let before = rule_with(vec![cond(boolean(false)), cond(and(g(0), g(1)))], false);
    let p = optimized(before.clone(), OptLevel::Off);
    assert_eq!(p, before);
}
