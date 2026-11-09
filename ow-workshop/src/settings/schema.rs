//! Looking settings up in the schema.

use ow_workshop_data::Hero;
use ow_workshop_data::settings::{
    GAMEMODE_GENERAL, HERO_GENERAL, Section, Setting, gamemode_settings, hero_settings,
};

use super::ModeKey;

/// Setting keys that Overwatch spelled differently in older versions.
const RENAMED_KEYS: [(&str, &str); 1] = [("ability1KB%", "ability1Kb%")];

/// The sections that hold a game mode's settings, most specific first.
pub(crate) fn mode_sections(key: ModeKey) -> Vec<&'static Section> {
    let specific = match key {
        ModeKey::General => None,
        ModeKey::Mode(mode) => gamemode_settings(mode),
    };
    specific.into_iter().chain([&GAMEMODE_GENERAL]).collect()
}

/// The sections that hold a hero's settings, most specific first.
pub(crate) fn hero_sections(hero: Hero) -> Vec<&'static Section> {
    hero_settings(hero)
        .into_iter()
        .chain([&HERO_GENERAL])
        .collect()
}

/// The setting with this key in the first section that has it.
pub(crate) fn find(sections: &[&Section], key: &str) -> Option<&'static Setting> {
    let key = RENAMED_KEYS
        .iter()
        .find(|(old, _)| *old == key)
        .map_or(key, |(_, new)| new);
    sections.iter().find_map(|section| section.get(key))
}
