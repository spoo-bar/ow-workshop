use ow_workshop_data::{ActionId, ValueId};

use super::*;
use crate::ir::{ModifyOp, Place, VarId};
use crate::lower::labels::resolve_labels;
use crate::lower::testutil::*;
use crate::options::OptLevel;

fn global(id: u32) -> Expr {
    expr(ExprKind::GlobalVar(VarId(id)))
}

fn add_one(var: u32) -> StmtKind {
    StmtKind::Modify {
        target: Place::Global(VarId(var)),
        op: ModifyOp::Add,
        value: num(1.0),
    }
}

fn switch(cases: Vec<(f64, Vec<StmtKind>)>, default: Option<(usize, Vec<StmtKind>)>) -> StmtKind {
    let (default_at, default) = match default {
        Some((at, body)) => (at, Some(block(body))),
        None => (cases.len(), None),
    };
    StmtKind::Switch {
        value: global(0),
        cases: cases.into_iter().map(|(c, b)| (num(c), block(b))).collect(),
        default,
        default_at,
    }
}

fn is_action(kind: &StmtKind, id: ActionId) -> bool {
    matches!(kind, StmtKind::Action { id: i, args } if *i == id && args.is_empty())
}

fn lowered(body: Vec<StmtKind>) -> Program {
    let mut p = program(body);
    run(lower, &mut p, &options(OptLevel::Size)).expect("lowers");
    p
}

/// The numbers in the skip table of the first switch, after labels are resolved.
fn table(p: &mut Program) -> Vec<f64> {
    run(resolve_labels, p, &options(OptLevel::Size)).expect("resolves");
    let skip = p.rules[0]
        .body
        .stmts
        .iter()
        .find_map(|s| match &s.kind {
            StmtKind::Skip(SkipTarget::Dynamic(e)) => Some(e.clone()),
            _ => None,
        })
        .expect("a skip");
    let ExprKind::Index { array, .. } = skip.kind() else {
        panic!("not a lookup")
    };
    let ExprKind::Array(items) = array.kind() else {
        panic!("not an array")
    };
    items
        .iter()
        .map(|e| match e.kind() {
            ExprKind::Num(n) => *n,
            other => panic!("not resolved: {other:?}"),
        })
        .collect()
}

#[test]
fn the_issue_fixture() {
    // switch A: case 1: B += 1; break; case 2: C += 1; break
    let mut p = lowered(vec![switch(
        vec![
            (1.0, vec![add_one(1), StmtKind::Break]),
            (2.0, vec![add_one(2), StmtKind::Break]),
        ],
        None,
    )]);
    let k = kinds(&p);
    assert_eq!(k.len(), 10);
    assert_eq!(k[0], &StmtKind::SwitchIf);
    assert!(matches!(k[1], StmtKind::Skip(SkipTarget::Dynamic(_))));
    assert!(matches!(k[2], StmtKind::Label(_)));
    assert!(matches!(k[3], StmtKind::Modify { .. }));
    assert!(is_action(k[4], ActionId::Else));
    assert!(matches!(k[5], StmtKind::Label(_)));
    assert!(matches!(k[6], StmtKind::Modify { .. }));
    assert!(is_action(k[7], ActionId::Else));
    assert!(matches!(k[8], StmtKind::Label(_)));
    assert!(is_action(k[9], ActionId::End));

    // Skip(Value In Array(Array(4, 0, 2), 1 + Index Of Array Value(Array(1, 2), Global.A)))
    let StmtKind::Skip(SkipTarget::Dynamic(sel)) = &p.rules[0].body.stmts[1].kind else {
        panic!("no skip")
    };
    let ExprKind::Index { index, .. } = sel.kind() else {
        panic!("not a lookup")
    };
    let ExprKind::Add(one, find) = index.kind() else {
        panic!("not an add")
    };
    assert_eq!(one.kind(), &ExprKind::Num(1.0));
    let ExprKind::Value { id, args } = find.kind() else {
        panic!("not a value")
    };
    assert_eq!(*id, ValueId::Index);
    assert_eq!(args[0].kind(), &ExprKind::Array(vec![num(1.0), num(2.0)]));
    assert_eq!(args[1].kind(), global(0).kind());
    assert_eq!(table(&mut p), [4.0, 0.0, 2.0]);
}

