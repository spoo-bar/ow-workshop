//! Write Overwatch Workshop mods in Rust.

// Lets the derives refer to `::ow_workshop` from inside this crate's own tests.
extern crate self as ow_workshop;

mod array;
mod block;
mod builder;
mod check;
mod control;
mod dict;
mod event;
mod expr;
mod generated;
mod ir;
mod label;
mod math;
mod ops;
mod options;
mod output;
mod pipeline;
pub mod prelude;
mod raycast;
mod rule;
mod subroutine;
mod support;
mod switch;
mod translations;
mod ty;
mod vars;
mod warning;
mod workshop;
#[cfg(test)]
mod workshop_enum;

pub use block::Block;
pub use builder::Builder;
pub use control::{ForRange, IfBuilder, IntoForRange, RangeStep, range, rule_condition};
pub use dict::{Dict, dict};
pub use event::{
    Ev, EventType, HasDamage, HasDamageOrHealing, HasEventPlayer, HasHealing, OngoingEachPlayer,
    OngoingGlobal, PlayerDealtDamage, PlayerDealtFinalBlow, PlayerDealtHealing,
    PlayerDealtKnockback, PlayerDied, PlayerEarnedElimination, PlayerEvent, PlayerJoinedMatch,
    PlayerLeftMatch, PlayerReceivedHealing, PlayerReceivedKnockback, PlayerTookDamage, RawEvent,
    SubroutineEvent,
};
pub use expr::{Expr, null, vect};
pub use generated::api::*;
pub use ir::{Diagnostic, Diagnostics, Location, Settings, Severity, TeamFilter, WarningSet};
pub use label::Label;
pub use math::{ceil, floor, round};
pub use ops::{Apply, NumLike, Operand, Ordered, VecLike, if_then_else};
pub use options::{CompileOptions, OptLevel};
pub use output::{Catalog, CompileOutput};
pub use ow_workshop_data::Lang;
pub use ow_workshop_macros::{GlobalVars, PlayerVars, WorkshopEnum};
pub use raycast::{RayCast, RayCastArgs, ray_cast, ray_cast_with};
pub use rule::{ConditionRef, Rule};
pub use subroutine::{Sub, SubroutineOptions};
pub use switch::SwitchBuilder;
pub use translations::Translations;
pub use ty::*;
pub use vars::{
    Global, GlobalHandle, GlobalVars, Indexed, Indexed2, Indexed3, Place, Plain, PlayerHandle,
    PlayerVar, PlayerVarRef, PlayerVars, RawPlace, VarRegistry,
};
pub use warning::Warning;
pub use workshop::{Workshop, WorkshopExt};
