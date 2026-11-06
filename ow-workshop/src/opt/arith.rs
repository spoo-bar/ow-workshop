//! Constant folding and identities for the arithmetic operators.
//!
//! Rules marked "non-strict" are skipped when `strict` is on: they change behaviour under
//! workshop type coercion (`A * 0` may be a vector, `A + 0` may not be a number).

// Operands are named after the maths they mirror.
#![allow(clippy::many_single_char_names)]

use super::Env;
use super::build::{is_num, is_zero_vec, num, num_of, vec_of, vec3};
use super::helpers::{always_equal, fold_guard};
use super::ty::{Kind, kind};
use crate::ir::{Expr, ExprKind};
use crate::warning::Warning;

/// Folds to a number literal unless the result would lose precision in game.
fn fold(n: f64, like: &Expr) -> Option<Expr> {
    fold_guard(n).map(|n| num(n, like.loc()))
}

/// Folds a vector the same way, component by component.
fn fold_vec(v: [f64; 3], like: &Expr) -> Option<Expr> {
    v.iter()
        .all(|n| fold_guard(*n).is_some())
        .then(|| vec3(v, like.loc()))
}

fn zip(a: [f64; 3], b: [f64; 3], f: impl Fn(f64, f64) -> f64) -> [f64; 3] {
    [f(a[0], b[0]), f(a[1], b[1]), f(a[2], b[2])]
}

/// The workshop has no negate: a negated literal becomes a negative literal, always.
pub(super) fn negative_literal(e: &Expr, _: &mut Env<'_, '_>) -> Option<Expr> {
    let ExprKind::Neg(a) = e.kind() else {
        return None;
    };
    num_of(a).map(|n| num(-n, e.loc()))
}

/// The arithmetic operators and `Negate`.
pub(super) fn rule(e: &Expr, env: &mut Env<'_, '_>) -> Option<Expr> {
    let strict = env.scope.strict();
    match e.kind() {
        ExprKind::Add(a, b) => add(e, a, b, strict),
        ExprKind::Sub(a, b) => sub(e, a, b),
        ExprKind::Mul(a, b) => mul(e, a, b, strict),
        ExprKind::Div(a, b) => div(e, a, b, strict, env),
        ExprKind::Mod(a, b) => modulo(e, a, b),
        ExprKind::Pow(a, b) => pow(e, a, b),
        ExprKind::Neg(a) => negate(e, a),
        _ => None,
    }
}

fn add(e: &Expr, a: &Expr, b: &Expr, strict: bool) -> Option<Expr> {
    if let (Some(x), Some(y)) = (num_of(a), num_of(b)) {
        return fold(x + y, e);
    }
    if let (Some(x), Some(y)) = (vec_of(a), vec_of(b)) {
        return fold_vec(zip(x, y, |p, q| p + q), e);
    }
    if !strict {
        if is_num(a, 0.0) || is_zero_vec(a) {
            return Some(b.clone());
        }
        if is_num(b, 0.0) || is_zero_vec(b) {
            return Some(a.clone());
        }
    }
    always_equal(a, b).then(|| Expr::new(ExprKind::Mul(num(2.0, e.loc()), a.clone()), e.loc()))
}

fn sub(e: &Expr, a: &Expr, b: &Expr) -> Option<Expr> {
    if let (Some(x), Some(y)) = (num_of(a), num_of(b)) {
        return fold(x - y, e);
    }
    if let (Some(x), Some(y)) = (vec_of(a), vec_of(b)) {
        return fold_vec(zip(x, y, |p, q| p - q), e);
    }
    if is_num(b, 0.0) || is_zero_vec(b) {
        return Some(a.clone());
    }
    // `A * 0`, not `0`: `A` may be a vector.
    always_equal(a, b).then(|| Expr::new(ExprKind::Mul(a.clone(), num(0.0, e.loc())), e.loc()))
}

