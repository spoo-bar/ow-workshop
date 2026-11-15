//! Custom game settings, shareable fragments, and the merge that resolves them.
//!
//! [`Lobby`], [`Mode`] and [`Hero`] collect settings. Each accepts shared
//! fragments through `.with(..)`: any type implementing [`LobbySettings`],
//! [`ModeSettings`] or [`HeroSettings`]. Merging happens once per scope when
//! the script is written; two sources setting the same key to different values
//! is a [`SettingsConflict`].

#![allow(clippy::result_large_err, clippy::needless_pass_by_value)]

use std::any::type_name;
use std::error::Error;
use std::fmt::{self, Display};

use ow_workshop_data::settings::{
    GAMEMODE_GENERAL, HERO_GENERAL, LOBBY, MAIN, Section, gamemode_settings, hero_settings,
};
use ow_workshop_data::{Gamemode, Hero as DataHero};

/// One `Key: Value` line of a settings block.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub(crate) key: String,
    pub(crate) value: String,
    /// Who set it, for conflict messages.
    pub(crate) source: String,
}

impl Entry {
    /// `Entry::new("Perk Generation", SettingValue::percent(175))`.
    pub fn new(key: impl Display, value: impl Display) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
            source: String::new(),
        }
    }

    fn from_source(key: &str, value: impl Display, source: &str) -> Self {
        Self {
            source: source.to_owned(),
            ..Self::new(key, value)
        }
    }

    fn sourced(mut self, source: &str) -> Self {
        if self.source.is_empty() {
            source.clone_into(&mut self.source);
        }
        self
    }
}

/// A setting of a hero scope: for every hero, or for one named hero.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeroEntry {
    pub(crate) hero: Option<String>,
    pub(crate) entry: Entry,
}

impl HeroEntry {
    /// A setting for every hero in the scope.
    pub fn all(key: impl Display, value: impl Display) -> Self {
        Self {
            hero: None,
            entry: Entry::new(key, value),
        }
    }

    /// A setting for a single hero.
    pub fn of(hero: impl Display, key: impl Display, value: impl Display) -> Self {
        Self {
            hero: Some(hero.to_string()),
            entry: Entry::new(key, value),
        }
    }
}

/// A reusable group of lobby settings.
pub trait LobbySettings {
    /// The settings to add.
    fn entries(&self) -> Vec<Entry>;
}

/// A reusable group of settings for one game mode.
pub trait ModeSettings {
    /// The settings to add.
    fn entries(&self) -> Vec<Entry>;
}

/// A reusable group of settings for one hero scope.
pub trait HeroSettings {
    /// The settings to add.
    fn entries(&self) -> Vec<HeroEntry>;
}

/// A setting value in the form workshop code expects.
#[derive(Clone, Debug, PartialEq)]
pub struct SettingValue(String);

impl SettingValue {
    /// `On` / `Off`.
    pub fn on_off(on: bool) -> Self {
        Self(if on { "On" } else { "Off" }.to_owned())
    }

    /// `Yes` / `No`.
    pub fn yes_no(yes: bool) -> Self {
        Self(if yes { "Yes" } else { "No" }.to_owned())
    }

    /// `Enabled` / `Disabled`.
    pub fn enabled(enabled: bool) -> Self {
        Self(if enabled { "Enabled" } else { "Disabled" }.to_owned())
    }

    /// A percentage, `175` is written `175%`.
    pub fn percent(value: impl Into<f64>) -> Self {
        Self(format!("{}%", value.into()))
    }

    /// An integer.
    pub fn int(value: i64) -> Self {
        Self(value.to_string())
    }

    /// A decimal, `5.0` is written `5.0`.
    pub fn decimal(value: f64) -> Self {
        Self(format!("{value:?}"))
    }

    /// A free-text choice, for example `2 Per Team`.
    pub fn choice(text: impl Display) -> Self {
        Self(text.to_string())
    }
}

