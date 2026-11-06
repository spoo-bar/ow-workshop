//! Small constructors and readers for the nodes the rewrite rules build.

use ow_workshop_data::ValueId;

use crate::ir::{Expr, ExprKind, Location};

/// A number literal; `-0` becomes `0`.
pub(super) fn num(n: f64, loc: Option<Location>) -> Expr {
    Expr::new(ExprKind::Num(if n == 0.0 { 0.0 } else { n }), loc)
}

/// A vector of number literals.
pub(super) fn vec3(v: [f64; 3], loc: Option<Location>) -> Expr {
    Expr::new(ExprKind::Vector(v.map(|n| num(n, loc))), loc)
}

/// A call of a workshop value.
pub(super) fn value(id: ValueId, args: Vec<Expr>, loc: Option<Location>) -> Expr {
    Expr::new(ExprKind::Value { id, args }, loc)
}

/// The number if `e` is a number literal.
pub(super) fn num_of(e: &Expr) -> Option<f64> {
    match e.kind() {
        ExprKind::Num(n) => Some(*n),
        _ => None,
    }
}

/// The components if `e` is a vector of three number literals.
pub(super) fn vec_of(e: &Expr) -> Option<[f64; 3]> {
    let ExprKind::Vector([x, y, z]) = e.kind() else {
        return None;
    };
    Some([num_of(x)?, num_of(y)?, num_of(z)?])
}

/// Whether `e` is the number `n`.
pub(super) fn is_num(e: &Expr, n: f64) -> bool {
    num_of(e) == Some(n)
}

/// Whether `e` is `Vector(0, 0, 0)`.
pub(super) fn is_zero_vec(e: &Expr) -> bool {
    vec_of(e) == Some([0.0; 3])
}

/// The arguments of a call of `id`.
pub(super) fn call_args(e: &Expr, id: ValueId) -> Option<&[Expr]> {
    match e.kind() {
        ExprKind::Value { id: got, args } if *got == id => Some(args),
        _ => None,
    }
}
