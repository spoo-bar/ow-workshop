//! Works around two editor quirks, before counting and printing so both see the same IR.
//!
//! - Some arguments reject number literals above a maximum (`literal_max`); wrapping the literal
//!   in `Absolute Value(..)` makes it a non-literal.
//! - Some values (`can_be_put_in_boolean = false`) are rejected where a boolean is expected;
//!   `First Of(..)` keeps their truthiness.

use ow_workshop_data::signature::{Arg, Signature};
use ow_workshop_data::{Ty, ValueId};

use crate::ir::{Diagnostics, Expr, ExprKind, Program, Stmt, StmtKind};
use crate::lower::Ctx;
use crate::lower::visit::{blocks_mut, rewrite_expr, rewrite_stmt_exprs};

/// The pass: wraps literals over their limit and values in boolean slots that reject them.
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn run(program: &mut Program, _: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    for rule in &mut program.rules {
        for cond in &mut rule.conditions {
            let expr = rewrite_expr(&cond.expr, &mut wrap_node);
            // A condition printed as `x == True` is a boolean slot too.
            cond.expr = if is_boolean_value(&expr) {
                bool_slot(expr)
            } else {
                expr
            };
        }
        wrap_block(&mut rule.body);
    }
    Ok(())
}

fn wrap_block(block: &mut crate::ir::Block) {
    for stmt in &mut block.stmts {
        wrap_stmt(stmt);
        for child in blocks_mut(stmt) {
            wrap_block(child);
        }
    }
}

fn wrap_stmt(stmt: &mut Stmt) {
    rewrite_stmt_exprs(stmt, &mut wrap_node);
    match &mut stmt.kind {
        StmtKind::Action { id, args } => {
            let signature = id.signature();
            wrap_args(signature, args);
        }
        StmtKind::If { cond, elifs, .. } => {
            *cond = bool_slot(cond.clone());
            for (cond, _) in elifs {
                *cond = bool_slot(cond.clone());
            }
        }
        StmtKind::While { cond, .. }
        | StmtKind::DoWhile { cond, .. }
        | StmtKind::AbortIf(cond)
        | StmtKind::LoopIf(cond)
        | StmtKind::SkipIf(cond, _) => *cond = bool_slot(cond.clone()),
        _ => {}
    }
}

/// Wraps the children of one node according to the slots they fill.
fn wrap_node(expr: Expr) -> Expr {
    let kind = match expr.kind() {
        ExprKind::Value { id, args } => {
            let mut args = args.clone();
            wrap_args(id.signature(), &mut args);
            ExprKind::Value { id: *id, args }
        }
        ExprKind::And(a, b) => ExprKind::And(bool_slot(a.clone()), bool_slot(b.clone())),
        ExprKind::Or(a, b) => ExprKind::Or(bool_slot(a.clone()), bool_slot(b.clone())),
        ExprKind::Not(a) => ExprKind::Not(bool_slot(a.clone())),
        ExprKind::IfThenElse { cond, then, else_ } => ExprKind::IfThenElse {
            cond: bool_slot(cond.clone()),
            then: then.clone(),
            else_: else_.clone(),
        },
        _ => return expr,
    };
    Expr::new(kind, expr.loc())
}

fn wrap_args(signature: &Signature, args: &mut [Expr]) {
    for (arg, slot) in args.iter_mut().zip(signature.args) {
        if signature.flags.has_literal_limit() {
            *arg = limit_literal(slot, arg.clone());
        }
        if slot.ty == Ty::Bool {
            *arg = bool_slot(arg.clone());
        }
    }
}

/// `Absolute Value(n)` for a number literal above the argument's maximum.
fn limit_literal(slot: &Arg, expr: Expr) -> Expr {
    match (slot.literal_max, expr.kind()) {
        (Some(max), ExprKind::Num(n)) if max > 0.0 && *n > max => call(ValueId::Abs, expr),
        _ => expr,
    }
}

/// `First Of(x)` for a value the game rejects in a boolean slot.
fn bool_slot(expr: Expr) -> Expr {
    match expr.kind() {
        ExprKind::Value { id, .. } if !id.signature().flags.can_be_put_in_boolean() => {
            call(ValueId::FirstOf, expr)
        }
        _ => expr,
    }
}

/// Whether the condition is a value that returns a boolean, which prints as `x == True`.
fn is_boolean_value(expr: &Expr) -> bool {
    matches!(expr.kind(), ExprKind::Value { id, .. } if id.signature().ret == Ty::Bool)
}

fn call(id: ValueId, arg: Expr) -> Expr {
    let loc = arg.loc();
    Expr::new(
        ExprKind::Value {
            id,
            args: vec![arg],
        },
        loc,
    )
}

#[cfg(test)]
mod tests;