fn mul(e: &Expr, a: &Expr, b: &Expr, strict: bool) -> Option<Expr> {
    if let (Some(x), Some(y)) = (num_of(a), num_of(b)) {
        return fold(x * y, e);
    }
    match (vec_of(a), vec_of(b), num_of(a), num_of(b)) {
        (Some(x), Some(y), ..) => return fold_vec(zip(x, y, |p, q| p * q), e),
        (Some(v), None, _, Some(k)) | (None, Some(v), Some(k), _) => {
            return fold_vec(v.map(|p| p * k), e);
        }
        _ => {}
    }
    for (zero, other) in [(a, b), (b, a)] {
        if is_num(zero, 0.0) && (!strict || kind(other).is_number()) {
            return Some(num(0.0, e.loc()));
        }
    }
    if !strict {
        if is_num(a, 1.0) {
            return Some(b.clone());
        }
        if is_num(b, 1.0) {
            return Some(a.clone());
        }
    }
    (kind(a).is_unsigned() && always_equal(a, b))
        .then(|| Expr::new(ExprKind::Pow(a.clone(), num(2.0, e.loc())), e.loc()))
}

fn div(e: &Expr, a: &Expr, b: &Expr, strict: bool, env: &mut Env<'_, '_>) -> Option<Expr> {
    match (num_of(a), num_of(b), vec_of(a), vec_of(b)) {
        (Some(x), Some(y), ..) if y != 0.0 => return fold(x / y, e),
        (_, _, Some(x), Some(y)) if y.iter().all(|c| *c != 0.0) => {
            return fold_vec(zip(x, y, |p, q| p / q), e);
        }
        (_, Some(y), Some(x), _) if y != 0.0 => return fold_vec(x.map(|p| p / y), e),
        (Some(_), _, _, Some(_)) => {
            env.warn(
                Warning::TypeCheck,
                "dividing a number by a vector is not a vector division".to_owned(),
                e.loc(),
            );
            return None;
        }
        _ => {}
    }
    if strict {
        return None;
    }
    if is_num(b, 1.0) {
        return Some(a.clone());
    }
    (is_num(b, 0.0) || is_num(a, 0.0)).then(|| num(0.0, e.loc()))
}

fn modulo(e: &Expr, a: &Expr, b: &Expr) -> Option<Expr> {
    if let (Some(x), Some(y)) = (num_of(a), num_of(b)) {
        if y != 0.0 {
            return fold(x % y.abs(), e);
        }
    }
    (always_equal(a, b) || is_num(a, 0.0) || is_num(b, 0.0)).then(|| num(0.0, e.loc()))
}

fn pow(e: &Expr, a: &Expr, b: &Expr) -> Option<Expr> {
    // The game returns 0 for a negative base.
    if num_of(a).is_some_and(|x| x < 0.0) {
        return Some(num(0.0, e.loc()));
    }
    if let (Some(x), Some(y)) = (num_of(a), num_of(b)) {
        return fold(x.powf(y), e);
    }
    if is_num(b, 1.0) {
        return Some(a.clone());
    }
    if is_num(a, 0.0) {
        return Some(num(0.0, e.loc()));
    }
    is_num(a, 1.0).then(|| num(1.0, e.loc()))
}

/// Pushes a negation into the literal factor; otherwise `Multiply(-1, A)`.
fn negate(e: &Expr, a: &Expr) -> Option<Expr> {
    let loc = e.loc();
    if let Some(v) = vec_of(a) {
        return Some(vec3(v.map(|p| -p), loc));
    }
    let negated = |n: f64| num(-n, loc);
    let rebuilt = match a.kind() {
        ExprKind::Mul(x, y) => match (num_of(x), num_of(y)) {
            (Some(n), _) => Some(ExprKind::Mul(negated(n), y.clone())),
            (_, Some(n)) => Some(ExprKind::Mul(x.clone(), negated(n))),
            _ => None,
        },
        ExprKind::Div(x, y) => match (num_of(x), num_of(y)) {
            (Some(n), _) => Some(ExprKind::Div(negated(n), y.clone())),
            (_, Some(n)) => Some(ExprKind::Div(x.clone(), negated(n))),
            _ => None,
        },
        ExprKind::Mod(x, y) => num_of(x).map(|n| ExprKind::Mod(negated(n), y.clone())),
        _ => None,
    };
    let kind = rebuilt.unwrap_or_else(|| ExprKind::Mul(num(-1.0, loc), a.clone()));
    Some(Expr::new(kind, loc))
}

#[cfg(test)]
mod tests;
