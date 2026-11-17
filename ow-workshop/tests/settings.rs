//! The settings block: reference samples, fragments and conflicts.

use ow_workshop::{
    BuildError, Entry, Hero, HeroEntry, HeroSettings, Lobby, LobbySettings, Mode, ModeSettings,
    Profile, Rule, SettingValue as V, SettingsConflict, SettingsError, UnknownSetting, Workshop,
    try_script, utils,
};

/// The `settings { .. }` block of a script as sorted, trimmed lines, without
/// blank lines and braces, so key order does not matter.
fn settings_lines(text: &str) -> Vec<String> {
    let start = text.find("settings\n").expect("settings block");
    let mut depth = 0;
    let mut lines = Vec::new();
    for line in text[start..].lines() {
        let line = line.trim();
        depth += i32::from(line == "{") - i32::from(line == "}");
        if !line.is_empty() && line != "{" && line != "}" {
            lines.push(line.to_owned());
        }
        if depth == 0 && line == "}" {
            break;
        }
    }
    lines.sort();
    lines
}

fn conflict(result: Result<String, BuildError>) -> SettingsConflict {
    match result.unwrap_err() {
        BuildError::Settings(SettingsError::Conflict(conflict)) => conflict,
        other => panic!("expected a conflict, got {other}"),
    }
}

fn unknown(result: Result<String, BuildError>) -> UnknownSetting {
    match result.unwrap_err() {
        BuildError::Settings(SettingsError::Unknown(unknown)) => unknown,
        other => panic!("expected an unknown setting, got {other}"),
    }
}

fn sample(name: &str) -> Vec<String> {
    let path = format!("{}/../examples/samples/{name}", env!("CARGO_MANIFEST_DIR"));
    settings_lines(&std::fs::read_to_string(path).unwrap())
}

macro_rules! workshop {
    ($name:ident, $lobby:expr, $modes:expr, $heroes:expr) => {
        struct $name;
        impl Workshop for $name {
            const NAME: &'static str = "Overchess";
            const DESCRIPTION: &'static str = "Overchess v0.1.0";
            type Globals = ();
            type Players = ();

            fn lobby() -> Lobby {
                $lobby
            }
            fn modes() -> Vec<Mode> {
                $modes
            }
            fn heroes() -> Vec<Hero> {
                $heroes
            }
            fn rules(&self, _: &(), _: &()) -> Vec<Box<dyn Rule>> {
                Vec::new()
            }
        }
    };
}

workshop!(
    SettingsOnly,
    Lobby::new()
        .set("Data Center Preference", "Netherlands")
        .set("Map Rotation", "After A Game")
        .set("Match Voice Chat", V::enabled(true))
        .set("Pause Game On Player Disconnect", V::yes_no(true))
        .set("Return To Lobby", "After A Game")
        .set("Team Balancing", "After A Game"),
    vec![
        // Declared first, printed last.
        Mode::new("General")
            .set("Health Pack Respawn Time Scalar", V::percent(49))
            .set("Hero Limit", "2 Per Team")
            .set("Limit Roles", "2 Of Each Role Per Team")
            .set("Spawn Health Packs", V::enabled(true))
            .set("Team Overlay", V::enabled(true)),
        Mode::new("Capture the Flag")
            .disabled()
            .set("Damage Interrupts Flag Interaction", V::enabled(true))
            .set("Enable Perks", V::on_off(true))
            .set("Flag Dropped Lock Time", V::decimal(5.0))
            .set("Flag Return Time", V::decimal(4.0))
            .set("Flag Score Respawn Time", V::decimal(15.0))
            .set("Perk Elimination Catchup Level Amount", V::percent(33))
            .set("Perk Generation", V::percent(175))
            .set("Team Needs Flag At Base To Score", V::yes_no(true)),
        Mode::new("Clash")
            .set("Capture Speed Modifier", V::percent(45))
            .set("Perk Elimination Catchup Level Amount", V::percent(46))
            .set("Perk Generation", V::percent(249)),
        Mode::new("Control")
            .set("Perk Elimination Catchup Level Amount", V::percent(47))
            .set("Perk Generation", V::percent(248)),
        Mode::new("Push")
            .set("Perk Elimination Catchup Level Amount", V::percent(47))
            .set("Perk Generation", V::percent(242))
            .set("TS-1 Walk Speed Modifier", V::percent(101)),
    ],
    vec![
        Hero::general()
            .set("Ability Cooldown Time", V::percent(101))
            .set("Ammunition Clip Size Scalar", V::percent(106))
            .set("Ultimate Duration", V::percent(252))
            .set("Ultimate Generation", V::percent(215))
            .set("Ultimate Generation - Combat", V::percent(216))
            .set("Ultimate Generation - Passive", V::percent(208)),
    ]
);

