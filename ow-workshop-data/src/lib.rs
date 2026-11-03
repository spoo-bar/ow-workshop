//! Static game data tables for `ow-workshop`.

// The tables are long `match`es, one arm per entry.
#![allow(clippy::too_many_lines)]

// Formatted by the generator (prettyplease), not rustfmt.
#[rustfmt::skip]
pub mod generated;

pub use generated::constants::*;
pub use generated::extension::*;
pub use generated::gamemodes::*;
pub use generated::heroes::*;
pub use generated::maps::*;
