use ow_workshop_data::{Extension, Gamemode, Hero, Lang, Map};

use crate::emit::emit;
use crate::ir::Program;
use crate::settings::{
    HeroSelection, HeroSettings, HeroTeam, Lobby, Main, MapRef, MapSelection, ModeKey,
    ModeSettings, SettingValue, Settings, Values, WorkshopValue,
};

fn print(settings: Settings, lang: Lang) -> String {
    let program = Program {
        settings,
        ..Program::default()
    };
    emit(&program, lang, None).expect("prints")
}

/// The block from the design: every section, unknown keys, and the hero list last.
fn sample() -> Settings {
    Settings {
        main: Some(Main {
            mode_name: "Ｔｏｗｅｒ Ｍｅｉｆｅｎｓｅ".into(),
            description: "Hold the tower".into(),
        }),
        ..Settings::default()
    }
    .with_lobby(
        Lobby::default()
            .map_rotation("paused")
            .spectator_slots(6)
            .swap_teams_after_match(false)
            .raw("unknownSetting", "abc"),
    )
    .with_mode(
        ModeKey::Mode(Gamemode::Assault),
        ModeSettings::default()
            .maps(MapSelection::Enabled(vec![MapRef::new(
                Map::TempleOfAnubis,
            )]))
            .capture_speed_percent(60),
    )
    .with_mode(
        ModeKey::General,
        ModeSettings::default().respawn_time_percent(100),
    )
    .with_heroes(
        HeroTeam::Team1,
        HeroSettings::default().selection(HeroSelection::Enabled(vec![Hero::Ashe, Hero::Bastion])),
    )
    .with_heroes(
        HeroTeam::AllTeams,
        HeroSettings::default().hero(
            Hero::Ashe,
            Values::new().with("ammoClipSize%", SettingValue::Percent(200.0)),
        ),
    )
    .with_workshop_value("My toggle", WorkshopValue::Bool(true))
    .with_workshop_value("My combo", WorkshopValue::Combo(2))
    .with_extension(Extension::BeamEffects)
}

#[test]
fn en_us_sample() {
    assert_eq!(
        print(sample(), Lang::EnUs),
        "settings
{
\tmain
\t{
\t\tMode Name: \"Ｔｏｗｅｒ Ｍｅｉｆｅｎｓｅ\"
\t\tDescription: \"Hold the tower\"
\t}
\tlobby
\t{
\t\tMap Rotation: Paused
\t\tMax Spectators: 6
\t\tSwap Teams After Match: No
\t\tunknownSetting: abc
\t}
\tmodes
\t{
\t\tAssault
\t\t{
\t\t\tenabled maps
\t\t\t{
\t\t\t\tTemple of Anubis
\t\t\t}
\t\t\tCapture Speed Modifier: 60%
\t\t}
\t\tGeneral
\t\t{
\t\t\tRespawn Time Scalar: 100%
\t\t}
\t}
\theroes
\t{
\t\tTeam 1
\t\t{
\t\t\tenabled heroes
\t\t\t{
\t\t\t\tAshe
\t\t\t\tBastion
\t\t\t}
\t\t}
\t\tGeneral
\t\t{
\t\t\tAshe
\t\t\t{
\t\t\t\tAmmunition Clip Size Scalar: 200%
\t\t\t}
\t\t}
\t}
\tworkshop
\t{
\t\tMy toggle: On
\t\tMy combo: [2]
\t}
\textensions
\t{
\t\tBeam Effects
\t}
}

"
    );
}

#[test]
fn de_de_sample() {
    assert_eq!(
        print(sample(), Lang::DeDe),
        "einstellungen
{
\thauptregeln
\t{
\t\tModusname: \"Ｔｏｗｅｒ Ｍｅｉｆｅｎｓｅ\"
\t\tBeschreibung: \"Hold the tower\"
\t}
\tLobby
\t{
\t\tKartenrotation: Pausiert
\t\tMax. Anzahl Zuschauer: 6
\t\tSeitenwechsel nach Match: Nein
\t\tunknownSetting: abc
\t}
\tmodi
\t{
\t\tAngriff
\t\t{
\t\t\tverfügbare karten
\t\t\t{
\t\t\t\tTempel des Anubis
\t\t\t}
\t\t\tEroberungsgeschwindigkeit: 60%
\t\t}
\t\tAllgemein
\t\t{
\t\t\tWiederbelebungszeit: 100%
\t\t}
\t}
\thelden
\t{
\t\tTeam 1
\t\t{
\t\t\tverfügbare helden
\t\t\t{
\t\t\t\tAshe
\t\t\t\tBastion
\t\t\t}
\t\t}
\t\tAllgemein
\t\t{
\t\t\tAshe
\t\t\t{
\t\t\t\tMagazingröße: 200%
\t\t\t}
\t\t}
\t}
\tworkshop
\t{
\t\tMy toggle: Ein
\t\tMy combo: [2]
\t}
\tErweiterungen
\t{
\t\tStrahleffekte
\t}
}

"
    );
}

