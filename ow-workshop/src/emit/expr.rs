//! Expression and statement printing.
//!
//! Only the general `Name(arg, ...)` form and literals so far; the special forms follow.

use core::fmt::{self, Write};

use ow_workshop_data::ValueId;

use super::{Cx, number::write_number};
use crate::ir::{Expr, ExprKind, Stmt, StmtKind};

/// Writes an expression.
pub(super) fn write_expr(out: &mut impl Write, cx: Cx<'_>, expr: &Expr) -> fmt::Result {
    match expr.kind() {
        ExprKind::Num(n) => write_number(out, *n),
        ExprKind::Bool(b) => out.write_str(truth(cx, *b)),
        ExprKind::Null => out.write_str(ValueId::Null.name(cx.lang)),
        ExprKind::Value { id, args } => {
            out.write_str(id.name(cx.lang))?;
            write_args(out, cx, args)
        }
        _ => Err(fmt::Error),
    }
}

/// The localized `True` or `False`.
pub(super) fn truth(cx: Cx<'_>, value: bool) -> &'static str {
    if value { ValueId::True } else { ValueId::False }.name(cx.lang)
}

/// Writes `(a, b, c)`, or nothing for no arguments.
fn write_args(out: &mut impl Write, cx: Cx<'_>, args: &[Expr]) -> fmt::Result {
    if args.is_empty() {
        return Ok(());
    }
    out.write_char('(')?;
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            out.write_str(", ")?;
        }
        write_expr(out, cx, arg)?;
    }
    out.write_char(')')
}

/// Writes a statement without its `;`.
pub(super) fn write_stmt(out: &mut impl Write, cx: Cx<'_>, stmt: &Stmt) -> fmt::Result {
    match &stmt.kind {
        StmtKind::Action { id, args } => {
            out.write_str(id.name(cx.lang))?;
            write_args(out, cx, args)
        }
        _ => Err(fmt::Error),
    }
}
