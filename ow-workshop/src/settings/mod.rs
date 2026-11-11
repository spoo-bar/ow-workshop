//! Typed custom game settings, returned by [`Workshop::settings`](crate::Workshop::settings).
//!
//! Overwatch renames settings over time, so the setting maps keep keys the schema lacks as given
//! instead of failing; use `raw` for those.

use ow_workshop_data::{Extension, Gamemode, Hero, Map};

pub(crate) mod schema;
mod validate;
mod values;

#[cfg(feature = "serde")]
mod json;

pub(crate) use validate::validate;
pub use values::{SettingValue, Values};

/// Custom game settings: the `settings {}` block.
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct Settings {
    /// The `main` section.
    pub main: Option<Main>,
    /// The `lobby` section, if the mod sets one.
    pub lobby: Option<Lobby>,
    /// Per game mode settings.
    pub modes: Vec<(ModeKey, ModeSettings)>,
    /// Hero settings per team.
    pub heroes: Vec<(HeroTeam, HeroSettings)>,
    /// Values for workshop settings, keyed by the setting's name.
    pub workshop: Vec<(String, WorkshopValue)>,
    /// Workshop extensions the mod turns on.
    pub extensions: Vec<Extension>,
    /// Skips validation and prints nothing when empty; see [`Settings::unchecked`].
    pub(crate) unchecked: bool,
}

impl Settings {
    /// Sets the `main` section.
    #[must_use]
    pub fn with_main(mut self, main: Main) -> Self {
        self.main = Some(main);
        self
    }

    /// Sets the `lobby` section.
    #[must_use]
    pub fn with_lobby(mut self, lobby: Lobby) -> Self {
        self.lobby = Some(lobby);
        self
    }

    /// Sets a game mode's settings, replacing earlier ones for the same mode.
    #[must_use]
    pub fn with_mode(mut self, key: ModeKey, settings: ModeSettings) -> Self {
        replace(&mut self.modes, key, settings);
        self
    }

    /// Sets a team's hero settings, replacing earlier ones for the same team.
    #[must_use]
    pub fn with_heroes(mut self, team: HeroTeam, settings: HeroSettings) -> Self {
        replace(&mut self.heroes, team, settings);
        self
    }

    /// Gives a workshop setting a value in the lobby, replacing an earlier one.
    #[must_use]
    pub fn with_workshop_value(mut self, name: impl Into<String>, value: WorkshopValue) -> Self {
        replace(&mut self.workshop, name.into(), value);
        self
    }

    /// Turns a workshop extension on.
    #[must_use]
    pub fn with_extension(mut self, extension: Extension) -> Self {
        if !self.extensions.contains(&extension) {
            self.extensions.push(extension);
        }
        self
    }

    /// Settings that skip validation and print no block while nothing is set.
    ///
    /// The game rejects code without a `settings` block, or with a game mode that has no map, so
    /// a real mod must declare complete settings. This exists only for tests of other parts of the
    /// compiler.
    #[doc(hidden)]
    #[must_use]
    pub fn unchecked() -> Self {
        Self::default().without_checks()
    }

    /// These settings, skipping validation; see [`Settings::unchecked`]. For reference tests that
    /// compile settings exported from the game as they are.
    #[doc(hidden)]
    #[must_use]
    pub fn without_checks(mut self) -> Self {
        self.unchecked = true;
        self
    }

    /// Whether nothing is set, so no settings block is printed.
    pub(crate) fn is_empty(&self) -> bool {
        Self {
            unchecked: false,
            ..self.clone()
        } == Self::default()
    }

    /// Whether the extension is on.
    pub(crate) fn has_extension(&self, extension: Extension) -> bool {
        self.extensions.contains(&extension)
    }
}

/// Replaces the entry with this key, or appends one.
fn replace<K: PartialEq, V>(entries: &mut Vec<(K, V)>, key: K, value: V) {
    match entries.iter_mut().find(|(k, _)| *k == key) {
        Some((_, slot)) => *slot = value,
        None => entries.push((key, value)),
    }
}

