//! The settings block, end to end, against the reference fixtures.

mod common;

use common::assert_workshop_code_eq;
use ow_workshop::prelude::*;
use ow_workshop_data::{Extension, Gamemode, Hero, Map};

const SHION: &str = include_str!("fixtures/reference/shionSettings.txt");
const EXTENSIONS: &str = include_str!("fixtures/reference/extensions.txt");

#[derive(GlobalVars)]
struct Globals {
    round: Global<Num>,
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
    let code = Mod(Settings::default().with_extension(Extension::Projectiles))
        .compile(Lang::EnUs)
        .expect("compiles")
        .code;
    assert!(
        code.starts_with("settings\n{\n\textensions\n\t{\n\t\tProjectiles\n\t}\n}\n\nvariables {"),
        "{code}"
    );
}