impl Display for SettingValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Two sources set the same key of one scope to different values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SettingsConflict {
    /// Where the key lives, for example `modes > Clash`.
    pub scope: String,
    /// The setting's name.
    pub key: String,
    /// The first source and its value.
    pub first: (String, String),
    /// The second source and its value.
    pub second: (String, String),
}

impl Display for SettingsConflict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "conflicting settings in {}: `{}` is `{}` from {} but `{}` from {}",
            self.scope, self.key, self.first.1, self.first.0, self.second.1, self.second.0
        )
    }
}

impl Error for SettingsConflict {}

/// A setting name or scope that `ow-workshop-data` does not know.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnknownSetting {
    /// Where the setting was written, for example `modes > Clash`.
    pub scope: String,
    /// The name that was not found. A scope name when the mode or hero itself is unknown.
    pub name: String,
}

impl Display for UnknownSetting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown setting `{}` in {}", self.name, self.scope)
    }
}

impl Error for UnknownSetting {}

/// Why the settings could not be resolved.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SettingsError {
    /// Two sources disagree.
    Conflict(SettingsConflict),
    /// A name is not in the game data.
    Unknown(UnknownSetting),
}

impl Display for SettingsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Conflict(error) => error.fmt(f),
            Self::Unknown(error) => error.fmt(f),
        }
    }
}

impl Error for SettingsError {}

impl From<SettingsConflict> for SettingsError {
    fn from(error: SettingsConflict) -> Self {
        Self::Conflict(error)
    }
}

impl From<UnknownSetting> for SettingsError {
    fn from(error: UnknownSetting) -> Self {
        Self::Unknown(error)
    }
}

/// The `lobby` settings block.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Lobby {
    entries: Vec<Entry>,
}

impl Lobby {
    /// A lobby with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets one setting, for example `set("Max Team 1 Players", 5)`.
    #[must_use]
    pub fn set(mut self, key: &str, value: impl Display) -> Self {
        self.entries
            .push(Entry::from_source(key, value, "Lobby::set"));
        self
    }

    /// Adds a shared group of settings.
    #[must_use]
    pub fn with<F: LobbySettings>(mut self, fragment: F) -> Self {
        let source = type_name::<F>();
        self.entries
            .extend(fragment.entries().into_iter().map(|e| e.sourced(source)));
        self
    }
}

impl LobbySettings for Lobby {
    fn entries(&self) -> Vec<Entry> {
        self.entries.clone()
    }
}

/// One game mode in the `modes` settings block.
#[derive(Clone, Debug, PartialEq)]
pub struct Mode {
    name: String,
    enabled: Option<bool>,
    entries: Vec<Entry>,
    enabled_maps: Vec<String>,
    disabled_maps: Vec<String>,
}

impl Mode {
    /// A mode, for example `Mode::new("Team Deathmatch")`. `"General"` holds settings for all modes.
    pub fn new(name: impl Display) -> Self {
        Self {
            name: name.to_string(),
            enabled: None,
            entries: Vec::new(),
            enabled_maps: Vec::new(),
            disabled_maps: Vec::new(),
        }
    }

    /// Turns the mode off.
    #[must_use]
    pub fn disabled(mut self) -> Self {
        self.enabled = Some(false);
        self
    }

    /// Turns the mode on explicitly. Modes are on unless disabled.
    #[must_use]
    pub fn enabled(mut self) -> Self {
        self.enabled = Some(true);
        self
    }

    /// Sets one setting of this mode.
    #[must_use]
    pub fn set(mut self, key: &str, value: impl Display) -> Self {
        let source = format!("Mode::set ({})", self.name);
        self.entries.push(Entry::from_source(key, value, &source));
        self
    }

    /// Adds a shared group of settings.
    #[must_use]
    pub fn with<F: ModeSettings>(mut self, fragment: F) -> Self {
        let source = type_name::<F>();
        self.entries
            .extend(fragment.entries().into_iter().map(|e| e.sourced(source)));
        self
    }

