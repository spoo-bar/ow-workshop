//! Checks the settings and the workshop extension budget.

use crate::ir::ErrorKind;
use ow_workshop_data::settings::{Kind, Section};
use ow_workshop_data::{Extension, Gamemode, Lang};

use super::schema::{find, hero_sections, mode_sections};
use super::{HeroTeam, MapSelection, ModeKey, ModeSettings, SettingValue, Settings, Values};
use crate::ir::Diagnostics;
use crate::options::CompileOptions;
use crate::warning::Warning;

/// Players the game can seat.
const MAX_SLOTS: u32 = 12;
/// Extension points each unused slot gives.
const POINTS_PER_FREE_SLOT: u32 = 4;
/// Extension points when only workshop maps are played.
const WORKSHOP_MAPS_POINTS: u32 = 16;

/// What is wrong with a number setting.
#[derive(Debug, PartialEq)]
enum NumberProblem {
    NotWhole,
    TooSmall(f64),
    TooLarge(f64),
}

/// What is wrong with `n` for a setting of this kind and range, if anything.
fn number_problem(kind: Kind, min: Option<f64>, max: Option<f64>, n: f64) -> Option<NumberProblem> {
    if !matches!(kind, Kind::Int | Kind::Float | Kind::Percent) {
        return None;
    }
    if matches!(kind, Kind::Int) && n.fract() != 0.0 {
        return Some(NumberProblem::NotWhole);
    }
    if let Some(min) = min.filter(|&min| n < min) {
        return Some(NumberProblem::TooSmall(min));
    }
    max.filter(|&max| n > max).map(NumberProblem::TooLarge)
}

/// Whether `text` is longer than a string setting allows: in Unicode scalars, then in UTF-8 bytes.
fn string_problem(max_chars: Option<u32>, max_bytes: Option<u32>, text: &str) -> Option<String> {
    let chars = text.chars().count();
    if max_chars.is_some_and(|max| chars > max as usize) {
        return Some(format!(
            "is {chars} characters long; the most is {}",
            max_chars.unwrap_or_default()
        ));
    }
    let bytes = text.len();
    max_bytes
        .filter(|&max| bytes > max as usize)
        .map(|max| format!("is {bytes} bytes long in UTF-8; the most is {max}"))
}

/// Reports problems with `settings` and applies the silent fixes. Settings made with
/// [`Settings::unchecked`] are left alone.
pub(crate) fn validate(settings: &mut Settings, options: &CompileOptions, diags: &mut Diagnostics) {
    if settings.unchecked {
        return;
    }
    drop_best_available(settings);
    let mut check = Check { diags };
    // `General` only holds settings shared by every mode; it is not a mode to play.
    let playable =
        |(key, mode): &&(ModeKey, ModeSettings)| mode.enabled && *key != ModeKey::General;
    if settings.modes.iter().find(playable).is_none() {
        check.error(
            ErrorKind::SettingsNoModes,
            "the settings enable no game mode; enable at least one, with a map".to_owned(),
        );
    }
    check.maps(settings);
    check.values(settings);
    let slots = Slots::of(settings);
    let max_slots = slots.max();
    if max_slots > MAX_SLOTS {
        check.error(
            ErrorKind::SettingsSlots,
            format!(
                "the lobby seats {max_slots} players (team 1: {}, team 2: {}, free-for-all: {}); \
                 the most is {MAX_SLOTS}",
                slots.team1, slots.team2, slots.ffa
            ),
        );
    } else if !options.suppress.contains(Warning::ExtensionPoints) {
        check.extension_points(settings, max_slots);
    }
}

/// `Best Available` cannot be pasted into the workshop, so it is the same as no preference.
fn drop_best_available(settings: &mut Settings) {
    let Some(lobby) = &mut settings.lobby else {
        return;
    };
    let best = |value: &SettingValue| matches!(value, SettingValue::Choice(c) | SettingValue::Str(c) if c == "bestAvailable");
    if lobby.values.get("dataCenterPreference").is_some_and(best) {
        lobby.values.remove("dataCenterPreference");
    }
}

/// The seats the lobby settings give each team, with game mode defaults for the unset ones.
struct Slots {
    team1: u32,
    team2: u32,
    ffa: u32,
}

