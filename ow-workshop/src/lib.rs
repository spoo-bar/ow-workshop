//! Write Overwatch Workshop mods in Rust.

// Lets the derives refer to `::ow_workshop` from inside this crate's own tests.
extern crate self as ow_workshop;

mod block;
mod builder;
mod check;
mod control;
mod event;
mod expr;
mod ir;
mod ops;
mod options;
mod output;
mod pipeline;
pub mod prelude;
mod rule;
mod translations;
mod ty;
mod vars;
mod warning;
mod workshop;

pub use block::Block;
pub use builder::Builder;
pub use control::{ForRange, IfBuilder, IntoForRange, RangeStep, range, rule_condition};
pub use event::{
    Ev, EventType, HasDamage, HasDamageOrHealing, HasEventPlayer, HasHealing, OngoingEachPlayer,
    OngoingGlobal, PlayerDealtDamage, PlayerDealtFinalBlow, PlayerDealtHealing,
    PlayerDealtKnockback, PlayerDied, PlayerEarnedElimination, PlayerEvent, PlayerJoinedMatch,
    PlayerLeftMatch, PlayerReceivedHealing, PlayerReceivedKnockback, PlayerTookDamage, RawEvent,
};
pub use expr::{Expr, null, vect};
pub use ir::{Diagnostic, Diagnostics, Location, Settings, Severity, TeamFilter, WarningSet};
pub use ops::{Apply, NumLike, Operand, Ordered, VecLike, if_then_else};
pub use options::{CompileOptions, OptLevel};
pub use output::{Catalog, CompileOutput};
pub use ow_workshop_data::Lang;
pub use ow_workshop_macros::{GlobalVars, PlayerVars};
pub use rule::{ConditionRef, Rule};
pub use translations::Translations;
pub use ty::*;
pub use vars::{
    Global, GlobalHandle, GlobalVars, Indexed, Place, Plain, PlayerHandle, PlayerVar, PlayerVarRef,
    PlayerVars, RawPlace, VarRegistry,
};
pub use warning::Warning;
pub use workshop::{Workshop, WorkshopExt};
