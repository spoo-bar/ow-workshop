//! The private, untyped intermediate representation the passes transform.

// Unused until the passes that consume the IR land.
#![allow(dead_code, unused_imports)]

mod expr;
mod ids;
mod loc;
mod stmt;

pub(crate) use expr::{CmpOp, ConstRef, Expr, ExprKind, StrExpr, StrToken, Translated};
pub(crate) use ids::{LabelId, SubId, VarId};
pub(crate) use loc::Location;
pub(crate) use stmt::{Block, ModifyOp, Place, SkipTarget, Stmt, StmtKind};

#[cfg(test)]
mod tests;
