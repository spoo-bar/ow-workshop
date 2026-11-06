//! Rewrites of statements that do nothing or ignore an argument.

use ow_workshop_data::ActionId;

use super::Env;
use super::build::is_num;
use super::helpers::definitely_falsy;
use crate::ir::{Expr, ExprKind, Stmt, StmtKind};

fn null_like(e: &Expr) -> Expr {
    Expr::new(ExprKind::Null, e.loc())
}

/// Removes no-op actions and nulls arguments the game ignores.
pub(super) fn rule(mut stmt: Stmt, env: &mut Env<'_, '_>) -> Vec<Stmt> {
    let StmtKind::Action { id, args } = &mut stmt.kind else {
        return vec![stmt];
    };
    // Removing a statement moves the distance of every skip over it.
    let removable = !env.dynamic_skips;
    match (*id, args.as_slice()) {
        (ActionId::AddToScore, [_, score]) if removable && is_num(score, 0.0) => return Vec::new(),
        (ActionId::SetStatusEffect, [_, _, _, duration]) if removable && is_num(duration, 0.0) => {
            return Vec::new();
        }
        (ActionId::StartForcingOutlineFor, [_, _, visible, color, _])
            if definitely_falsy(visible) =>
        {
            args[3] = null_like(color);
        }
        _ => {}
    }
    vec![stmt]
}

#[cfg(test)]
mod tests;
