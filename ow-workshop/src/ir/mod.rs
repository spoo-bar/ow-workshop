//! The private, untyped intermediate representation the passes transform.

// Unused until the passes that consume the IR land.
#![allow(dead_code, unused_imports)]

mod diag;
mod error_kind;
mod expr;
mod ids;
mod loc;
mod program;
mod rule;
mod stmt;
mod vars;
mod warning_set;

pub use diag::{Diagnostic, Diagnostics, Kind, Severity};
pub use error_kind::ErrorKind;
pub(crate) use expr::{
    CmpOp, ConstRef, Decompress, Expr, ExprKind, StrExpr, StrModifier, StrToken, Translated,
};
pub use ids::VarId;
pub(crate) use ids::{LabelId, SubId};
pub use loc::Location;
pub(crate) use program::Program;
pub use rule::TeamFilter;
pub(crate) use rule::{
    Cond, Event, MAX_SLOT, OptOverride, PlayerFilter, PlayerSlot, Rule, RuleError,
};
pub(crate) use stmt::{Block, ModifyOp, Place, SkipTarget, Stmt, StmtKind};
pub(crate) use vars::{
    Allocation, CallGraph, MAX_SLOTS, SubDecl, SubUsage, Subs, VarDecl, VarFlags, Vars,
};
pub use warning_set::WarningSet;

#[cfg(test)]
mod rule_tests;
#[cfg(test)]
mod tests;
