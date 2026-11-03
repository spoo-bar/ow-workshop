//! Static game data tables for `ow-workshop`.

// The tables are long `match`es, one arm per entry.
#![allow(clippy::too_many_lines, clippy::unreadable_literal)]

// Formatted by the generator (prettyplease), not rustfmt.
#[rustfmt::skip]
pub mod generated;
pub mod lang;
pub mod settings;
pub mod signature;

pub use generated::actions::ActionId;
pub use generated::constants::*;
pub use generated::entities::entity;
pub use generated::extension::*;
pub use generated::fonts::{DEFAULT_WIDTH, Glyph, SPACES, char_width, lowercase_metrics};
pub use generated::gamemodes::*;
pub use generated::heroes::*;
pub use generated::keywords::*;
pub use generated::maps::*;
pub use generated::settings::Label;
pub use generated::strings::LocalizedString;
pub use generated::textures::Texture;
pub use generated::ty::*;
pub use generated::values::ValueId;
pub use lang::{Lang, Names, ParseLangError};
