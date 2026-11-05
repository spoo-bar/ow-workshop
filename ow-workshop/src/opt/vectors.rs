//! Prints literal vectors as direction constants (the workshop spells `Vector(1, 0, 0)` `Left`).
//!
//! Runs in the finish sweep: while rewriting, the constants are plain vectors so they fold.

use ow_workshop_data::ValueId as V;

use super::Env;
use super::build::{num_of, value};
use crate::ir::{Expr, ExprKind, Location};

/// The direction constant for one axis (0 = x, 1 = y, 2 = z) and sign.
fn unit(axis: usize, positive: bool) -> V {
    match (axis, positive) {
        (0, true) => V::VectorLeft,
        (0, false) => V::VectorRight,
        (1, true) => V::VectorUp,
        (1, false) => V::VectorDown,
        (2, true) => V::VectorForward,
        _ => V::VectorBackward,
    }
}

fn constant(axis: usize, positive: bool, loc: Option<Location>) -> Expr {
    value(unit(axis, positive), Vec::new(), loc)
}

#[allow(clippy::float_cmp)] // unit components are exactly ±1
/// Direction constants for literal vectors; in size mode also sums and multiples of them.
pub(super) fn rule(e: &Expr, env: &mut Env<'_, '_>) -> Option<Expr> {
    let ExprKind::Vector(parts) = e.kind() else {
        return None;
    };
    let v = [num_of(&parts[0])?, num_of(&parts[1])?, num_of(&parts[2])?];
    let loc = e.loc();
    let set: Vec<usize> = (0..3).filter(|i| v[*i] != 0.0).collect();
    let units: Vec<usize> = set.iter().copied().filter(|i| v[*i].abs() == 1.0).collect();
    match (set.as_slice(), units.len()) {
        ([axis], 1) => Some(constant(*axis, v[*axis] > 0.0, loc)),
        ([axis], _) if env.scope.size() => {
            let along = constant(*axis, true, loc);
            Some(Expr::new(
                ExprKind::Mul(Expr::new(ExprKind::Num(v[*axis]), loc), along),
                loc,
            ))
        }
        ([a, b], 2) if env.scope.size() => Some(Expr::new(
            ExprKind::Add(
                constant(*a, v[*a] > 0.0, loc),
                constant(*b, v[*b] > 0.0, loc),
            ),
            loc,
        )),
        _ if env.scope.size() => {
            let parts: &[Expr; 3] = parts;
            super::size::vector_parts(parts, env).map(|p| Expr::new(ExprKind::Vector(p), loc))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests;