/// The `main` section. Both fields are required.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Main {
    /// The name of the mode.
    pub mode_name: String,
    /// The description of the mode.
    pub description: String,
}

/// The `lobby` section. Typed setters, one per schema setting, are generated.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Lobby {
    /// The values set so far.
    pub values: Values,
}

impl Lobby {
    /// Sets a setting by its camelCase key, for keys the schema lacks.
    #[must_use]
    pub fn raw(mut self, key: &str, value: impl Into<SettingValue>) -> Self {
        self.values.set(key, value);
        self
    }
}

/// Which settings a game mode entry holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModeKey {
    /// Settings shared by all modes.
    General,
    /// One game mode.
    Mode(Gamemode),
}

/// One game mode's settings.
#[derive(Clone, Debug, PartialEq)]
pub struct ModeSettings {
    /// Whether the mode is on.
    pub enabled: bool,
    /// Which maps the mode plays on.
    pub maps: Option<MapSelection>,
    /// The mode's setting values.
    pub values: Values,
}

impl Default for ModeSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            maps: None,
            values: Values::new(),
        }
    }
}

impl ModeSettings {
    /// Turns the mode off.
    #[must_use]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    /// Chooses the maps.
    #[must_use]
    pub fn maps(mut self, maps: MapSelection) -> Self {
        self.maps = Some(maps);
        self
    }

    /// Sets a setting by its camelCase key, for keys the schema lacks.
    #[must_use]
    pub fn raw(mut self, key: &str, value: impl Into<SettingValue>) -> Self {
        self.values.set(key, value);
        self
    }
}

/// A map list: either the maps that are on, or the ones that are off.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MapSelection {
    /// Only these maps.
    Enabled(Vec<MapRef>),
    /// All maps except these.
    Disabled(Vec<MapRef>),
}

/// A map in a map list, with the variants that are picked.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapRef {
    /// The map.
    pub map: Map,
    /// Variant ids, e.g. `dawn`.
    pub variants: Vec<String>,
}

impl MapRef {
    /// The map, with no variants.
    #[must_use]
    pub fn new(map: Map) -> Self {
        Self {
            map,
            variants: Vec::new(),
        }
    }

    /// Adds a variant id.
    #[must_use]
    pub fn variant(mut self, id: impl Into<String>) -> Self {
        self.variants.push(id.into());
        self
    }
}

impl From<Map> for MapRef {
    fn from(map: Map) -> Self {
        Self::new(map)
    }
}

/// The team a set of hero settings is for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeroTeam {
    /// Both teams.
    AllTeams,
    /// Team 1.
    Team1,
    /// Team 2.
    Team2,
}

/// A hero list: either the heroes that are on, or the ones that are off.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HeroSelection {
    /// Only these heroes.
    Enabled(Vec<Hero>),
    /// All heroes except these.
    Disabled(Vec<Hero>),
}

/// One team's hero settings. Typed setters for the settings every hero has are generated.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HeroSettings {
    /// Which heroes can be picked.
    pub selection: Option<HeroSelection>,
    /// Settings that apply to every hero.
    pub general: Values,
    /// Settings of single heroes.
    pub per_hero: Vec<(Hero, Values)>,
}

impl HeroSettings {
    /// Chooses the heroes.
    #[must_use]
    pub fn selection(mut self, selection: HeroSelection) -> Self {
        self.selection = Some(selection);
        self
    }

    /// Sets a hero's settings, replacing earlier ones for the same hero.
    #[must_use]
    pub fn hero(mut self, hero: Hero, values: Values) -> Self {
        replace(&mut self.per_hero, hero, values);
        self
    }

    /// Sets a general setting by its camelCase key, for keys the schema lacks.
    #[must_use]
    pub fn raw(mut self, key: &str, value: impl Into<SettingValue>) -> Self {
        self.general.set(key, value);
        self
    }
}

/// The value of a workshop setting, as chosen in the lobby settings.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum WorkshopValue {
    /// A toggle.
    Bool(bool),
    /// An integer or real number.
    Number(f64),
    /// The index of a combo's option.
    Combo(usize),
    /// A hero.
    Hero(Hero),
}

#[cfg(test)]
mod tests;