impl Slots {
    fn of(settings: &Settings) -> Self {
        let set = |key: &str| {
            let value = settings.lobby.as_ref()?.values.get(key)?;
            match value {
                // A count is a small whole number; anything else is reported as a value problem.
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                SettingValue::Number(n) => Some(n.clamp(0.0, f64::from(u32::MAX)) as u32),
                _ => None,
            }
        };
        // The largest default among the modes that are played.
        let default = |slot: fn(&ow_workshop_data::Slots) -> Option<u32>| {
            settings
                .modes
                .iter()
                .filter(|(_, mode)| mode.enabled)
                .filter_map(|(key, _)| match key {
                    ModeKey::Mode(mode) => slot(&mode.default_slots()),
                    ModeKey::General => None,
                })
                .max()
                .unwrap_or(0)
        };
        Self {
            team1: set("team1Slots").unwrap_or_else(|| default(|s| s.team1)),
            team2: set("team2Slots").unwrap_or_else(|| default(|s| s.team2)),
            ffa: set("ffaSlots").unwrap_or_else(|| default(|s| s.ffa)),
        }
    }

    fn max(&self) -> u32 {
        self.team1.saturating_add(self.team2).max(self.ffa)
    }
}

struct Check<'a> {
    diags: &'a mut Diagnostics,
}

impl Check<'_> {
    fn error(&mut self, code: ErrorKind, message: String) {
        self.diags.error(code, message, None);
    }

    fn maps(&mut self, settings: &Settings) {
        for (key, mode) in &settings.modes {
            let name = match key {
                ModeKey::General => "general".to_owned(),
                ModeKey::Mode(mode) => mode.name(Lang::EnUs).to_owned(),
            };
            if let ModeKey::Mode(mode) = key
                && mode.only_in_ow1()
            {
                self.error(
                    ErrorKind::SettingsOw1Only,
                    format!("the game mode `{name}` only exists in Overwatch 1"),
                );
            }
            let no_maps = match &mode.maps {
                None => true,
                Some(MapSelection::Enabled(maps)) => maps.is_empty(),
                Some(MapSelection::Disabled(_)) => false,
            };
            if mode.enabled && no_maps && *key != ModeKey::General {
                self.error(
                    ErrorKind::SettingsNoMaps,
                    format!(
                        "the game mode `{name}` has no map; the game refuses it with \"No Map Selected\""
                    ),
                );
            }
            let Some(maps) = &mode.maps else { continue };
            let (MapSelection::Enabled(maps) | MapSelection::Disabled(maps)) = maps;
            for map in maps {
                let map_name = map.map.name(Lang::EnUs);
                if map.map.only_in_ow1() {
                    self.error(
                        ErrorKind::SettingsOw1Only,
                        format!("the map `{map_name}` only exists in Overwatch 1"),
                    );
                } else if !can_play(*key, map.map.gamemodes()) {
                    self.error(
                        ErrorKind::SettingsUnknownMap,
                        format!("`{map_name}` is not a map of the game mode `{name}`"),
                    );
                }
            }
        }
    }

    fn values(&mut self, settings: &Settings) {
        if let Some(main) = &settings.main {
            for (key, text) in [
                ("modeName", &main.mode_name),
                ("description", &main.description),
            ] {
                self.string("main", &[&ow_workshop_data::settings::MAIN], key, text);
            }
        }
        if let Some(lobby) = &settings.lobby {
            self.value_map(
                "lobby",
                &[&ow_workshop_data::settings::LOBBY],
                &lobby.values,
            );
        }
        for (key, mode) in &settings.modes {
            self.mode_values(*key, mode);
        }
        for (team, heroes) in &settings.heroes {
            let team = match team {
                HeroTeam::AllTeams => "all teams",
                HeroTeam::Team1 => "team 1",
                HeroTeam::Team2 => "team 2",
            };
            self.value_map(
                &format!("heroes ({team})"),
                &[&ow_workshop_data::settings::HERO_GENERAL],
                &heroes.general,
            );
            for (hero, values) in &heroes.per_hero {
                let place = format!("heroes ({team}) {}", hero.name(Lang::EnUs));
                self.value_map(&place, &hero_sections(*hero), values);
            }
        }
    }

    fn mode_values(&mut self, key: ModeKey, mode: &ModeSettings) {
        let name = match key {
            ModeKey::General => "general",
            ModeKey::Mode(mode) => mode.name(Lang::EnUs),
        };
        self.value_map(
            &format!("modes ({name})"),
            &mode_sections(key),
            &mode.values,
        );
    }

    fn value_map(&mut self, place: &str, sections: &[&Section], values: &Values) {
        for (key, value) in values.iter() {
            match value {
                SettingValue::Number(n) | SettingValue::Percent(n) => {
                    self.number(place, sections, key, *n);
                }
                SettingValue::Str(text) => self.string(place, sections, key, text),
                _ => {}
            }
        }
    }

    fn number(&mut self, place: &str, sections: &[&Section], key: &str, n: f64) {
        let Some(setting) = find(sections, key) else {
            return;
        };
        let Some(problem) = number_problem(setting.kind, setting.min, setting.max, n) else {
            return;
        };
        let (code, message) = match problem {
            NumberProblem::NotWhole => (
                ErrorKind::SettingsNotInteger,
                format!("{n} is not a whole number"),
            ),
            NumberProblem::TooSmall(min) => (
                ErrorKind::SettingsOutOfRange,
                format!("{n} is below the minimum {min}"),
            ),
            NumberProblem::TooLarge(max) => (
                ErrorKind::SettingsOutOfRange,
                format!("{n} is above the maximum {max}"),
            ),
        };
        self.error(code, format!("{place}: `{key}`: {message}"));
    }

    fn string(&mut self, place: &str, sections: &[&Section], key: &str, text: &str) {
        let Some(setting) = find(sections, key) else {
            return;
        };
        let Kind::String {
            max_chars,
            max_bytes,
        } = setting.kind
        else {
            return;
        };
        if let Some(problem) = string_problem(max_chars, max_bytes, text) {
            self.error(
                ErrorKind::SettingsStringTooLong,
                format!("{place}: `{key}` {problem}"),
            );
        }
    }

    /// Warns when the extensions cost more points than the settings leave.
    fn extension_points(&mut self, settings: &Settings, max_slots: u32) {
        let spent = spent_points(settings);
        let available = available_points(settings, max_slots);
        if spent > available {
            self.diags.warning(
                Warning::ExtensionPoints,
                format!(
                    "the extensions cost {spent} points, but the settings give {available}; \
                     free up lobby slots or play only workshop maps"
                ),
                None,
            );
        }
    }
}

