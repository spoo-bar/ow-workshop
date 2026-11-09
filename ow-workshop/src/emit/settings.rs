//! The `settings {}` block.
//!
//! Tab-indented. An object is its title on one line and `{` on the next; a scalar is
//! `key: value`; a list is a titled object with one item per line. Keys and choices come from the
//! schema in the output language; anything the schema lacks prints as given.

use core::fmt::{self, Write};

use ow_workshop_data::settings::{
    GAMEMODE_GENERAL, HERO_GENERAL, Kind, LOBBY, MAIN, Section, gamemode_settings, hero_settings,
};
use ow_workshop_data::{Extension, Keyword, Label, Lang};

use super::{Cx, write_censored_string, write_number};
use crate::settings::{
    HeroSelection, HeroSettings, HeroTeam, MapRef, MapSelection, ModeKey, ModeSettings,
    SettingValue, Settings, Values, WorkshopValue,
};

/// Setting keys that Overwatch spelled differently in older versions.
const RENAMED_KEYS: [(&str, &str); 1] = [("ability1KB%", "ability1Kb%")];

/// Writes the block and the blank line after it, or nothing when no settings are set.
pub(super) fn write_settings(out: &mut impl Write, cx: Cx<'_>) -> fmt::Result {
    let settings = &cx.program.settings;
    if settings.is_empty() {
        return Ok(());
    }
    let mut w = Writer {
        out,
        lang: cx.lang,
        depth: 0,
    };
    w.open(Keyword::Settings.name(cx.lang))?;
    w.main(settings)?;
    w.lobby(settings)?;
    w.modes(settings)?;
    w.heroes(settings)?;
    w.workshop(settings)?;
    w.extensions(settings)?;
    w.close()?;
    w.out.write_char('\n')
}

struct Writer<'a, O: Write> {
    out: &'a mut O,
    lang: Lang,
    depth: usize,
}

