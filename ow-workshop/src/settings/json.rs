//! Reads settings from JSON (feature `serde`), using the schema's camelCase keys.
//!
//! The JSON has the shape of the reference format: `main`, `lobby`, `gamemodes` (or `modes`),
//! `heroes`, `workshop` and `extensions`. Numbers are typed by the schema, so
//! `"respawnTime%": 50` becomes a percent. Keys the schema lacks are kept as given.

use core::fmt;

use ow_workshop_data::settings::{
    GAMEMODE_GENERAL, HERO_GENERAL, Kind, LOBBY, Section, gamemode_settings, hero_settings,
};
use ow_workshop_data::{Extension, Gamemode, Hero, Map};
use serde::Deserialize;
use serde::de::{self, Deserializer, MapAccess, Visitor};

use super::{
    HeroSelection, HeroSettings, HeroTeam, Lobby, Main, MapRef, MapSelection, ModeKey,
    ModeSettings, SettingValue, Settings, Values, WorkshopValue,
};

/// A JSON object that keeps its key order.
#[derive(Debug)]
struct Ordered<V>(Vec<(String, V)>);

impl<V> Default for Ordered<V> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<'de, V: Deserialize<'de>> Deserialize<'de> for Ordered<V> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Entries<V>(core::marker::PhantomData<V>);

        impl<'de, V: Deserialize<'de>> Visitor<'de> for Entries<V> {
            type Value = Ordered<V>;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("an object")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut entries = Vec::new();
                while let Some(entry) = map.next_entry()? {
                    entries.push(entry);
                }
                Ok(Ordered(entries))
            }
        }

        deserializer.deserialize_map(Entries(core::marker::PhantomData))
    }
}

/// Anything that can be a value in the settings JSON.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Entry {
    Bool(bool),
    Number(f64),
    Str(String),
    List(Vec<Item>),
    Object(Ordered<Entry>),
}

/// A list item: a key, or a map with variants.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Item {
    Key(String),
    Full {
        map: String,
        #[serde(default)]
        variants: Vec<String>,
    },
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Raw {
    main: Ordered<Entry>,
    lobby: Option<Ordered<Entry>>,
    #[serde(alias = "modes")]
    gamemodes: Ordered<Entry>,
    heroes: Ordered<Entry>,
    workshop: Ordered<Entry>,
    extensions: Vec<String>,
}

type Res<T> = core::result::Result<T, String>;

/// Hero keys that older settings files use.
const LEGACY_HEROES: [(&str, Hero); 2] =
    [("mccree", Hero::Cassidy), ("hammond", Hero::WreckingBall)];

fn hero(key: &str) -> Res<Hero> {
    LEGACY_HEROES
        .iter()
        .find(|(legacy, _)| *legacy == key)
        .map(|&(_, hero)| hero)
        .or_else(|| Hero::ALL.iter().copied().find(|h| h.key() == key))
        .ok_or_else(|| format!("unknown hero `{key}`"))
}

fn setting_value(sections: &[&Section], key: &str, entry: &Entry) -> Res<SettingValue> {
    let kind = sections
        .iter()
        .find_map(|section| section.get(key))
        .map(|setting| setting.kind);
    Ok(match entry {
        Entry::Bool(b) => SettingValue::Bool(*b),
        Entry::Number(n) if matches!(kind, Some(Kind::Percent)) => SettingValue::Percent(*n),
        Entry::Number(n) => SettingValue::Number(*n),
        Entry::Str(s) if matches!(kind, Some(Kind::Combo(_))) => SettingValue::Choice(s.clone()),
        Entry::Str(s) => SettingValue::Str(s.clone()),
        Entry::List(_) | Entry::Object(_) => {
            return Err(format!("`{key}` must be a boolean, number or string"));
        }
    })
}

fn values<'a>(
    sections: &[&Section],
    entries: impl IntoIterator<Item = &'a (String, Entry)>,
) -> Res<Values> {
    let mut out = Values::new();
    for (key, entry) in entries {
        out.set(key.as_str(), setting_value(sections, key, entry)?);
    }
    Ok(out)
}

fn string<'a>(key: &str, entry: &'a Entry) -> Res<&'a str> {
    match entry {
        Entry::Str(s) => Ok(s),
        _ => Err(format!("`{key}` must be a string")),
    }
}

fn main_section(entries: &[(String, Entry)]) -> Res<Main> {
    let mut main = Main::default();
    for (key, entry) in entries {
        let slot = match key.as_str() {
            "modeName" => &mut main.mode_name,
            "description" => &mut main.description,
            _ => return Err(format!("unknown `main` setting `{key}`")),
        };
        *slot = Some(string(key, entry)?.to_owned());
    }
    Ok(main)
}

fn maps(key: &str, entry: &Entry) -> Res<Vec<MapRef>> {
    let Entry::List(items) = entry else {
        return Err(format!("`{key}` must be a list"));
    };
    items
        .iter()
        .map(|item| {
            let (name, variants) = match item {
                Item::Key(name) => (name, Vec::new()),
                Item::Full { map, variants } => (map, variants.clone()),
            };
            let map = Map::ALL
                .iter()
                .copied()
                .find(|m| m.key() == name)
                .ok_or_else(|| format!("unknown map `{name}`"))?;
            Ok(MapRef { map, variants })
        })
        .collect()
}

