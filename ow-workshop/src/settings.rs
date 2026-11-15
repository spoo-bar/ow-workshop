//! Custom game settings. Keys and values are written as they appear in the script.

use std::fmt::Display;

pub(crate) type Entries = Vec<(String, String)>;

fn entry(key: &str, value: impl Display) -> (String, String) {
    (key.to_owned(), value.to_string())
}

fn names(items: impl IntoIterator<Item = impl Display>) -> Vec<String> {
    items.into_iter().map(|item| item.to_string()).collect()
}

/// The `lobby` settings block.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Lobby {
    pub(crate) settings: Entries,
}

impl Lobby {
    /// A lobby with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets one setting, for example `set("Max Team 1 Players", 5)`.
    #[must_use]
    pub fn set(mut self, key: &str, value: impl Display) -> Self {
        self.settings.push(entry(key, value));
        self
    }
}

/// One game mode in the `modes` settings block.
#[derive(Clone, Debug, PartialEq)]
pub struct Mode {
    pub(crate) name: String,
    pub(crate) enabled: bool,
    pub(crate) settings: Entries,
    pub(crate) enabled_maps: Vec<String>,
    pub(crate) disabled_maps: Vec<String>,
}

impl Mode {
    /// An enabled mode, for example `Mode::new("Team Deathmatch")`. `"General"` holds settings for all modes.
    pub fn new(name: impl Display) -> Self {
        Self {
            name: name.to_string(),
            enabled: true,
            settings: Vec::new(),
            enabled_maps: Vec::new(),
            disabled_maps: Vec::new(),
        }
    }

    /// Turns the mode off.
    #[must_use]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    /// Sets one setting of this mode.
    #[must_use]
    pub fn set(mut self, key: &str, value: impl Display) -> Self {
        self.settings.push(entry(key, value));
        self
    }

    /// Allows only these maps.
    #[must_use]
    pub fn enabled_maps(mut self, maps: impl IntoIterator<Item = impl Display>) -> Self {
        self.enabled_maps = names(maps);
        self
    }

    /// Allows every map except these.
    #[must_use]
    pub fn disabled_maps(mut self, maps: impl IntoIterator<Item = impl Display>) -> Self {
        self.disabled_maps = names(maps);
        self
    }
}

/// One scope in the `heroes` settings block: `General`, `Team 1` or `Team 2`.
#[derive(Clone, Debug, PartialEq)]
pub struct Hero {
    pub(crate) scope: String,
    pub(crate) settings: Entries,
    pub(crate) overrides: Vec<(String, Entries)>,
    pub(crate) enabled_heroes: Vec<String>,
    pub(crate) disabled_heroes: Vec<String>,
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
            settings: Vec::new(),
            overrides: Vec::new(),
            enabled_heroes: Vec::new(),
            disabled_heroes: Vec::new(),
        }
    }

    /// Sets one setting for every hero.
    #[must_use]
    pub fn set(mut self, key: &str, value: impl Display) -> Self {
        self.settings.push(entry(key, value));
        self
    }

    /// Sets one setting for a single hero.
    #[must_use]
    pub fn hero(mut self, hero: impl Display, key: &str, value: impl Display) -> Self {
        let hero = hero.to_string();
        let slot = self
            .overrides
            .iter()
            .position(|(name, _)| *name == hero)
            .unwrap_or_else(|| {
                self.overrides.push((hero, Vec::new()));
                self.overrides.len() - 1
            });
        self.overrides[slot].1.push(entry(key, value));
        self
    }

    /// Allows only these heroes.
    #[must_use]
    pub fn enabled_heroes(mut self, heroes: impl IntoIterator<Item = impl Display>) -> Self {
        self.enabled_heroes = names(heroes);
        self
    }

    /// Allows every hero except these.
    #[must_use]
    pub fn disabled_heroes(mut self, heroes: impl IntoIterator<Item = impl Display>) -> Self {
        self.disabled_heroes = names(heroes);
        self
    }
}