    /// Allows only these maps.
    #[must_use]
    pub fn enabled_maps(mut self, maps: impl IntoIterator<Item = impl Display>) -> Self {
        self.enabled_maps.extend(names(maps));
        self
    }

    /// Allows only this map, limited to the given variant ids.
    #[must_use]
    pub fn enabled_map(
        mut self,
        map: impl Display,
        variants: impl IntoIterator<Item = impl Display>,
    ) -> Self {
        self.enabled_maps.push(map_line(&map, variants));
        self
    }

    /// Allows every map except these.
    #[must_use]
    pub fn disabled_maps(mut self, maps: impl IntoIterator<Item = impl Display>) -> Self {
        self.disabled_maps.extend(names(maps));
        self
    }

    /// Disables this map, limited to the given variant ids.
    #[must_use]
    pub fn disabled_map(
        mut self,
        map: impl Display,
        variants: impl IntoIterator<Item = impl Display>,
    ) -> Self {
        self.disabled_maps.push(map_line(&map, variants));
        self
    }
}

impl ModeSettings for Mode {
    fn entries(&self) -> Vec<Entry> {
        self.entries.clone()
    }
}

/// One scope in the `heroes` settings block: `General`, `Team 1` or `Team 2`.
#[derive(Clone, Debug, PartialEq)]
pub struct Hero {
    scope: String,
    entries: Vec<HeroEntry>,
    enabled_heroes: Vec<String>,
    disabled_heroes: Vec<String>,
}

impl Hero {
    /// Settings for both teams.
    pub fn general() -> Self {
        Self::scope("General")
    }

    /// Settings for one scope, for example `Hero::scope("Team 1")`.
    pub fn scope(scope: impl Display) -> Self {
        Self {
            scope: scope.to_string(),
            entries: Vec::new(),
            enabled_heroes: Vec::new(),
            disabled_heroes: Vec::new(),
        }
    }

    /// Sets one setting for every hero.
    #[must_use]
    pub fn set(mut self, key: &str, value: impl Display) -> Self {
        let source = format!("Hero::set ({})", self.scope);
        self.entries.push(HeroEntry {
            hero: None,
            entry: Entry::from_source(key, value, &source),
        });
        self
    }

    /// Sets one setting for a single hero.
    #[must_use]
    pub fn hero(mut self, hero: impl Display, key: &str, value: impl Display) -> Self {
        let source = format!("Hero::hero ({})", self.scope);
        self.entries.push(HeroEntry {
            hero: Some(hero.to_string()),
            entry: Entry::from_source(key, value, &source),
        });
        self
    }

    /// Adds a shared group of settings.
    #[must_use]
    pub fn with<F: HeroSettings>(mut self, fragment: F) -> Self {
        let source = type_name::<F>();
        self.entries
            .extend(fragment.entries().into_iter().map(|mut e| {
                e.entry = e.entry.sourced(source);
                e
            }));
        self
    }

    /// Allows only these heroes.
    #[must_use]
    pub fn enabled_heroes(mut self, heroes: impl IntoIterator<Item = impl Display>) -> Self {
        self.enabled_heroes.extend(names(heroes));
        self
    }

    /// Allows every hero except these.
    #[must_use]
    pub fn disabled_heroes(mut self, heroes: impl IntoIterator<Item = impl Display>) -> Self {
        self.disabled_heroes.extend(names(heroes));
        self
    }
}

impl HeroSettings for Hero {
    fn entries(&self) -> Vec<HeroEntry> {
        self.entries.clone()
    }
}

fn names(items: impl IntoIterator<Item = impl Display>) -> Vec<String> {
    items.into_iter().map(|item| item.to_string()).collect()
}

fn map_line(map: &impl Display, variants: impl IntoIterator<Item = impl Display>) -> String {
    let mut line = map.to_string();
    for id in variants {
        line.push(' ');
        line.push_str(&id.to_string());
    }
    line
}

