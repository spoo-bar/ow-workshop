//! Write Overwatch Workshop mods in Rust.
//!
//! A mod is one type implementing [`Workshop`]. Its rules are values of types
//! implementing [`Rule`], which can live in any module or crate. A two-line
//! `build.rs` calling [`build::emit`] writes the script to
//! `target/<profile>/<name>.owmod` on every `cargo build`.

mod action;
pub mod build;
mod debug;
mod emit;
mod error;
mod event;
mod rule;
mod script;
mod settings;
#[cfg(feature = "utils")]
pub mod utils;
mod value;
mod vars;
mod workshop;

pub use action::{Action, Chase};
pub use debug::{DebugOpts, HudLocation, WorkshopDebug};
pub use error::{BuildError, UndeclaredSubroutine};
pub use event::{Event, PlayerEvent, PlayerFilter, TeamFilter};
pub use ow_workshop_data as data;
pub use ow_workshop_data::{
    ActionId, ChaseRateReeval, ChaseTimeReeval, CompareOp, Extension, ModifyOp, StartRuleBehavior,
    ValueId,
};
pub use ow_workshop_macros::{Globals, Players, WorkshopDebug};
pub use rule::{BasicRule, BasicSubroutine, Condition, Rule, Subroutine};
pub use script::{Profile, script, try_script};
pub use settings::{
    Entry, Hero, HeroEntry, HeroSettings, Lobby, LobbySettings, Mode, ModeSettings, SettingValue,
    SettingsConflict, SettingsError, UnknownSetting,
};
pub use value::Value;
pub use vars::{Global, Globals, PlayerVar, Players};
pub use workshop::Workshop;
