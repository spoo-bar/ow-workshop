use super::*;
use crate::ir::{ModifyOp, Place, VarId};
use crate::lower::testutil::*;
use crate::options::OptLevel;

fn var(id: u32) -> Expr {
    expr(ExprKind::GlobalVar(VarId(id)))
}

fn grid(indices: &[u32]) -> Place {
    let mut place = Place::Global(VarId(0));
    for &i in indices {
        place = Place::Index {
            base: Box::new(place),
            index: var(i),
        };
    }
    place
}

fn lowered(kind: StmtKind) -> Program {
    let mut p = program(vec![kind]);
    run(lower, &mut p, &options(OptLevel::Size)).expect("lowers");
    p
}

/// `target = value` of the first statement, rendered.
fn set(p: &Program) -> String {
    let StmtKind::Set { target, value } = &p.rules[0].body.stmts[0].kind else {
        panic!("not a set: {:?}", p.rules[0].body.stmts[0].kind)
    };
    let (_, indices) = split(target);
    let path: Vec<String> = indices.iter().map(|i| format!("[{}]", show(i))).collect();
    format!("g0{} = {}", path.concat(), show(value))
}

const END: &str = "999999999999";

fn dict_get(default: Option<Expr>, key: Expr) -> Expr {
    expr(ExprKind::Index {
        array: expr(ExprKind::Dict {
            entries: vec![(num(10.0), num(2.0)), (num(11.0), num(3.0))],
            default,
        }),
        index: key,
    })
}

fn lowered_expr(e: Expr) -> String {
    let p = lowered(StmtKind::Set {
        target: Place::Global(VarId(0)),
        value: e,
    });
    let StmtKind::Set { value, .. } = &p.rules[0].body.stmts[0].kind else {
        panic!("not a set")
    };
    show(value)
}

#[test]
fn dictionary_without_default() {
    assert_eq!(
        lowered_expr(dict_get(None, var(1))),
        "[2, 3][Index([10, 11], g1)]"
    );
}

#[test]
fn dictionary_with_default() {
    assert_eq!(
        lowered_expr(dict_get(Some(num(0.0)), var(1))),
        "[0, 2, 3][(1 + Index([10, 11], g1))]"
    );
}

#[test]
fn dictionary_in_a_condition_and_nested() {
    let inner = dict_get(None, var(1));
    let outer = dict_get(None, inner);
    assert_eq!(
        lowered_expr(outer),
        "[2, 3][Index([10, 11], [2, 3][Index([10, 11], g1)])]"
    );
}

#[test]
fn three_dimensional_set() {
    let p = lowered(StmtKind::Set {
        target: grid(&[1, 2, 3]),
        value: num(5.0),
    });
    let inner = format!(
        "Concat(Concat(Slice(g0[g1][g2], 0, g3), [5]), Slice(g0[g1][g2], (g3 + 1), {END}))"
    );
    assert_eq!(
        set(&p),
        format!(
            "g0[g1] = Concat(Concat(Slice(g0[g1], 0, g2), [{inner}]), Slice(g0[g1], (g2 + 1), {END}))"
        )
    );
}

#[test]
fn deleting_from_a_three_dimensional_place() {
    let p = lowered(StmtKind::Modify {
        target: grid(&[1, 2, 3]),
        op: ModifyOp::RemoveFromArrayByIndex,
        value: var(4),
    });
    let inner = format!(
        "Concat(Concat(Slice(g0[g1][g2], 0, g3), [FilteredArray(g0[g1][g2][g3], (# Ne g4))]), Slice(g0[g1][g2], (g3 + 1), {END}))"
    );
    assert_eq!(
        set(&p),
        format!(
            "g0[g1] = Concat(Concat(Slice(g0[g1], 0, g2), [{inner}]), Slice(g0[g1], (g2 + 1), {END}))"
        )
    );
}

#[test]
fn append_and_remove_on_a_multi_dimensional_place() {
    let append = lowered(StmtKind::Modify {
        target: grid(&[1, 2]),
        op: ModifyOp::AppendToArray,
        value: num(7.0),
    });
    assert!(set(&append).contains("[Concat(g0[g1][g2], 7)]"));
    let remove = lowered(StmtKind::Modify {
        target: grid(&[1, 2]),
        op: ModifyOp::RemoveFromArrayByValue,
        value: num(7.0),
    });
    assert!(set(&remove).contains("[Exclude(g0[g1][g2], 7)]"));
}

#[test]
fn arithmetic_on_a_multi_dimensional_place() {
    let p = lowered(StmtKind::Modify {
        target: grid(&[1, 2]),
        op: ModifyOp::Add,
        value: num(1.0),
    });
    assert!(set(&p).contains("[(g0[g1][g2] + 1)]"));
}

#[test]
fn one_dimensional_places_keep_their_native_actions() {
    for kind in [
        StmtKind::Set {
            target: grid(&[1]),
            value: num(5.0),
        },
        StmtKind::Modify {
            target: grid(&[1]),
            op: ModifyOp::AppendToArray,
            value: num(5.0),
        },
        StmtKind::Modify {
            target: grid(&[]),
            op: ModifyOp::RemoveFromArrayByIndex,
            value: num(0.0),
        },
    ] {
        let p = lowered(kind.clone());
        assert_eq!(p.rules[0].body.stmts[0].kind, kind);
    }
}

#[test]
fn nested_blocks_and_places_are_lowered() {
    let p = lowered(if_(
        boolean(true),
        vec![StmtKind::Set {
            target: grid(&[1, 2]),
            value: dict_get(None, var(3)),
        }],
    ));
    let StmtKind::If { then, .. } = &p.rules[0].body.stmts[0].kind else {
        panic!("not an if")
    };
    let StmtKind::Set { target, value } = &then.stmts[0].kind else {
        panic!("not a set")
    };
    assert_eq!(depth(target), 1);
    assert!(show(value).contains("[2, 3][Index([10, 11], g3)]"));
}

#[test]
fn a_dictionary_that_is_not_read_is_an_error() {
    let mut p = program(vec![StmtKind::Set {
        target: Place::Global(VarId(0)),
        value: expr(ExprKind::Dict {
            entries: vec![],
            default: None,
        }),
    }]);
    let err = run(lower, &mut p, &options(OptLevel::Size)).expect_err("unread");
    assert_eq!(codes(&err), ["dict-not-indexed"]);
}
