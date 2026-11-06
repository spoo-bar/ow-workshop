//! Folding of math values: trigonometry, vectors, `Min` and `Max`.

// Operands are named after the maths they mirror.
#![allow(clippy::many_single_char_names)]

use ow_workshop_data::ValueId;

use super::Env;
use super::build::{call_args, num, num_of, value, vec_of, vec3};
use super::helpers::{always_equal, fold_guard};
use crate::ir::{Expr, ExprKind};

fn fold(n: f64, like: &Expr) -> Option<Expr> {
    fold_guard(n).map(|n| num(n, like.loc()))
}

fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [b[0] - a[0], b[1] - a[1], b[2] - a[2]]
}

fn length(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn normalized(v: [f64; 3]) -> [f64; 3] {
    let len = length(v);
    if len == 0.0 {
        [0.0; 3]
    } else {
        v.map(|p| p / len)
    }
}

fn fold_vec(v: [f64; 3], like: &Expr) -> Option<Expr> {
    v.iter()
        .all(|n| fold_guard(*n).is_some())
        .then(|| vec3(v, like.loc()))
}

/// The unary number functions and their folds.
fn unary(id: ValueId, x: f64) -> Option<f64> {
    Some(match id {
        ValueId::Abs => x.abs(),
        ValueId::Sqrt => x.max(0.0).sqrt(),
        ValueId::Sin => x.sin(),
        ValueId::SinDeg => x.to_radians().sin(),
        ValueId::Cos => x.cos(),
        ValueId::CosDeg => x.to_radians().cos(),
        ValueId::Tan => x.tan(),
        ValueId::TanDeg => x.to_radians().tan(),
        ValueId::Asin => x.clamp(-1.0, 1.0).asin(),
        ValueId::AsinDeg => x.clamp(-1.0, 1.0).asin().to_degrees(),
        ValueId::Acos => x.clamp(-1.0, 1.0).acos(),
        ValueId::AcosDeg => x.clamp(-1.0, 1.0).acos().to_degrees(),
        _ => return None,
    })
}

fn round(e: &Expr, args: &[Expr]) -> Option<Expr> {
    let [value, mode] = args else { return None };
    let ExprKind::Const(c) = mode.kind() else {
        return None;
    };
    let x = num_of(value)?;
    let rounded = match (c.enum_name, c.key) {
        ("Rounding", "__roundDown__") => x.floor(),
        ("Rounding", "__roundUp__") => x.ceil(),
        ("Rounding", "__roundToNearest__") => x.round(),
        _ => return None,
    };
    fold(rounded, e)
}

fn extremum(e: &Expr, id: ValueId) -> Option<Expr> {
    let mut operands = Vec::new();
    flatten(e, id, &mut operands);
    let (nums, rest): (Vec<&Expr>, Vec<&Expr>) =
        operands.iter().copied().partition(|o| num_of(o).is_some());
    let best = nums.iter().filter_map(|o| num_of(o)).reduce(|a, b| {
        if id == ValueId::Min {
            a.min(b)
        } else {
            a.max(b)
        }
    });
    if rest.is_empty() {
        return fold(best?, e);
    }
    if nums.len() < 2 {
        return None;
    }
    // Only the winning literal can matter next to the unknown operands.
    let mut keep: Vec<Expr> = rest.into_iter().cloned().collect();
    keep.push(num(best?, e.loc()));
    let mut keep = keep.into_iter();
    let first = keep.next()?;
    Some(keep.fold(first, |acc, next| value(id, vec![acc, next], e.loc())))
}

fn flatten<'a>(e: &'a Expr, id: ValueId, out: &mut Vec<&'a Expr>) {
    match call_args(e, id) {
        Some([a, b]) => {
            flatten(a, id, out);
            flatten(b, id, out);
        }
        _ => out.push(e),
    }
}

/// Rewrites of calls of math values.
pub(super) fn rule(e: &Expr, _: &mut Env<'_, '_>) -> Option<Expr> {
    let ExprKind::Value { id, args } = e.kind() else {
        return None;
    };
    let loc = e.loc();
    match (*id, args.as_slice()) {
        (ValueId::Round, _) => round(e, args),
        (ValueId::Atan2 | ValueId::Atan2Deg, [y, x]) => {
            let (y, x) = (num_of(y)?, num_of(x)?);
            let angle = y.atan2(x);
            fold(
                if *id == ValueId::Atan2 {
                    angle
                } else {
                    angle.to_degrees()
                },
                e,
            )
        }
        (ValueId::Sqrt, [a]) => {
            if let Some([p, q]) = call_args(a, ValueId::DotProduct) {
                if always_equal(p, q) {
                    return Some(value(ValueId::Magnitude, vec![p.clone()], loc));
                }
            }
            fold(unary(*id, num_of(a)?)?, e)
        }
        (ValueId::Distance, [a, b]) => {
            if let (Some(p), Some(q)) = (vec_of(a), vec_of(b)) {
                return fold(length(sub3(p, q)), e);
            }
            if super::build::is_zero_vec(a) {
                return Some(value(ValueId::Magnitude, vec![b.clone()], loc));
            }
            super::build::is_zero_vec(b).then(|| value(ValueId::Magnitude, vec![a.clone()], loc))
        }
        (ValueId::Magnitude, [a]) => fold(length(vec_of(a)?), e),
        (ValueId::Normalize, [a]) => {
            if let Some([p, q]) = call_args(a, ValueId::VectorTowards) {
                return Some(value(
                    ValueId::DirectionTowards,
                    vec![p.clone(), q.clone()],
                    loc,
                ));
            }
            fold_vec(normalized(vec_of(a)?), e)
        }
        (ValueId::DirectionTowards, [a, b]) => {
            fold_vec(normalized(sub3(vec_of(a)?, vec_of(b)?)), e)
        }
        (ValueId::VectorTowards, [a, b]) => fold_vec(sub3(vec_of(a)?, vec_of(b)?), e),
        (ValueId::CrossProduct, [a, b]) => {
            let (p, q) = (vec_of(a)?, vec_of(b)?);
            fold_vec(
                [
                    p[1] * q[2] - p[2] * q[1],
                    p[2] * q[0] - p[0] * q[2],
                    p[0] * q[1] - p[1] * q[0],
                ],
                e,
            )
        }
        (ValueId::DotProduct, [a, b]) => {
            let (p, q) = (vec_of(a)?, vec_of(b)?);
            fold(p[0] * q[0] + p[1] * q[1] + p[2] * q[2], e)
        }
        (ValueId::Min | ValueId::Max, _) => extremum(e, *id),
        (_, [a]) => fold(unary(*id, num_of(a)?)?, e),
        _ => None,
    }
}

#[cfg(test)]
mod tests;
