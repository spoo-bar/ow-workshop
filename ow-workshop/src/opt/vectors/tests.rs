use crate::ir::{Expr, Program, StmtKind};
use crate::lower::testutil::{act, num, options, program, run};
use crate::opt::testutil::*;
use crate::options::OptLevel;

use ow_workshop_data::ActionId;

fn printed(v: Expr, level: OptLevel) -> String {
    let mut p: Program = program(vec![
        StmtKind::Set {
            target: crate::ir::Place::Global(crate::ir::VarId(0)),
            value: v,
        },
        act(ActionId::Kill, vec![]),
    ]);
    run(crate::opt::run, &mut p, &options(level)).expect("optimizes");
    match &p.rules[0].body.stmts[0].kind {
        StmtKind::Set { value, .. } => sh(value),
        other => panic!("not a set: {other:?}"),
    }
}

fn speed(v: Expr) -> String {
    printed(v, OptLevel::Speed)
}

fn size(v: Expr) -> String {
    printed(v, OptLevel::Size)
}

#[test]
fn unit_vectors_print_as_direction_constants() {
    for (v, name) in [
        (v3(1.0, 0.0, 0.0), "VectorLeft()"),
        (v3(-1.0, 0.0, 0.0), "VectorRight()"),
        (v3(0.0, 1.0, 0.0), "VectorUp()"),
        (v3(0.0, -1.0, 0.0), "VectorDown()"),
        (v3(0.0, 0.0, 1.0), "VectorForward()"),
        (v3(0.0, 0.0, -1.0), "VectorBackward()"),
    ] {
        assert_eq!(speed(v.clone()), name);
        assert_eq!(size(v), name);
    }
}

#[test]
fn other_vectors_stay_vectors_outside_size_mode() {
    assert_eq!(speed(v3(1.0, 1.0, 0.0)), "V(1, 1, 0)");
    assert_eq!(speed(v3(5.0, 0.0, 0.0)), "V(5, 0, 0)");
    assert_eq!(speed(v3(0.0, 0.0, 0.0)), "V(0, 0, 0)");
    assert_eq!(printed(v3(1.0, 0.0, 0.0), OptLevel::Off), "V(1, 0, 0)");
}

#[test]
fn two_unit_components_are_a_sum_of_constants() {
    assert_eq!(size(v3(1.0, 1.0, 0.0)), "(VectorLeft() + VectorUp())");
    assert_eq!(
        size(v3(-1.0, 0.0, 1.0)),
        "(VectorRight() + VectorForward())"
    );
    assert_eq!(
        size(v3(0.0, -1.0, -1.0)),
        "(VectorDown() + VectorBackward())"
    );
}

#[test]
fn one_other_component_is_a_multiple() {
    assert_eq!(size(v3(5.0, 0.0, 0.0)), "(5 * VectorLeft())");
    assert_eq!(size(v3(0.0, -3.0, 0.0)), "(-3 * VectorUp())");
    assert_eq!(size(v3(0.0, 0.0, 2.5)), "(2.5 * VectorForward())");
}

#[test]
fn remaining_vectors_get_cheaper_components() {
    // An assigned zero vector is replaced as a whole instead.
    assert_eq!(size(v3(0.0, 0.0, 0.0)), "(VectorLeft() + VectorRight())");
    assert_eq!(size(v3(2.0, 1.0, 3.0)), "V(2, true, 3)");
    assert_eq!(size(v3(1.0, 1.0, 1.0)), "V(true, true, true)");
}

#[test]
fn constants_still_fold_while_optimizing() {
    let left = val(ow_workshop_data::ValueId::VectorLeft, vec![]);
    assert_eq!(size(mul(left, num(2.0))), "(2 * VectorLeft())");
    assert_eq!(
        speed(add(v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0))),
        "V(1, 1, 0)"
    );
}
