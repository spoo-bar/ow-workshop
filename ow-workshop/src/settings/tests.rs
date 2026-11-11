use super::*;

#[test]
fn values_keep_order_and_replace_in_place() {
    let mut values = Values::new();
    values.set("b", 1);
    values.set("a", true);
    values.set("b", "x");
    assert_eq!(
        values.iter().collect::<Vec<_>>(),
        [
            ("b", &SettingValue::Str("x".into())),
            ("a", &SettingValue::Bool(true))
        ]
    );
    assert_eq!(values.len(), 2);
}

#[test]
fn setters_store_typed_values() {
    let mode = ModeSettings::default()
        .respawn_time_percent(50)
        .enable_kill_cam(false)
        .hero_limit("off");
    assert_eq!(
        mode.values.get("respawnTime%"),
        Some(&SettingValue::Percent(50.0))
    );
    assert_eq!(
        mode.values.get("enableKillCam"),
        Some(&SettingValue::Bool(false))
    );
    assert_eq!(
        mode.values.get("heroLimit"),
        Some(&SettingValue::Choice("off".into()))
    );
    let lobby = Lobby::default().team1_slots(3).map_rotation("paused");
    assert_eq!(
        lobby.values.get("team1Slots"),
        Some(&SettingValue::Number(3.0))
    );
}

#[test]
fn unknown_keys_are_kept() {
    let lobby = Lobby::default().raw("unknownSetting", "abc");
    assert_eq!(
        lobby.values.get("unknownSetting"),
        Some(&SettingValue::Str("abc".into()))
    );
}

#[test]
fn modes_replace_by_key() {
    let settings = Settings::default()
        .with_mode(ModeKey::General, ModeSettings::default())
        .with_mode(ModeKey::General, ModeSettings::default().disabled());
    assert_eq!(settings.modes.len(), 1);
    assert!(!settings.modes[0].1.enabled);
}

#[test]
fn extensions_are_unique() {
    let settings = Settings::default()
        .with_extension(Extension::BeamEffects)
        .with_extension(Extension::BeamEffects);
    assert_eq!(settings.extensions, [Extension::BeamEffects]);
}

#[cfg(feature = "serde")]
mod json {
    use super::*;

    fn parse(json: &str) -> Result<Settings, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }

    #[test]
    fn main_cannot_be_set_from_json() {
        let err = parse(r#"{"main": {"modeName": "Tower", "description": "Hold"}}"#)
            .expect_err("rejected");
        assert!(err.contains("Workshop::NAME"), "{err}");
    }

    #[test]
    fn builder_matches_json() {
        let built = Settings::default()
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
                    .maps(MapSelection::Enabled(vec![
                        MapRef::new(Map::TempleOfAnubis).variant("night"),
                    ]))
                    .capture_speed_percent(60)
                    .raw("unknown value", 60),
            )
            .with_mode(
                ModeKey::General,
                ModeSettings::default()
                    .respawn_time_percent(100)
                    .spawn_health_packs("disabled"),
            )
            .with_heroes(
                HeroTeam::Team1,
                HeroSettings::default()
                    .selection(HeroSelection::Enabled(vec![Hero::Ashe, Hero::Bastion])),
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
            .with_workshop_value("My hero", WorkshopValue::Hero(Hero::Ana))
            .with_extension(Extension::BeamEffects);
        let parsed = parse(
            r#"{
                "lobby": {
                    "mapRotation": "paused",
                    "spectatorSlots": 6,
                    "swapTeamsAfterMatch": false,
                    "unknownSetting": "abc"
                },
                "gamemodes": {
                    "assault": {
                        "enabledMaps": [{"map": "templeOfAnubis", "variants": ["night"]}],
                        "captureSpeed%": 60,
                        "unknown value": 60
                    },
                    "general": {"respawnTime%": 100, "spawnHealthPacks": "disabled"}
                },
                "heroes": {
                    "team1": {"enabledHeroes": ["ashe", "bastion"]},
                    "allTeams": {"ashe": {"ammoClipSize%": 200}}
                },
                "workshop": {
                    "My toggle": true,
                    "My combo": {"combo": 2},
                    "My hero": {"hero": "ana"}
                },
                "extensions": ["beamEffects"]
            }"#,
        )
        .expect("parses");
        assert_eq!(parsed, built);
    }

    #[test]
    fn empty_json_is_default() {
        assert_eq!(parse("{}"), Ok(Settings::default()));
    }

    #[test]
    fn modes_alias_and_disabled_mode() {
        let settings = parse(r#"{"modes": {"skirmish": {"enabled": false}}}"#).expect("parses");
        assert_eq!(
            settings.modes,
            [(
                ModeKey::Mode(Gamemode::Skirmish),
                ModeSettings::default().disabled()
            )]
        );
    }

    #[test]
    fn legacy_hero_keys() {
        let settings =
            parse(r#"{"heroes": {"allTeams": {"mccree": {"x": 1}, "hammond": {"x": 2}}}}"#)
                .expect("parses");
        let heroes: Vec<Hero> = settings.heroes[0]
            .1
            .per_hero
            .iter()
            .map(|(h, _)| *h)
            .collect();
        assert_eq!(heroes, [Hero::Cassidy, Hero::WreckingBall]);
    }

    #[test]
    fn rejects_both_map_lists() {
        let err = parse(r#"{"gamemodes": {"assault": {"enabledMaps": [], "disabledMaps": []}}}"#)
            .expect_err("both lists");
        assert!(err.contains("both enabledMaps and disabledMaps"), "{err}");
    }

    #[test]
    fn rejects_both_hero_lists() {
        let err = parse(r#"{"heroes": {"team1": {"enabledHeroes": [], "disabledHeroes": []}}}"#)
            .expect_err("both lists");
        assert!(
            err.contains("both enabledHeroes and disabledHeroes"),
            "{err}"
        );
    }

    #[test]
    fn rejects_unknown_names() {
        for (json, needle) in [
            (r#"{"gamemodes": {"nope": {}}}"#, "unknown game mode `nope`"),
            (
                r#"{"gamemodes": {"assault": {"enabledMaps": ["nope"]}}}"#,
                "unknown map `nope`",
            ),
            (
                r#"{"heroes": {"team1": {"nope": {}}}}"#,
                "unknown hero `nope`",
            ),
            (r#"{"heroes": {"nope": {}}}"#, "unknown hero team `nope`"),
            (r#"{"extensions": ["nope"]}"#, "unknown extension `nope`"),
        ] {
            let err = parse(json).expect_err(json);
            assert!(err.contains(needle), "{json}: {err}");
        }
    }
}
