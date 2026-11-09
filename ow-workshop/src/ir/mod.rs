//! The private, untyped intermediate representation the passes transform.

// Unused until the passes that consume the IR land.
#![allow(dead_code, unused_imports)]

mod diag;
mod expr;
mod ids;
mod loc;
mod program;
mod rule;
mod stmt;
mod vars;

pub use diag::{Diagnostic, Diagnostics, Severity};
pub(crate) use expr::{
    CmpOp, ConstRef, Expr, ExprKind, StrExpr, StrModifier, StrToken, Translated,
};
pub(crate) use ids::{LabelId, SubId, VarId};
pub use loc::Location;
pub(crate) use program::Program;
pub(crate) use rule::{
    Cond, Event, MAX_SLOT, OptOverride, PlayerFilter, PlayerSlot, Rule, RuleError,
};
pub use rule::{TeamFilter, WarningSet};
pub(crate) use stmt::{Block, ModifyOp, Place, SkipTarget, Stmt, StmtKind};
pub(crate) use vars::{
    Allocation, CallGraph, MAX_SLOTS, SubDecl, SubUsage, Subs, VarDecl, VarFlags, Vars,
};

#[cfg(test)]
mod rule_tests;
#[cfg(test)]
mod tests;