/// Whether a map that is played in `modes` can be listed for `key`.
fn can_play(key: ModeKey, modes: &[Gamemode]) -> bool {
    match key {
        ModeKey::Mode(mode) => modes.contains(&mode),
        // The shared settings have no map list.
        ModeKey::General => false,
    }
}

fn spent_points(settings: &Settings) -> u32 {
    let mut seen: Vec<Extension> = Vec::new();
    for extension in &settings.extensions {
        if !seen.contains(extension) {
            seen.push(*extension);
        }
    }
    seen.iter().map(|e| e.points()).sum()
}

fn available_points(settings: &Settings, max_slots: u32) -> u32 {
    let mut available = 0;
    if settings.lobby.is_some() {
        available += POINTS_PER_FREE_SLOT * (MAX_SLOTS - max_slots);
    }
    if only_workshop_maps(settings) {
        available += WORKSHOP_MAPS_POINTS;
    }
    available
}

/// Whether the played modes have map lists, and those only enable workshop maps.
///
/// Modes without a map list don't change the answer.
fn only_workshop_maps(settings: &Settings) -> bool {
    let mut any = false;
    for (_, mode) in settings.modes.iter().filter(|(_, mode)| mode.enabled) {
        match &mode.maps {
            Some(MapSelection::Disabled(_)) => return false,
            Some(MapSelection::Enabled(maps)) => {
                if maps.is_empty() || !maps.iter().all(|m| m.map.is_workshop_map()) {
                    return false;
                }
                any = true;
            }
            None => {}
        }
    }
    any
}

#[cfg(test)]
mod tests;
