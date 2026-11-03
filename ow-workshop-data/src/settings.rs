//! Custom game settings: sections, settings and their value kinds.
//!
//! Data only; names are stored once in a shared table and referenced by index.

use crate::generated::setting_names::NAMES;
use crate::generated::settings::HERO_SETS;
use crate::{Hero, Lang, Names};

pub use crate::generated::settings::{GAMEMODE_GENERAL, LOBBY, MAIN, gamemode_settings};

/// One option of a [`Kind::Combo`] setting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Choice {
    /// The option's camelCase key.
    pub key: &'static str,
    name: u16,
}

impl Choice {
    pub(crate) const fn new(key: &'static str, name: u16) -> Self {
        Self { key, name }
    }

    /// The option's name in `lang`.
    pub fn name(&self, lang: Lang) -> &'static str {
        Names(NAMES[self.name as usize]).get(lang)
    }
}

/// How a setting's value is written in workshop code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Kind {
    /// `Yes` / `No`.
    BoolYesNo,
    /// `Enabled` / `Disabled`.
    BoolEnabled,
    /// `true` is written `Disabled`.
    BoolReverseEnabled,
    /// `On` / `Off`.
    BoolOnOff,
    /// `true` is written `Off`.
    BoolReverseOnOff,
    /// One of a set of localized choices.
    Combo(&'static [Choice]),
    /// A number printed with `%`.
    Percent,
    /// An integer.
    Int,
    /// A number.
    Float,
    /// An escaped string with a length limit.
    String {
        /// Longest allowed string, in characters.
        max_chars: Option<u32>,
        /// Longest allowed string, in bytes.
        max_bytes: Option<u32>,
    },
}

/// A custom game setting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Setting {
    /// The setting's camelCase key.
    pub key: &'static str,
    /// How the value is written.
    pub kind: Kind,
    name: u16,
    only: Option<u8>,
    except: Option<u8>,
}

impl Setting {
    pub(crate) const fn new(key: &'static str, name: u16, kind: Kind) -> Self {
        Self {
            key,
            kind,
            name,
            only: None,
            except: None,
        }
    }

    /// The setting's name in `lang`.
    pub fn name(&self, lang: Lang) -> &'static str {
        Names(NAMES[self.name as usize]).get(lang)
    }

    /// Whether the setting exists for `hero`.
    pub fn applies_to(&self, hero: Hero) -> bool {
        let listed = |set: u8| HERO_SETS[usize::from(set)].contains(&hero);
        self.only.is_none_or(listed) && !self.except.is_some_and(listed)
    }
}

/// A named group of settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Section {
    /// The section's key.
    pub key: &'static str,
    /// The settings, in order.
    pub settings: &'static [Setting],
    name: u16,
}

impl Section {
    pub(crate) const fn new(key: &'static str, name: u16, settings: &'static [Setting]) -> Self {
        Self {
            key,
            settings,
            name,
        }
    }

    /// The section's name in `lang`.
    pub fn name(&self, lang: Lang) -> &'static str {
        Names(NAMES[self.name as usize]).get(lang)
    }

    /// The setting with this key.
    pub fn get(&self, key: &str) -> Option<&'static Setting> {
        self.settings.iter().find(|s| s.key == key)
    }
}
