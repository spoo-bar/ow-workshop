//! The settings block, end to end, against the reference fixtures.

mod common;

use common::assert_workshop_code_eq;
use ow_workshop::prelude::*;
use ow_workshop_data::{Extension, Gamemode, Map};

const SHION: &str = include_str!("fixtures/reference/shionSettings.txt");
const EXTENSIONS: &str = include_str!("fixtures/reference/extensions.txt");

#[derive(GlobalVars)]
struct Globals {
    round: Global<Num>,
    flag: Global<Bool>,
    who: Global<Hero>,
}

struct Mod(Settings);

impl Workshop for Mod {
    type Globals = Globals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &Globals, (): &()) {
        ws.rule("Rule 1", OngoingGlobal, |r, _| {
            r.actions(|b| b.set(g.round, 1));
        });
    }

    fn settings(&self) -> Settings {
        self.0.clone()
    }
}

fn settings_of(settings: Settings) -> String {
    let code = Mod(settings).compile(Lang::EnUs).expect("compiles").code;
    let end = code
        .find("variables {")
        .expect("the variables block follows the settings");
    code[..end].to_owned()
}

#[test]
fn shion_settings_match_the_reference() {
    let shion = Values::new()
        .with("secondaryFireCooldown%", SettingValue::Percent(113.0))
        .with("ability1Cooldown%", SettingValue::Percent(81.0))
        .with("ability1Distance%", SettingValue::Percent(110.0))
        .with("ability2Cooldown%", SettingValue::Percent(86.0))
        .with("ability2Kb%", SettingValue::Percent(119.0))
        .with("ability2Duration%", SettingValue::Percent(83.0))
        .with("ability2Speed%", SettingValue::Percent(109.0))
        .with("enableInfiniteUlt", true);
    let settings = Settings::default()
        .with_mode(ModeKey::Mode(Gamemode::Skirmish), ModeSettings::default())
        .with_heroes(
            HeroTeam::AllTeams,
            HeroSettings::default().hero(Hero::Shion, shion),
        );
    assert_workshop_code_eq(&settings_of(settings), SHION);
}

#[test]
fn extensions_settings_match_the_reference() {
    let settings = Settings::default()
        .with_lobby(Lobby::default().team1_slots(0).team2_slots(0))
        .with_mode(
            ModeKey::Mode(Gamemode::TeamDeathmatch),
            ModeSettings::default().maps(MapSelection::Enabled(
                [
                    Map::WorkshopChamber,
                    Map::WorkshopExpanse,
                    Map::WorkshopGreenScreen,
                    Map::WorkshopIsland,
                ]
                .map(MapRef::new)
                .to_vec(),
            )),
        )
        .with_extension(Extension::BeamEffects)
        .with_extension(Extension::BeamSounds)
        .with_extension(Extension::BuffAndDebuffSounds)
        .with_extension(Extension::SpawnMoreDummyBots);
    let expected = &EXTENSIONS[..EXTENSIONS.find("variables {").expect("variables block")];
    assert_workshop_code_eq(&settings_of(settings), expected);
}

#[test]
fn the_block_comes_before_the_rules_and_ends_with_a_blank_line() {
    let settings =
        Settings::default().with_mode(ModeKey::Mode(Gamemode::Skirmish), ModeSettings::default());
    let code = Mod(settings).compile(Lang::EnUs).expect("compiles").code;
    assert!(
        code.starts_with(
            "settings\n{\n\tmodes\n\t{\n\t\tSkirmish\n\t\t{\n\t\t}\n\t}\n}\n\nvariables {"
        ),
        "{code}"
    );
}

#[test]
fn invalid_settings_fail_the_compile() {
    let settings = Settings::default().with_extension(Extension::Projectiles);
    let err = Mod(settings).compile(Lang::EnUs).expect_err("no modes");
    assert!(err.0.iter().any(|d| d.code == "settings-no-modes"), "{err}");
}

#[test]
fn extension_points_are_a_warning() {
    let settings = Settings::default()
        .with_mode(ModeKey::Mode(Gamemode::Skirmish), ModeSettings::default())
        .with_extension(Extension::Projectiles);
    let out = Mod(settings).compile(Lang::EnUs).expect("compiles");
    let codes: Vec<_> = out.warnings.iter().map(|w| w.code).collect();
    assert_eq!(codes, ["extension-points"]);
}

mod workshop_values {
    use super::*;

    struct Values;

    impl Workshop for Values {
        type Globals = Globals;
        type Players = ();

        fn build(&self, ws: &mut Builder, g: &Globals, (): &()) {
            ws.rule("workshop settings", OngoingGlobal, |r, _| {
                r.actions(|b| {
                    b.set(g.flag, workshop_setting::toggle("", "boolean", true));
                    b.set(g.round, workshop_setting::int("int", "int", 3));
                    b.set(
                        g.round,
                        workshop_setting::int("int", "unsigned int", 3).unsigned(),
                    );
                    b.set(
                        g.round,
                        workshop_setting::real("float", "float<2.5;19>", 3.5).range(2.5..=19.0),
                    );
                    b.set(g.who, workshop_setting::hero("hero", "hero", Hero::Ana));
                    b.set(
                        g.who,
                        workshop_setting::hero("hero", "hero", Hero::WreckingBall).sort(3),
                    );
                    b.set(
                        g.round,
                        workshop_setting::combo("enum", "enum", ["a", "b\nc\nd"], 0),
                    );
                    b.set(
                        g.round,
                        workshop_setting::combo("<txC00000000008E02>", "{x}", ["a:b"], 0),
                    );
                });
            });
        }
    }

    #[test]
    fn matches_the_reference_lines() {
        let code = Values.compile(Lang::EnUs).expect("compiles").code;
        for line in [
            r#"Workshop Setting Toggle(Custom String("　"), Custom String("boolean"), True, 0)"#,
            r#"Workshop Setting Integer(Custom String("int"), Custom String("int"), 3, -999999999999, 999999999999, 0)"#,
            r#"Workshop Setting Integer(Custom String("int"), Custom String("unsigned int"), 3, 0, 999999999999, 0)"#,
            "Workshop Setting Real(Custom String(\"float\"), Custom String(\"float<2.5\u{E003B}19>\"), 3.5, 2.5, 19, 0)",
            r#"Workshop Setting Hero(Custom String("hero"), Custom String("hero"), Ana, 0)"#,
            "Workshop Setting Hero(Custom String(\"hero\"), Custom String(\"hero\u{1}\"), Wrecking Ball, 3)",
            r#"Workshop Setting Combo(Custom String("enum"), Custom String("enum"), 0, Array(Custom String("a"), Custom String("b\nc\nd")), 0)"#,
            "Workshop Setting Combo(Custom String(\"<txC00000000008E02>\"), Custom String(\"\u{E007B}x\u{E007D}\"), 0, Array(Custom String(\"a\u{E003A}b\")), 0)",
        ] {
            assert!(code.contains(line), "missing {line}\nin\n{code}");
        }
    }

    #[test]
    fn a_hundred_and_twenty_nine_settings_fail() {
        struct Many;

        impl Workshop for Many {
            type Globals = Globals;
            type Players = ();

            fn build(&self, ws: &mut Builder, g: &Globals, (): &()) {
                ws.rule("many", OngoingGlobal, |r, _| {
                    r.actions(|b| {
                        for i in 0..129 {
                            b.set(g.round, workshop_setting::int("c", &format!("n{i}"), 0));
                        }
                    });
                });
            }
        }

        let err = Many.compile(Lang::EnUs).expect_err("too many");
        assert!(err.0.iter().any(|d| d.code == "too-many-workshop-settings"));
    }
}
