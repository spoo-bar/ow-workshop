//! Simplification of a rule's conditions.

use crate::ir::{Cond, ExprKind};

use super::helpers::{definitely_falsy, definitely_truthy};

/// Splits top-level `And`s, drops conditions that always hold, and reports `None` when one can
/// never hold (the rule never runs).
pub(super) fn simplify(conditions: &[Cond]) -> Option<Vec<Cond>> {
    let mut out = Vec::new();
    for cond in conditions {
        let mut parts = Vec::new();
        split(cond, &mut parts);
        for part in parts {
            if definitely_falsy(&part.expr) {
                return None;
            }
            if !definitely_truthy(&part.expr) {
                out.push(part);
            }
        }
    }
    Some(out)
}

/// Pushes the conjuncts of `cond`; the first keeps the comment.
fn split(cond: &Cond, out: &mut Vec<Cond>) {
    let ExprKind::And(a, b) = cond.expr.kind() else {
        out.push(cond.clone());
        return;
    };
    let first = out.len();
    for side in [a, b] {
        split(
            &Cond {
                expr: side.clone(),
                comment: None,
            },
            out,
        );
    }
    if let Some(head) = out.get_mut(first) {
        head.comment.clone_from(&cond.comment);
    }
}

#[cfg(test)]
mod tests;
