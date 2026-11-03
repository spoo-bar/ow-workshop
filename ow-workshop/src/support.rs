//! Helpers the generated API builds expressions with.

use ow_workshop_data::ValueId;

use crate::expr::Expr;
use crate::ir::{self, ConstRef, ExprKind};

/// A call of a workshop value, located at the caller.
#[track_caller]
pub(crate) fn value<R>(id: ValueId, args: Vec<ir::Expr>) -> Expr<R> {
    Expr::from_ir(ir::Expr::here(ExprKind::Value { id, args }))
}

/// A call of a workshop value inside a default argument.
#[allow(dead_code)] // used by the defaults of generated actions
pub(crate) fn value_ir(id: ValueId, args: Vec<ir::Expr>) -> ir::Expr {
    ir::Expr::new(ExprKind::Value { id, args }, None)
}

/// A number literal.
pub(crate) fn num(n: f64) -> ir::Expr {
    ir::Expr::new(ExprKind::Num(n), None)
}

/// A boolean literal.
#[allow(dead_code)] // used by the defaults of generated actions
pub(crate) fn boolean(b: bool) -> ir::Expr {
    ir::Expr::new(ExprKind::Bool(b), None)
}

/// `Null`.
#[allow(dead_code)] // used by the defaults of generated actions
pub(crate) fn null() -> ir::Expr {
    ir::Expr::new(ExprKind::Null, None)
}

/// A constant enum member.
pub(crate) fn constant(enum_name: &'static str, key: &'static str) -> ir::Expr {
    ir::Expr::new(ExprKind::Const(ConstRef { enum_name, key }), None)
}

/// A vector literal.
#[allow(dead_code)] // used by the defaults of generated actions
pub(crate) fn vector([x, y, z]: [f64; 3]) -> ir::Expr {
    ir::Expr::new(ExprKind::Vector([num(x), num(y), num(z)]), None)
}