fn heroes_list(key: &str, entry: &Entry) -> Res<Vec<Hero>> {
    let Entry::List(items) = entry else {
        return Err(format!("`{key}` must be a list"));
    };
    items
        .iter()
        .map(|item| match item {
            Item::Key(name) => hero(name),
            Item::Full { .. } => Err(format!("`{key}` must list hero keys")),
        })
        .collect()
}

fn mode(key: &str, entries: &[(String, Entry)]) -> Res<(ModeKey, ModeSettings)> {
    let (mode_key, section) = if key == "general" {
        (ModeKey::General, None)
    } else {
        let mode = Gamemode::ALL
            .iter()
            .copied()
            .find(|m| m.key() == key)
            .ok_or_else(|| format!("unknown game mode `{key}`"))?;
        (ModeKey::Mode(mode), gamemode_settings(mode))
    };
    let sections: Vec<&Section> = section.into_iter().chain([&GAMEMODE_GENERAL]).collect();
    let mut settings = ModeSettings::default();
    let mut rest = Vec::new();
    for (name, entry) in entries {
        match (name.as_str(), entry) {
            ("enabled", Entry::Bool(enabled)) => settings.enabled = *enabled,
            ("enabledMaps", entry) => {
                if settings.maps.is_some() {
                    return Err(format!("`{key}` has both enabledMaps and disabledMaps"));
                }
                settings.maps = Some(MapSelection::Enabled(maps(name, entry)?));
            }
            ("disabledMaps", entry) => {
                if settings.maps.is_some() {
                    return Err(format!("`{key}` has both enabledMaps and disabledMaps"));
                }
                settings.maps = Some(MapSelection::Disabled(maps(name, entry)?));
            }
            _ => rest.push((name.clone(), entry)),
        }
    }
    for (name, entry) in rest {
        settings
            .values
            .set(name.as_str(), setting_value(&sections, &name, entry)?);
    }
    Ok((mode_key, settings))
}

fn team(key: &str, entries: &[(String, Entry)]) -> Res<(HeroTeam, HeroSettings)> {
    let team = match key {
        "allTeams" => HeroTeam::AllTeams,
        "team1" => HeroTeam::Team1,
        "team2" => HeroTeam::Team2,
        _ => return Err(format!("unknown hero team `{key}`")),
    };
    let mut settings = HeroSettings::default();
    for (name, entry) in entries {
        match (name.as_str(), entry) {
            ("enabledHeroes" | "disabledHeroes", entry) => {
                if settings.selection.is_some() {
                    return Err(format!("`{key}` has both enabledHeroes and disabledHeroes"));
                }
                let list = heroes_list(name, entry)?;
                settings.selection = Some(if name == "enabledHeroes" {
                    HeroSelection::Enabled(list)
                } else {
                    HeroSelection::Disabled(list)
                });
            }
            (_, Entry::Object(Ordered(inner))) => {
                let h = hero(name)?;
                let sections: Vec<&Section> = hero_settings(h)
                    .into_iter()
                    .chain([&HERO_GENERAL])
                    .collect();
                settings.per_hero.push((h, values(&sections, inner)?));
            }
            _ => settings
                .general
                .set(name.as_str(), setting_value(&[&HERO_GENERAL], name, entry)?),
        }
    }
    Ok((team, settings))
}

fn workshop_value(key: &str, entry: &Entry) -> Res<WorkshopValue> {
    match entry {
        Entry::Bool(b) => Ok(WorkshopValue::Bool(*b)),
        Entry::Number(n) => Ok(WorkshopValue::Number(*n)),
        Entry::Object(Ordered(inner)) => match inner.as_slice() {
            [(tag, Entry::Number(index))] if tag == "combo" && *index >= 0.0 => {
                // The index is a small non-negative whole number.
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                Ok(WorkshopValue::Combo(*index as usize))
            }
            [(tag, Entry::Str(name))] if tag == "hero" => Ok(WorkshopValue::Hero(hero(name)?)),
            _ => Err(format!(
                "`{key}` must be {{\"combo\": index}} or {{\"hero\": key}}"
            )),
        },
        _ => Err(format!("`{key}` must be a boolean, number, combo or hero")),
    }
}

fn convert(raw: &Raw) -> Res<Settings> {
    let mut settings = Settings {
        main: main_section(&raw.main.0)?,
        ..Settings::default()
    };
    if let Some(Ordered(entries)) = &raw.lobby {
        settings.lobby = Some(Lobby {
            values: values(&[&LOBBY], entries)?,
        });
    }
    for (key, entry) in &raw.gamemodes.0 {
        let Entry::Object(Ordered(entries)) = entry else {
            return Err(format!("`{key}` must be an object"));
        };
        settings.modes.push(mode(key, entries)?);
    }
    for (key, entry) in &raw.heroes.0 {
        let Entry::Object(Ordered(entries)) = entry else {
            return Err(format!("`{key}` must be an object"));
        };
        settings.heroes.push(team(key, entries)?);
    }
    for (key, entry) in &raw.workshop.0 {
        settings
            .workshop
            .push((key.clone(), workshop_value(key, entry)?));
    }
    for key in &raw.extensions {
        let extension = Extension::ALL
            .iter()
            .copied()
            .find(|e| e.key() == key)
            .ok_or_else(|| format!("unknown extension `{key}`"))?;
        settings = settings.with_extension(extension);
    }
    Ok(settings)
}

impl<'de> Deserialize<'de> for Settings {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> core::result::Result<Self, D::Error> {
        let raw = Raw::deserialize(deserializer)?;
        convert(&raw).map_err(de::Error::custom)
    }
}