// ---- resolved settings, what the emitter prints ----

pub(crate) struct ModeBlock {
    pub name: String,
    pub enabled: bool,
    pub entries: Vec<Entry>,
    pub enabled_maps: Vec<String>,
    pub disabled_maps: Vec<String>,
}

pub(crate) struct HeroBlock {
    pub scope: String,
    pub entries: Vec<Entry>,
    pub heroes: Vec<(String, Vec<Entry>)>,
    pub enabled_heroes: Vec<String>,
    pub disabled_heroes: Vec<String>,
}

pub(crate) struct Resolved {
    pub main: Vec<Entry>,
    pub lobby: Vec<Entry>,
    pub modes: Vec<ModeBlock>,
    pub heroes: Vec<HeroBlock>,
}

pub(crate) fn resolve(
    name: &str,
    description: &str,
    lobby: &[Lobby],
    modes: &[Mode],
    heroes: &[Hero],
) -> Result<Resolved, SettingsError> {
    let main_key = |key: &str| MAIN.get(key).map_or(key, |s| s.name()).to_owned();
    let main = vec![
        Entry::new(main_key("description"), quoted(description)),
        Entry::new(main_key("modeName"), quoted(name)),
    ];
    let lobby = merge(
        "lobby",
        lobby.iter().flat_map(LobbySettings::entries).collect(),
    )?;
    check("lobby", &lobby, &[&LOBBY], None)?;
    Ok(Resolved {
        main,
        lobby,
        modes: resolve_modes(modes)?,
        heroes: resolve_heroes(heroes)?,
    })
}

fn resolve_modes(modes: &[Mode]) -> Result<Vec<ModeBlock>, SettingsError> {
    let mut order: Vec<&str> = Vec::new();
    for mode in modes {
        if !order.contains(&mode.name.as_str()) {
            order.push(&mode.name);
        }
    }
    // `General` holds settings for all modes and goes last.
    order.sort_by_key(|name| *name == "General");

    order
        .into_iter()
        .map(|name| {
            let group: Vec<&Mode> = modes.iter().filter(|m| m.name == name).collect();
            let scope = format!("modes > {name}");
            let mut flags = Vec::new();
            for mode in &group {
                if let Some(enabled) = mode.enabled {
                    let flag = if enabled { "enabled" } else { "disabled" };
                    flags.push(Entry::from_source("enabled", flag, "Mode::new"));
                }
            }
            merge(&scope, flags)?;
            let entries = merge(
                &scope,
                group.iter().flat_map(|m| m.entries.clone()).collect(),
            )?;
            check_mode(&scope, name, &entries)?;
            Ok(ModeBlock {
                name: name.to_owned(),
                enabled: group.iter().all(|m| m.enabled != Some(false)),
                entries,
                enabled_maps: union(group.iter().flat_map(|m| &m.enabled_maps)),
                disabled_maps: union(group.iter().flat_map(|m| &m.disabled_maps)),
            })
        })
        .collect()
}