#[test]
fn default_in_the_middle() {
    // case 1: A; default: A; case 2: A  (no breaks)
    let mut p = lowered(vec![switch(
        vec![(1.0, vec![add_one(1)]), (2.0, vec![add_one(1)])],
        Some((1, vec![add_one(1)])),
    )]);
    // Bodies in source order: case 1, default, case 2.
    assert_eq!(table(&mut p), [1.0, 0.0, 2.0]);
}

#[test]
fn without_break_execution_falls_through() {
    let p = lowered(vec![switch(
        vec![(1.0, vec![add_one(1)]), (2.0, vec![add_one(2)])],
        None,
    )]);
    assert!(
        !kinds(&p)
            .iter()
            .any(|k| is_action(k, ActionId::Else) || matches!(k, StmtKind::Break))
    );
}

#[test]
fn nested_switches_each_get_their_own_if_and_end() {
    let inner = switch(vec![(7.0, vec![add_one(2), StmtKind::Break])], None);
    let p = lowered(vec![switch(
        vec![(1.0, vec![inner, add_one(3), StmtKind::Break])],
        None,
    )]);
    let k = kinds(&p);
    let count = |id| k.iter().filter(|k| is_action(k, id)).count();
    assert_eq!(k.iter().filter(|k| ***k == StmtKind::SwitchIf).count(), 2);
    assert_eq!(count(ActionId::End), 2);
    // One `break` per switch, each an `Else` at its own depth.
    assert_eq!(count(ActionId::Else), 2);
    // The outer `Else` comes after the inner `End`.
    let inner_end = k
        .iter()
        .position(|k| is_action(k, ActionId::End))
        .expect("end");
    let last_else = k
        .iter()
        .rposition(|k| is_action(k, ActionId::Else))
        .expect("else");
    assert!(last_else > inner_end);
}

#[test]
fn a_break_inside_an_if_skips_to_the_end() {
    let p = lowered(vec![switch(
        vec![(
            1.0,
            vec![if_(boolean(true), vec![StmtKind::Break]), add_one(1)],
        )],
        None,
    )]);
    let k = kinds(&p);
    let StmtKind::If { then, .. } = k[3] else {
        panic!("not an if: {:?}", k[3])
    };
    let StmtKind::Skip(SkipTarget::Label(exit)) = then.stmts[0].kind else {
        panic!("not a skip")
    };
    // The exit label sits right before the closing `End`.
    assert_eq!(k[k.len() - 2], &StmtKind::Label(exit));
    assert!(is_action(k[k.len() - 1], ActionId::End));
}

#[test]
fn a_break_inside_a_loop_belongs_to_the_loop() {
    let looped = StmtKind::While {
        cond: boolean(true),
        body: block(vec![StmtKind::Break]),
    };
    let p = lowered(vec![switch(vec![(1.0, vec![looped])], None)]);
    let StmtKind::While { body, .. } = kinds(&p)[3] else {
        panic!("not a loop")
    };
    assert_eq!(body.stmts[0].kind, StmtKind::Break);
}

#[test]
fn a_switch_inside_a_loop_is_lowered_too() {
    let inner = switch(vec![(1.0, vec![StmtKind::Break])], None);
    let p = lowered(vec![StmtKind::While {
        cond: boolean(true),
        body: block(vec![inner]),
    }]);
    let StmtKind::While { body, .. } = kinds(&p)[0] else {
        panic!("not a loop")
    };
    assert_eq!(body.stmts[0].kind, StmtKind::SwitchIf);
}
