//! Write Overwatch Workshop mods in Rust.
//!
//! Implement [`Workshop`] for a type and call [`WorkshopExt::compile`]; the core does no I/O.
//! [`main!`] defines a native command-line `main` for it.
//!
//! # Quick start
//!
//! ```
//! use ow_workshop::prelude::*;
//! use ow_workshop::{Gamemode, Map};
//!
//! struct HelloWorld;
//!
//! impl Workshop for HelloWorld {
//!     fn mode_name(&self) -> &'static str {
//!         "My Mode"
//!     }
//!
//!     fn description(&self) -> &'static str {
//!         "Made with ow-workshop"
//!     }
//!
//!     type Globals = ();
//!     type Players = ();
//!
//!     fn build(&self, ws: &mut Builder, (): &(), (): &()) {
//!         ws.rule("Header", OngoingGlobal, |r, _| {
//!             r.actions(|b| b.hud_header("Hello, Workshop!"));
//!         });
//!
//!         ws.rule("Welcome", OngoingEachPlayer, |r, ev| {
//!             r.condition(ev.event_player().has_spawned());
//!             r.actions(|b| b.small_message(ws_fmt!("Welcome, {}!", ev.event_player())));
//!         });
//!     }
//!
//!     fn settings(&self) -> Settings {
//!         let maps = MapSelection::Enabled(vec![Map::WorkshopChamber.into()]);
//!         Settings::default()
//!             .with_mode(
//!                 ModeKey::Mode(Gamemode::TeamDeathmatch),
//!                 ModeSettings::default().maps(maps),
//!             )
//!     }
//! }
//!
//! let output = HelloWorld.compile(Lang::EnUs).expect("compiles");
//! assert!(output.code.contains("Custom String(\"Hello, Workshop!\")"));
//! ```
//!
//! # Pasting into the game
//!
//! Print `output.code` (or use [`main!`] and `cargo run`), copy it, then in *Custom Game →
//! Settings → Workshop* choose *Paste Rules*.
//!
//! # Concepts and their API
//!
//! | Workshop concept | Rust API |
//! |---|---|
//! | Rules and events | [`Builder::rule`], event types such as [`OngoingGlobal`] and [`PlayerDied`] |
//! | Conditions | [`Rule::condition`], comparisons like [`Expr::eq`] |
//! | Variables | `#[derive(GlobalVars)]`, `#[derive(PlayerVars)]`, [`Global`], [`PlayerVar`] |
//! | Subroutines | [`Builder::declare_subroutine`], [`Builder::define`], [`Block::call`] |
//! | Skip and labels | [`Label`], [`Block::switch`], [`Block::break_`] |
//! | Strings | [`ws_fmt!`](ws_fmt), [`CasedTextArgs`], [`spaces_for`] |
//! | Settings | [`Workshop::settings`], [`Settings`], [`workshop_setting`] |
//! | Translations | [`Workshop::translations`], [`t!`], [`Translations`] |
//!
//! The `examples` directory has runnable mods: `hello_world`, `zombie_survival` and
//! `custom_settings`.
//!
//! # Debugging
//!
//! A debug build adds debug information to the mod without you changing it. The Cargo profile of
//! *your* binary decides: `cargo run` is a debug build, `cargo run --release` exports the
//! workshop code as is. (`main!` also takes `--debug` and `--no-debug` to force either, and
//! [`CompileOptions::debug`] sets a [`DebugMode`] for a profile that does not fit.)
//!
//! A debug build
//!
//! * starts the output with the comment line `// ow-workshop debug build`, so a pasted debug
//!   build is recognizable;
//! * shows a live panel of your variables on screen: the rules `Debug: globals` and
//!   `Debug: players` print one HUD text per variable of your `Globals` and `Players`, which
//!   the derives describe with [`WorkshopDebug`]. If they would push the mod over the element
//!   limit they shrink, with a `debug-budget` warning, and never push over a mod that fits in
//!   release;
//! * prints a log on **stderr** (variables, subroutines, rules with their element counts,
//!   totals and warnings), before the warnings. The code stays on stdout, so
//!   `cargo run > mod.txt` still captures just the code. The log is also
//!   [`CompileOutput::debug_log`];
//! * keeps the calls that only exist for debugging: [`debug!`], [`Block::print`] and anything
//!   inside [`Block::debug_only`]. A release build drops them before labels are resolved, so
//!   jumps keep their distances.
//!
//! Control how a field shows with attributes on the variable structs: `#[ow(debug = false)]`
//! hides it, `#[ow(debug_label = "Lives left")]` renames it (the default is the field name) and
//! `#[ow(debug_with = path::to_fn)]` renders it with your own `fn(Expr<T>) -> Expr<Str>`. Enums
//! derived with [`WorkshopEnum`] render their variant names:
//!
//! ```
//! use ow_workshop::prelude::*;
//! use ow_workshop::{Gamemode, Map};
//!
//! #[derive(Clone, Copy, WorkshopEnum)]
//! enum State {
//!     Lobby,
//!     Playing,
//! }
//!
//! #[derive(GlobalVars)]
//! struct Globals {
//!     #[ow(debug_with = State::debug_render)]
//!     state: Global<Num>,
//!     #[ow(debug_label = "Boss HP")]
//!     boss_health: Global<Num>,
//!     #[ow(debug = false)]
//!     scratch: Global<Num>,
//! }
//!
//! struct Mode;
//!
//! impl Workshop for Mode {
//!     fn mode_name(&self) -> &'static str {
//!         "My Mode"
//!     }
//!
//!     fn description(&self) -> &'static str {
//!         "Made with ow-workshop"
//!     }
//!
//!     type Globals = Globals;
//!     type Players = ();
//!
//!     fn build(&self, ws: &mut Builder, g: &Globals, (): &()) {
//!         ws.rule("Run", OngoingGlobal, |r, _| {
//!             r.actions(|b| {
//!                 b.set(g.state, State::Playing);
//!                 b.set(g.boss_health, 100);
//!                 b.set(g.scratch, 0);
//!                 // Only in a debug build.
//!                 b.print("started");
//!                 b.debug_only(|b| b.set(g.boss_health, 1));
//!             });
//!         });
//!     }
//!
//!     fn settings(&self) -> Settings {
//!         let maps = MapSelection::Enabled(vec![Map::WorkshopChamber.into()]);
//!         Settings::default()
//!             .with_mode(
//!                 ModeKey::Mode(Gamemode::TeamDeathmatch),
//!                 ModeSettings::default().maps(maps),
//!             )
//!     }
//! }
//!
//! let debug = Mode
//!     .compile_with_debug(Lang::EnUs, Some(DebugMode::On(DebugOptions::default())))
//!     .expect("compiles");
//! assert!(debug.code.starts_with("// ow-workshop debug build"));
//! assert!(debug.code.contains("Debug: globals"));
//! assert!(debug.code.contains("Boss HP: {0}"));
//! assert!(!debug.code.contains("scratch: {0}"));
//! assert!(debug.debug_log.is_some());
//!
//! let release = Mode
//!     .compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
//!     .expect("compiles");
//! assert!(!release.code.contains("Debug"));
//! assert!(!release.code.contains("started"));
//! assert!(release.debug_log.is_none());
//! ```
//!
//! # Using it from WASM
//!
//! `ow-workshop` has no `wasm-bindgen` dependency. Export a function from your own crate, which
//! depends on `wasm-bindgen`:
//!
//! ```
//! use ow_workshop::prelude::*;
//! use ow_workshop::{Gamemode, Map};
//!
//! struct MyMode;
//!
//! impl Workshop for MyMode {
//!     fn mode_name(&self) -> &'static str {
//!         "My Mode"
//!     }
//!
//!     fn description(&self) -> &'static str {
//!         "Made with ow-workshop"
//!     }
//!
//!     type Globals = ();
//!     type Players = ();
//!
//!     fn build(&self, ws: &mut Builder, _: &(), _: &()) {
//!         ws.rule("Init", OngoingGlobal, |_, _| {});
//!     }
//!
//!     fn settings(&self) -> Settings {
//!         let maps = MapSelection::Enabled(vec![Map::WorkshopChamber.into()]);
//!         Settings::default()
//!             .with_mode(
//!                 ModeKey::Mode(Gamemode::TeamDeathmatch),
//!                 ModeSettings::default().maps(maps),
//!             )
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
//! assert!(compile("en-US").is_ok());
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
mod current_map;
mod debug;
mod debug_log;
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
mod workshop_debug;
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
pub use cli::{args_from, lang_from_args};
pub use columns::{ColumnPlace, Columns, Table};
pub use compress::{
    CompressError, Compressible, compress, compressed, decompress_numbers, decompress_vectors,
};
pub use control::{ForRange, IfBuilder, IntoForRange, RangeStep, range, rule_condition};
pub use debug::DebugValue;
pub use debug_log::{DebugLog, DebugRule, DebugSubroutine, DebugTotals, DebugVariable};
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
    Diagnostic, Diagnostics, ErrorKind, Kind, Location, Severity, TeamFilter, VarId, WarningSet,
};
pub use label::Label;
pub use localized::{localized, localized_en};
pub use math::{ceil, floor, round};
pub use ops::{Apply, NumLike, Operand, Ordered, VecLike, if_then_else};
pub use options::{CompileOptions, DebugMode, DebugOptions, EmptyString, OptLevel, Replacements};
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
    pub use crate::workshop_debug::{debug_field, debug_field_with};
}
pub use raycast::{RayCast, RayCastArgs, ray_cast, ray_cast_with};
pub use rule::{ConditionRef, Rule};
pub use rule_name::{RuleMeta, default_rule_name};
pub use settings::{
    HeroSelection, HeroSettings, HeroTeam, Lobby, MapRef, MapSelection, ModeKey, ModeSettings,
    SettingValue, Settings, Values, WorkshopValue,
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
pub use workshop_debug::{DebugField, DebugHandle, DebugRender, Scope, WorkshopDebug};
