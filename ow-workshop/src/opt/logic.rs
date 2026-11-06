//! Rewrites of `Not`, `And`, `Or`, comparisons and `If-Then-Else`.
//!
//! Rules marked "non-strict" are skipped when `strict` is on.

use ow_workshop_data::ValueId;

use super::Env;
use super::build::{call_args, num_of, value};
use super::helpers::{always_equal, definitely_falsy, definitely_truthy, is_literal};
use super::ty::{Kind, kind};
use crate::ir::{CmpOp, Expr, ExprKind};

fn boolean(b: bool, like: &Expr) -> Expr {
    Expr::new(ExprKind::Bool(b), like.loc())
}

fn negated(a: &Expr, like: &Expr) -> Expr {
    Expr::new(ExprKind::Not(a.clone()), like.loc())
}

fn is_true(e: &Expr) -> bool {
    matches!(
        e.kind(),
        ExprKind::Bool(true)
            | ExprKind::Value {
                id: ValueId::True,
                ..
            }
    )
}

fn is_bool(e: &Expr) -> bool {
    kind(e) == Kind::Bool
}

/// Whether `a` is `not b` or `b` is `not a`.
fn opposite(a: &Expr, b: &Expr) -> bool {
    let inverse =
        |x: &Expr, y: &Expr| matches!(x.kind(), ExprKind::Not(inner) if always_equal(inner, y));
    inverse(a, b) || inverse(b, a)
}

/// The comparison that holds exactly when `op` does not.
pub(super) fn inverse(op: CmpOp) -> CmpOp {
    match op {
        CmpOp::Eq => CmpOp::Ne,
        CmpOp::Ne => CmpOp::Eq,
        CmpOp::Gt => CmpOp::Le,
        CmpOp::Ge => CmpOp::Lt,
        CmpOp::Lt => CmpOp::Ge,
        CmpOp::Le => CmpOp::Gt,
    }
}

/// `Not`, `And`, `Or`, comparisons and `If-Then-Else`.
pub(super) fn rule(e: &Expr, env: &mut Env<'_, '_>) -> Option<Expr> {
    let strict = env.scope.strict();
    match e.kind() {
        ExprKind::Not(a) => not(e, a),
        ExprKind::And(a, b) => and(e, a, b, strict),
        ExprKind::Or(a, b) => or(e, a, b, strict),
        ExprKind::Compare { op, lhs, rhs } => compare(e, *op, lhs, rhs, strict),
        ExprKind::IfThenElse { cond, then, else_ } => if_then_else(cond, then, else_),
        _ => None,
    }
}

fn not(e: &Expr, a: &Expr) -> Option<Expr> {
    if definitely_falsy(a) {
        return Some(boolean(true, e));
    }
    if definitely_truthy(a) {
        return Some(boolean(false, e));
    }
    match a.kind() {
        ExprKind::Not(inner) if is_bool(inner) => Some(inner.clone()),
        ExprKind::Compare { op, lhs, rhs } => Some(Expr::new(
            ExprKind::Compare {
                op: inverse(*op),
                lhs: lhs.clone(),
                rhs: rhs.clone(),
            },
            e.loc(),
        )),
        _ => {
            let flipped = if call_args(a, ValueId::IsAlive).is_some() {
                ValueId::IsDead
            } else if call_args(a, ValueId::IsDead).is_some() {
                ValueId::IsAlive
            } else {
                return None;
            };
            let ExprKind::Value { args, .. } = a.kind() else {
                return None;
            };
            Some(value(flipped, args.clone(), e.loc()))
        }
    }
}