#[test]
fn empty_settings_print_nothing() {
    assert_eq!(print(Settings::default(), Lang::EnUs), "");
}

#[test]
fn only_modes_and_heroes_sections_appear_when_set() {
    let settings =
        Settings::default().with_mode(ModeKey::Mode(Gamemode::Skirmish), ModeSettings::default());
    assert_eq!(
        print(settings, Lang::EnUs),
        "settings\n{\n\tmodes\n\t{\n\t\tSkirmish\n\t\t{\n\t\t}\n\t}\n}\n\n"
    );
}

#[test]
fn disabled_mode_has_no_body() {
    let settings = Settings::default().with_mode(
        ModeKey::Mode(Gamemode::Assault),
        ModeSettings::default().disabled(),
    );
    assert!(print(settings, Lang::EnUs).contains("\t\tdisabled Assault\n"));
}

#[test]
fn disabled_map_list_with_variants() {
    let settings = Settings::default().with_mode(
        ModeKey::Mode(Gamemode::Control),
        ModeSettings::default().maps(MapSelection::Disabled(vec![
            MapRef::new(Map::LijiangTower)
                .variant("night")
                .variant("unknown"),
        ])),
    );
    let text = print(settings, Lang::EnUs);
    assert!(text.contains("\t\t\tdisabled maps\n"), "{text}");
    assert!(
        text.contains("\t\t\t\tLijiang Tower 972777519512068153 unknown\n"),
        "{text}"
    );
}

#[test]
fn booleans_follow_the_setting_kind() {
    let settings = Settings::default()
        .with_lobby(
            Lobby::default()
                .allow_players_in_queue(true)
                .enable_match_voice_chat(false)
                .raw("unknownFlag", true),
        )
        .with_mode(
            ModeKey::General,
            ModeSettings::default().enable_kill_cam(false),
        );
    let text = print(settings, Lang::EnUs);
    assert!(
        text.contains("Allow Players Who Are In Queue: Yes\n"),
        "{text}"
    );
    assert!(text.contains("Match Voice Chat: Disabled\n"), "{text}");
    assert!(text.contains("unknownFlag: On\n"), "{text}");
    assert!(text.contains("Kill Cam: Off\n"), "{text}");
}

#[test]
fn unknown_choice_and_value_pass_through() {
    let settings = Settings::default().with_lobby(
        Lobby::default()
            .return_to_lobby("unknown value")
            .raw("weird", SettingValue::Percent(123.0)),
    );
    let text = print(settings, Lang::EnUs);
    assert!(text.contains("Return To Lobby: unknown value\n"), "{text}");
    assert!(text.contains("weird: 123%\n"), "{text}");
}

#[test]
fn renamed_hero_key_resolves() {
    let settings = Settings::default().with_heroes(
        HeroTeam::AllTeams,
        HeroSettings::default().hero(
            Hero::Domina,
            Values::new().with("ability1KB%", SettingValue::Percent(125.0)),
        ),
    );
    let text = print(settings, Lang::EnUs);
    assert!(
        text.contains("Sonic Repulsors Knockback Scalar: 125%\n"),
        "{text}"
    );
}

#[test]
fn extensions_keep_their_order() {
    let settings = Settings::default()
        .with_extension(Extension::Projectiles)
        .with_extension(Extension::BeamEffects);
    let text = print(settings, Lang::EnUs);
    assert!(
        text.contains("\t\tProjectiles\n\t\tBeam Effects\n\t}"),
        "{text}"
    );
}

#[test]
fn hero_list_comes_after_hero_objects() {
    let settings = Settings::default().with_heroes(
        HeroTeam::Team2,
        HeroSettings::default()
            .selection(HeroSelection::Disabled(vec![Hero::Ana]))
            .damage_dealt_percent(50)
            .hero(Hero::Mei, Values::new()),
    );
    let text = print(settings, Lang::EnUs);
    let general = text.find("Damage Dealt").expect("general value");
    let hero = text.find("Mei").expect("hero");
    let list = text.find("disabled heroes").expect("list");
    assert!(general < hero && hero < list, "{text}");
}

#[test]
fn string_settings_are_quoted_and_escaped() {
    let settings = Settings {
        main: Some(Main {
            mode_name: "a \"b\"\nc".into(),
            description: String::new(),
        }),
        ..Settings::default()
    };
    assert!(print(settings, Lang::EnUs).contains("Mode Name: \"a \\\"b\\\"\\nc\"\n"));
}
