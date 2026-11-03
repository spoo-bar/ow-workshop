//! The items a mod needs: `use ow_workshop::prelude::*;`.

pub use crate::block::Block;
pub use crate::builder::Builder;
pub use crate::control::{ForRange, IfBuilder, RangeStep, range, rule_condition};
pub use crate::event::*;
pub use crate::expr::{Expr, null, vect};
pub use crate::ops::if_then_else;
pub use crate::options::{CompileOptions, OptLevel};
pub use crate::output::CompileOutput;
pub use crate::rule::{ConditionRef, Rule};
pub use crate::switch::SwitchBuilder;
pub use crate::translations::Translations;
pub use crate::ty::*;
pub use crate::vars::{
    Global, GlobalVars, Indexed, Place, PlayerVar, PlayerVarRef, PlayerVars, VarRegistry,
};
pub use crate::workshop::{Workshop, WorkshopExt};
pub use crate::{Diagnostic, Diagnostics, Lang, Settings, TeamFilter, Warning};
pub use ow_workshop_macros::{GlobalVars, PlayerVars};