fn and(e: &Expr, a: &Expr, b: &Expr, strict: bool) -> Option<Expr> {
    if definitely_falsy(a) {
        return Some(a.clone());
    }
    if definitely_truthy(a) {
        return Some(b.clone());
    }
    if !strict {
        if definitely_falsy(b) {
            return Some(b.clone());
        }
        if definitely_truthy(b) {
            return Some(a.clone());
        }
        if opposite(a, b) {
            return Some(boolean(false, e));
        }
    }
    if always_equal(a, b) {
        return Some(a.clone());
    }
    both_negated(a, b).map(|(x, y)| {
        let any = Expr::new(ExprKind::Or(x.clone(), y.clone()), e.loc());
        negated(&any, e)
    })
}

fn or(e: &Expr, a: &Expr, b: &Expr, strict: bool) -> Option<Expr> {
    if definitely_falsy(a) {
        return Some(b.clone());
    }
    if definitely_truthy(a) {
        return Some(a.clone());
    }
    if !strict {
        if definitely_falsy(b) {
            return Some(a.clone());
        }
        if definitely_truthy(b) {
            return Some(b.clone());
        }
        if opposite(a, b) {
            return Some(boolean(true, e));
        }
    }
    if always_equal(a, b) {
        return Some(a.clone());
    }
    both_negated(a, b).map(|(x, y)| {
        let all = Expr::new(ExprKind::And(x.clone(), y.clone()), e.loc());
        negated(&all, e)
    })
}

/// The operands of `not A` and `not B`.
fn both_negated<'a>(a: &'a Expr, b: &'a Expr) -> Option<(&'a Expr, &'a Expr)> {
    match (a.kind(), b.kind()) {
        (ExprKind::Not(x), ExprKind::Not(y)) => Some((x, y)),
        _ => None,
    }
}

#[allow(clippy::float_cmp)] // exact comparison is the point
fn compare(e: &Expr, op: CmpOp, lhs: &Expr, rhs: &Expr, strict: bool) -> Option<Expr> {
    if let (Some(x), Some(y)) = (num_of(lhs), num_of(rhs)) {
        let holds = match op {
            CmpOp::Eq => x == y,
            CmpOp::Ne => x != y,
            CmpOp::Lt => x < y,
            CmpOp::Le => x <= y,
            CmpOp::Gt => x > y,
            CmpOp::Ge => x >= y,
        };
        return Some(boolean(holds, e));
    }
    if always_equal(lhs, rhs) {
        return Some(boolean(matches!(op, CmpOp::Eq | CmpOp::Le | CmpOp::Ge), e));
    }
    if matches!(op, CmpOp::Eq | CmpOp::Ne) {
        if is_literal(lhs, strict) && is_literal(rhs, strict) {
            return Some(boolean(op == CmpOp::Ne, e));
        }
        return compare_with_bool(e, op, lhs, rhs).or_else(|| compare_with_bool(e, op, rhs, lhs));
    }
    None
}

/// `x == falsy`, `x == True`, `x != falsy`, `x != True` with `other` as the constant.
fn compare_with_bool(e: &Expr, op: CmpOp, x: &Expr, other: &Expr) -> Option<Expr> {
    let eq = op == CmpOp::Eq;
    if definitely_falsy(other) {
        // `x == falsy` holds exactly when `x` is falsy, whatever its type.
        return if eq {
            Some(negated(x, e))
        } else {
            is_bool(x).then(|| x.clone())
        };
    }
    if is_true(other) && is_bool(x) {
        return Some(if eq { x.clone() } else { negated(x, e) });
    }
    None
}

fn if_then_else(cond: &Expr, then: &Expr, else_: &Expr) -> Option<Expr> {
    if definitely_truthy(cond) {
        return Some(then.clone());
    }
    if definitely_falsy(cond) {
        return Some(else_.clone());
    }
    if always_equal(then, else_) {
        return Some(then.clone());
    }
    let ExprKind::Not(inner) = cond.kind() else {
        return None;
    };
    Some(Expr::new(
        ExprKind::IfThenElse {
            cond: inner.clone(),
            then: else_.clone(),
            else_: then.clone(),
        },
        then.loc(),
    ))
}

#[cfg(test)]
mod tests;