#[test]
fn reproduces_settings_only_sample() {
    let script = try_script(&SettingsOnly, Profile::Release).unwrap();
    // The sample has no `main` block; ours always has one.
    let mut got = settings_lines(&script);
    got.retain(|line| {
        ![
            "main",
            "Mode Name: \"Overchess\"",
            "Description: \"Overchess v0.1.0\"",
        ]
        .contains(&line.as_str())
    });
    assert_eq!(got, sample("settings_only.owmod"));
}

#[test]
fn main_block_comes_from_name_and_description() {
    let script = try_script(&SettingsOnly, Profile::Release).unwrap();
    assert!(script.contains(
        "\tmain\n\t{\n\t\tDescription: \"Overchess v0.1.0\"\n\t\tMode Name: \"Overchess\"\n\t}\n"
    ));
}

#[test]
fn general_mode_is_printed_last() {
    let script = try_script(&SettingsOnly, Profile::Release).unwrap();
    assert!(script.find("\t\tPush\n").unwrap() < script.find("\t\tGeneral\n").unwrap());
}

workshop!(
    Overchess,
    Lobby::new()
        .set("Max Team 1 Players", 1)
        .set("Max Team 2 Players", 1),
    vec![
        Mode::new("Clash").disabled(),
        Mode::new("Control")
            .disabled_map(
                "Antarctic Peninsula",
                ["972777519512068154", "972777519512068153"]
            )
            .disabled_map(
                "Busan",
                [
                    "972777519512068153",
                    "972777519512068154",
                    "972777519512068194"
                ]
            )
            .set("Capture Speed Modifier", V::percent(500))
            .set("Game Mode Start", "Manual")
            .set("Limit Roles", V::on_off(false))
            .set("Score To Win", V::int(3)),
        Mode::new("Escort").disabled(),
    ],
    vec![Hero::general().with(utils::UnlimitedFlight)]
);

#[test]
fn overchess_settings_are_a_subset_of_the_sample() {
    let got = settings_lines(&try_script(&Overchess, Profile::Release).unwrap());
    let expected = sample("overchess.owmod");
    for line in &got {
        assert!(expected.contains(line), "not in the sample: {line:?}");
    }
    for line in [
        "disabled Clash",
        "Busan 972777519512068153 972777519512068154 972777519512068194",
        "Frenetic Flight Unlimited Fuel: On",
    ] {
        assert!(got.iter().any(|l| l == line), "missing {line:?}");
    }
}