fn resolve_heroes(heroes: &[Hero]) -> Result<Vec<HeroBlock>, SettingsError> {
    let mut order: Vec<&str> = Vec::new();
    for hero in heroes {
        if !order.contains(&hero.scope.as_str()) {
            order.push(&hero.scope);
        }
    }
    let rank = |scope: &str| match scope {
        "General" => 0,
        "Team 1" => 1,
        "Team 2" => 2,
        _ => 3,
    };
    order.sort_by_key(|scope| rank(scope));

    order
        .into_iter()
        .map(|scope| {
            let group: Vec<&Hero> = heroes.iter().filter(|h| h.scope == scope).collect();
            let all: Vec<&HeroEntry> = group.iter().flat_map(|h| &h.entries).collect();
            let place = format!("heroes > {scope}");
            let scalar = all
                .iter()
                .filter(|e| e.hero.is_none())
                .map(|e| e.entry.clone())
                .collect();
            let mut names: Vec<&str> = Vec::new();
            for hero in all.iter().filter_map(|e| e.hero.as_deref()) {
                if !names.contains(&hero) {
                    names.push(hero);
                }
            }
            let per_hero = names
                .into_iter()
                .map(|hero| {
                    let entries = all
                        .iter()
                        .filter(|e| e.hero.as_deref() == Some(hero))
                        .map(|e| e.entry.clone())
                        .collect();
                    let place = format!("{place} > {hero}");
                    let entries = merge(&place, entries)?;
                    check_hero(&place, hero, &entries)?;
                    Ok((hero.to_owned(), entries))
                })
                .collect::<Result<_, SettingsError>>()?;
            let entries = merge(&place, scalar)?;
            check(&place, &entries, &[&HERO_GENERAL], None)?;
            Ok(HeroBlock {
                scope: scope.to_owned(),
                entries,
                heroes: per_hero,
                enabled_heroes: union(group.iter().flat_map(|h| &h.enabled_heroes)),
                disabled_heroes: union(group.iter().flat_map(|h| &h.disabled_heroes)),
            })
        })
        .collect()
}

fn unknown(scope: &str, name: &str) -> UnknownSetting {
    UnknownSetting {
        scope: scope.to_owned(),
        name: name.to_owned(),
    }
}

/// Every key must be a setting of one of `sections`, limited to `hero` where given.
fn check(
    scope: &str,
    entries: &[Entry],
    sections: &[&Section],
    hero: Option<DataHero>,
) -> Result<(), UnknownSetting> {
    for entry in entries {
        let known = sections.iter().any(|section| {
            section
                .settings
                .iter()
                .any(|s| s.name() == entry.key && hero.is_none_or(|h| s.applies_to(h)))
        });
        if !known {
            return Err(unknown(scope, &entry.key));
        }
    }
    Ok(())
}

fn check_mode(scope: &str, name: &str, entries: &[Entry]) -> Result<(), UnknownSetting> {
    if name == "General" {
        return check(scope, entries, &[&GAMEMODE_GENERAL], None);
    }
    let mode = Gamemode::ALL
        .iter()
        .copied()
        .find(|mode| mode.name().eq_ignore_ascii_case(name))
        .ok_or_else(|| unknown("modes", name))?;
    let mut sections = vec![&GAMEMODE_GENERAL];
    sections.extend(gamemode_settings(mode));
    check(scope, entries, &sections, None)
}

fn check_hero(scope: &str, name: &str, entries: &[Entry]) -> Result<(), UnknownSetting> {
    let hero = DataHero::ALL
        .iter()
        .copied()
        .find(|hero| hero.name().eq_ignore_ascii_case(name))
        .ok_or_else(|| unknown(scope, name))?;
    let mut sections = vec![&HERO_GENERAL];
    sections.extend(hero_settings(hero));
    check(scope, entries, &sections, Some(hero))
}

/// Joins entries by key. Equal repeats collapse; different values conflict.
fn merge(scope: &str, entries: Vec<Entry>) -> Result<Vec<Entry>, SettingsConflict> {
    let mut merged: Vec<Entry> = Vec::new();
    for entry in entries {
        match merged.iter().find(|have| have.key == entry.key) {
            None => merged.push(entry),
            Some(have) if have.value == entry.value => {}
            Some(have) => {
                return Err(SettingsConflict {
                    scope: scope.to_owned(),
                    key: entry.key,
                    first: (have.source.clone(), have.value.clone()),
                    second: (entry.source, entry.value),
                });
            }
        }
    }
    Ok(merged)
}

fn union<'a>(items: impl Iterator<Item = &'a String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for item in items {
        if !out.contains(item) {
            out.push(item.clone());
        }
    }
    out
}

pub(crate) fn quoted(text: &str) -> String {
    let escaped = text
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n");
    format!("\"{escaped}\"")
}
