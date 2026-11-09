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
mod array_helpers;
mod array_to_string;
mod block;
mod builder;
mod cased;
mod catalog;
mod check;
mod cli;
pub mod colors;
mod columns;
mod compress;
mod control;
mod count;
mod debug;
mod dict;
mod emit;
mod event;
mod expr;
mod flatten;
mod fmt;
mod generated;
mod hud;
mod ir;
mod label;
mod localized;
mod lower;
pub mod math;
mod ops;
mod opt;
mod options;
mod output;
mod pipeline;
mod players;
pub mod prelude;
mod raycast;
mod rule;
mod rule_name;
mod settings;
mod subroutine;
mod support;
mod suppress;
mod switch;
mod tags;
mod text;
mod translate;
mod translations;
mod ty;
mod vars;
mod warning;
mod width;
mod workshop;
#[cfg(test)]
mod workshop_enum;
pub mod workshop_setting;
mod wrap;

pub use array_to_string::array_to_string;
pub use block::Block;
pub use builder::Builder;
pub use cased::{CasedTextArgs, CasedTextError};
pub use catalog::{Catalog, Entry};
#[doc(hidden)]
pub use cli::lang_from_args;
pub use columns::{ColumnPlace, Columns, Table};
pub use compress::{
    CompressError, Compressible, compress, compressed, decompress_numbers, decompress_vectors,
};
pub use control::{ForRange, IfBuilder, IntoForRange, RangeStep, range, rule_condition};
pub use debug::DebugValue;
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
pub use hud::HudArgs;
pub use ir::{
    Diagnostic, Diagnostics, ErrorKind, Kind, Location, Severity, TeamFilter, WarningSet,
};
pub use label::Label;
pub use localized::{localized, localized_en};
pub use math::{ceil, floor, round};
pub use ops::{Apply, NumLike, Operand, Ordered, VecLike, if_then_else};
pub use options::{CompileOptions, EmptyString, OptLevel, Replacements};
pub use output::CompileOutput;
pub use ow_workshop_data::chars;
pub use ow_workshop_data::{Extension, Gamemode, Lang, LocalizedString, Map, Texture};
pub use ow_workshop_macros::{GlobalVars, PlayerVars, WorkshopEnum, ws_fmt};
pub use players::{
    PlayerHelperError, PlayerHelpers, RealPlayersArgs, RealRadiusArgs, real_closest_player,
    real_closest_player_with, real_closest_players, real_closest_players_with,
    real_farthest_player, real_farthest_player_with, real_farthest_players,
    real_farthest_players_with, real_players_in_radius, real_players_in_radius_with,
};

/// Support items for macro expansions; not public API.
#[doc(hidden)]
pub mod __private {
    pub use crate::fmt::{Piece, template};
    pub use crate::translate::translated;
}
pub use raycast::{RayCast, RayCastArgs, ray_cast, ray_cast_with};
pub use rule::{ConditionRef, Rule};
pub use rule_name::{RuleMeta, default_rule_name};
pub use settings::{
    HeroSelection, HeroSettings, HeroTeam, Lobby, Main, MapRef, MapSelection, ModeKey,
    ModeSettings, SettingValue, Settings, Values, WorkshopValue,
};
pub use subroutine::{Sub, SubroutineOptions};
pub use switch::SwitchBuilder;
pub use text::{button_to_string, time_to_string};
pub use translate::{tr, tr_raw, tr_spectator};
pub use translations::{ParseTranslationLangError, PlayerVarMode, TranslationLang, Translations};
pub use ty::*;
pub use vars::{
    Global, GlobalHandle, GlobalVars, Indexed, Indexed2, Indexed3, Place, Plain, PlayerHandle,
    PlayerVar, PlayerVarRef, PlayerVars, RawPlace, VarRegistry,
};
pub use warning::Warning;
pub use width::{TextWidthError, spaces_for, spaces_for_str, visual_len};
pub use workshop::{Workshop, WorkshopExt};