/// A fragment usable in every scope, setting one value.
struct First(&'static str);
struct Second(&'static str);

macro_rules! fragment {
    ($ty:ident) => {
        impl LobbySettings for $ty {
            fn entries(&self) -> Vec<Entry> {
                vec![Entry::new("Team Balancing", self.0)]
            }
        }
        impl ModeSettings for $ty {
            fn entries(&self) -> Vec<Entry> {
                vec![Entry::new("Perk Generation", self.0)]
            }
        }
        impl HeroSettings for $ty {
            fn entries(&self) -> Vec<HeroEntry> {
                vec![HeroEntry::of("Ana", "Health", self.0)]
            }
        }
    };
}
fragment!(First);
fragment!(Second);

workshop!(
    Agreeing,
    Lobby::new(),
    vec![
        Mode::new("Clash").with(First("1%")),
        Mode::new("Clash").with(Second("1%"))
    ],
    Vec::new()
);

workshop!(
    ModeConflict,
    Lobby::new(),
    vec![Mode::new("Clash").with(First("1%")).with(Second("2%"))],
    Vec::new()
);

workshop!(
    HeroConflict,
    Lobby::new(),
    Vec::new(),
    vec![Hero::general().with(First("1%")).with(Second("2%"))]
);

workshop!(
    LobbyConflict,
    Lobby::new().with(First("A")).set("Team Balancing", "B"),
    Vec::new(),
    Vec::new()
);

#[test]
fn equal_values_from_two_fragments_merge_into_one_block() {
    let script = try_script(&Agreeing, Profile::Release).unwrap();
    assert_eq!(script.matches("Perk Generation: 1%").count(), 1);
    assert_eq!(script.matches("\t\tClash\n").count(), 1);
}

#[test]
fn different_values_are_a_conflict_naming_both_sources() {
    let error = conflict(try_script(&ModeConflict, Profile::Release));
    assert_eq!(error.scope, "modes > Clash");
    assert_eq!(error.key, "Perk Generation");
    let message = error.to_string();
    assert!(
        message.contains("First") && message.contains("Second"),
        "{message}"
    );
    assert!(
        message.contains("`1%`") && message.contains("`2%`"),
        "{message}"
    );
}

#[test]
fn conflicts_are_found_in_hero_and_lobby_scopes() {
    let hero = conflict(try_script(&HeroConflict, Profile::Release));
    assert_eq!(hero.scope, "heroes > General > Ana");
    let lobby = conflict(try_script(&LobbyConflict, Profile::Release));
    assert!(lobby.first.0.ends_with("First"));
    assert_eq!(lobby.second.0, "Lobby::set");
}

#[test]
#[should_panic(expected = "conflicting settings in modes > Clash")]
fn script_panics_on_conflict() {
    let _ = ow_workshop::script(&ModeConflict, Profile::Release);
}

workshop!(
    BadLobbyKey,
    Lobby::new().set("Max Team 1 Plyers", 1),
    Vec::new(),
    Vec::new()
);
workshop!(
    BadModeKey,
    Lobby::new(),
    vec![Mode::new("Clash").set("Max Team 1 Players", 3)],
    Vec::new()
);
workshop!(
    BadModeName,
    Lobby::new(),
    vec![Mode::new("Chess")],
    Vec::new()
);
workshop!(
    BadHeroKey,
    Lobby::new(),
    Vec::new(),
    vec![Hero::general().hero("Jetpack Cat", "Perk Generation", "1%")]
);
workshop!(
    BadHeroName,
    Lobby::new(),
    Vec::new(),
    vec![Hero::general().hero("Jetpack Dog", "Health", "1%")]
);

#[test]
fn unknown_names_are_errors_naming_the_scope() {
    let lobby = unknown(try_script(&BadLobbyKey, Profile::Release));
    assert_eq!(
        (lobby.scope.as_str(), lobby.name.as_str()),
        ("lobby", "Max Team 1 Plyers")
    );
    let mode = unknown(try_script(&BadModeKey, Profile::Release));
    assert_eq!(
        (mode.scope.as_str(), mode.name.as_str()),
        ("modes > Clash", "Max Team 1 Players")
    );
    let name = unknown(try_script(&BadModeName, Profile::Release));
    assert_eq!(
        (name.scope.as_str(), name.name.as_str()),
        ("modes", "Chess")
    );
    let key = unknown(try_script(&BadHeroKey, Profile::Release));
    assert_eq!(key.scope, "heroes > General > Jetpack Cat");
    let hero = unknown(try_script(&BadHeroName, Profile::Release));
    assert_eq!(hero.name, "Jetpack Dog");
}