impl<O: Write> Writer<'_, O> {
    fn tabs(&mut self) -> fmt::Result {
        for _ in 0..self.depth {
            self.out.write_char('\t')?;
        }
        Ok(())
    }

    /// `title`, then `{`, and one level deeper.
    fn open(&mut self, title: &str) -> fmt::Result {
        self.line(title)?;
        self.line("{")?;
        self.depth += 1;
        Ok(())
    }

    fn close(&mut self) -> fmt::Result {
        self.depth -= 1;
        self.line("}")
    }

    fn line(&mut self, text: &str) -> fmt::Result {
        self.tabs()?;
        self.out.write_str(text)?;
        self.out.write_char('\n')
    }

    fn main(&mut self, settings: &Settings) -> fmt::Result {
        let main = &settings.main;
        if main.mode_name.is_none() && main.description.is_none() {
            return Ok(());
        }
        self.open(MAIN.name(self.lang))?;
        for (key, text) in [
            ("modeName", &main.mode_name),
            ("description", &main.description),
        ] {
            if let Some(text) = text {
                self.tabs()?;
                self.out.write_str(&key_name(&[&MAIN], key, self.lang))?;
                self.out.write_str(": ")?;
                write_censored_string(self.out, text)?;
                self.out.write_char('\n')?;
            }
        }
        self.close()
    }

    fn lobby(&mut self, settings: &Settings) -> fmt::Result {
        let Some(lobby) = &settings.lobby else {
            return Ok(());
        };
        self.open(LOBBY.name(self.lang))?;
        self.values(&[&LOBBY], &lobby.values)?;
        self.close()
    }

    fn modes(&mut self, settings: &Settings) -> fmt::Result {
        if settings.modes.is_empty() {
            return Ok(());
        }
        self.open(Label::Gamemodes.name(self.lang))?;
        for (key, mode) in &settings.modes {
            self.mode(*key, mode)?;
        }
        self.close()
    }

    fn mode(&mut self, key: ModeKey, mode: &ModeSettings) -> fmt::Result {
        let (name, section) = match key {
            ModeKey::General => (GAMEMODE_GENERAL.name(self.lang), None),
            ModeKey::Mode(m) => (m.name(self.lang), gamemode_settings(m)),
        };
        if !mode.enabled {
            return self.line(&format!("{} {name}", Keyword::Disabled.name(self.lang)));
        }
        self.open(name)?;
        if let Some(maps) = &mode.maps {
            let (label, maps) = match maps {
                MapSelection::Enabled(maps) => (Label::EnabledMaps, maps),
                MapSelection::Disabled(maps) => (Label::DisabledMaps, maps),
            };
            self.open(label.name(self.lang))?;
            for map in maps {
                self.map(map)?;
            }
            self.close()?;
        }
        let sections: Vec<&Section> = section.into_iter().chain([&GAMEMODE_GENERAL]).collect();
        self.values(&sections, &mode.values)?;
        self.close()
    }

    fn map(&mut self, map: &MapRef) -> fmt::Result {
        self.tabs()?;
        self.out.write_str(map.map.name(self.lang))?;
        for variant in &map.variants {
            // Variants are picked by key; the game wants the id.
            let id = map
                .map
                .variants()
                .iter()
                .find(|(key, _)| key == variant)
                .map_or(variant.as_str(), |(_, id)| id);
            write!(self.out, " {id}")?;
        }
        self.out.write_char('\n')
    }

    fn heroes(&mut self, settings: &Settings) -> fmt::Result {
        if settings.heroes.is_empty() {
            return Ok(());
        }
        self.open(Label::Heroes.name(self.lang))?;
        for (team, hero_settings) in &settings.heroes {
            let label = match team {
                HeroTeam::AllTeams => Label::TeamAll,
                HeroTeam::Team1 => Label::TeamTeam1,
                HeroTeam::Team2 => Label::TeamTeam2,
            };
            self.open(label.name(self.lang))?;
            self.team(hero_settings)?;
            self.close()?;
        }
        self.close()
    }

    /// The general values, then each hero, then the hero list, which must come last.
    fn team(&mut self, team: &HeroSettings) -> fmt::Result {
        self.values(&[&HERO_GENERAL], &team.general)?;
        for (hero, values) in &team.per_hero {
            self.open(hero.name(self.lang))?;
            let sections: Vec<&Section> = hero_settings(*hero)
                .into_iter()
                .chain([&HERO_GENERAL])
                .collect();
            self.values(&sections, values)?;
            self.close()?;
        }
        if let Some(selection) = &team.selection {
            let (label, heroes) = match selection {
                HeroSelection::Enabled(heroes) => (Label::EnabledHeroes, heroes),
                HeroSelection::Disabled(heroes) => (Label::DisabledHeroes, heroes),
            };
            self.open(label.name(self.lang))?;
            for hero in heroes {
                self.line(hero.name(self.lang))?;
            }
            self.close()?;
        }
        Ok(())
    }

    fn workshop(&mut self, settings: &Settings) -> fmt::Result {
        if settings.workshop.is_empty() {
            return Ok(());
        }
        self.open(Label::Workshop.name(self.lang))?;
        for (name, value) in &settings.workshop {
            self.tabs()?;
            write!(self.out, "{name}: ")?;
            match value {
                WorkshopValue::Bool(b) => self.out.write_str(on_off(*b, self.lang))?,
                WorkshopValue::Number(n) => write_number(self.out, *n)?,
                WorkshopValue::Combo(index) => write!(self.out, "[{index}]")?,
                WorkshopValue::Hero(hero) => self.out.write_str(hero.name(self.lang))?,
            }
            self.out.write_char('\n')?;
        }
        self.close()
    }

    fn extensions(&mut self, settings: &Settings) -> fmt::Result {
        if settings.extensions.is_empty() {
            return Ok(());
        }
        self.open(Label::Extensions.name(self.lang))?;
        let mut seen: Vec<Extension> = Vec::new();
        for extension in &settings.extensions {
            if !seen.contains(extension) {
                seen.push(*extension);
                self.line(extension.name(self.lang))?;
            }
        }
        self.close()
    }

    /// `Name: value` lines, with names and values looked up in `sections`.
    fn values(&mut self, sections: &[&Section], values: &Values) -> fmt::Result {
        for (key, value) in values.iter() {
            self.tabs()?;
            self.out.write_str(&key_name(sections, key, self.lang))?;
            self.out.write_str(": ")?;
            self.value(find(sections, key).map(|s| s.kind), value)?;
            self.out.write_char('\n')?;
        }
        Ok(())
    }

    fn value(&mut self, kind: Option<Kind>, value: &SettingValue) -> fmt::Result {
        match (value, kind) {
            (SettingValue::Bool(b), Some(kind)) => {
                self.out.write_str(&bool_word(kind, *b, self.lang))
            }
            (SettingValue::Bool(b), None) => self.out.write_str(on_off(*b, self.lang)),
            (SettingValue::Number(n) | SettingValue::Percent(n), kind) => {
                write_number(self.out, *n)?;
                if matches!(value, SettingValue::Percent(_)) || matches!(kind, Some(Kind::Percent))
                {
                    self.out.write_char('%')?;
                }
                Ok(())
            }
            (SettingValue::Str(s) | SettingValue::Choice(s), Some(Kind::Combo(choices))) => {
                let name = choices
                    .iter()
                    .find(|choice| choice.key == s)
                    .map_or(s.as_str(), |choice| choice.name(self.lang));
                self.out.write_str(name)
            }
            (SettingValue::Str(s), Some(Kind::String { .. })) => write_censored_string(self.out, s),
            (SettingValue::Str(s) | SettingValue::Choice(s), _) => self.out.write_str(s),
        }
    }
}

/// The setting with this key in the first section that has it.
fn find(sections: &[&Section], key: &str) -> Option<&'static ow_workshop_data::settings::Setting> {
    let key = RENAMED_KEYS
        .iter()
        .find(|(old, _)| *old == key)
        .map_or(key, |(_, new)| new);
    sections.iter().find_map(|section| section.get(key))
}

/// The localized name of a setting, or `key` itself when the schema lacks it.
fn key_name(sections: &[&Section], key: &str, lang: Lang) -> String {
    find(sections, key).map_or_else(|| key.to_owned(), |s| s.name(lang).to_owned())
}

fn on_off(value: bool, lang: Lang) -> &'static str {
    let word = if value {
        Keyword::SettingsOn
    } else {
        Keyword::SettingsOff
    };
    word.name(lang)
}

/// How a boolean is written for a setting of this kind.
fn bool_word(kind: Kind, value: bool, lang: Lang) -> String {
    let word = match kind {
        Kind::BoolYesNo => {
            if value {
                Keyword::SettingsYes
            } else {
                Keyword::SettingsNo
            }
        }
        Kind::BoolEnabled | Kind::BoolReverseEnabled => {
            if value == matches!(kind, Kind::BoolEnabled) {
                Keyword::SettingsEnabled
            } else {
                Keyword::SettingsDisabled
            }
        }
        Kind::BoolReverseOnOff => {
            return on_off(!value, lang).to_owned();
        }
        _ => return on_off(value, lang).to_owned(),
    };
    word.name(lang).to_owned()
}

#[cfg(test)]
mod tests;
