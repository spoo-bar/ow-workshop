//! Write Overwatch Workshop mods in Rust.
//!
//! Implement [`Workshop`] for a type and call [`WorkshopExt::compile`]; the core does no I/O.
//! [`main!`] defines a native command-line `main` for it.
//!
//! # Using it from WASM
//!
//! `ow-workshop` has no `wasm-bindgen` dependency. Export a function from your own crate, which
//! depends on `wasm-bindgen`:
//!
//! ```
//! use ow_workshop::prelude::*;
//!
//! struct MyMode;
//!
//! impl Workshop for MyMode {
//!     type Globals = ();
//!     type Players = ();
//!
//!     fn build(&self, ws: &mut Builder, _: &(), _: &()) {
//!         ws.rule("Init", OngoingGlobal, |_, _| {});
//!     }
//! }
//!
//! // #[wasm_bindgen::prelude::wasm_bindgen]
//! pub fn compile(lang: &str) -> Result<String, String> {
//!     let lang: Lang = lang.parse().map_err(|e| format!("{e}"))?;
//!     MyMode
//!         .compile(lang)
//!         .map(|output| output.code)
//!         .map_err(|diagnostics| diagnostics.to_string())
//! }
//!
//! assert_eq!(compile("en-US"), Ok(String::new()));
//! assert!(compile("xx-XX").is_err());
//! ```

// Lets the derives refer to `::ow_workshop` from inside this crate's own tests.
extern crate self as ow_workshop;

mod array;
mod block;
mod builder;
mod check;
mod cli;
mod control;
mod count;
mod dict;
mod emit;
mod event;
mod expr;
mod flatten;
mod generated;
mod ir;
mod label;
mod lower;
mod math;
mod ops;
mod opt;
mod options;
mod output;
mod pipeline;
pub mod prelude;
mod raycast;
mod rule;
mod rule_name;
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
#[doc(hidden)]
pub use cli::lang_from_args;
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
pub use options::{CompileOptions, EmptyString, OptLevel, Replacements};
pub use output::{Catalog, CompileOutput};
pub use ow_workshop_data::Lang;
pub use ow_workshop_macros::{GlobalVars, PlayerVars, WorkshopEnum};
pub use raycast::{RayCast, RayCastArgs, ray_cast, ray_cast_with};
pub use rule::{ConditionRef, Rule};
pub use rule_name::{RuleMeta, default_rule_name};
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
