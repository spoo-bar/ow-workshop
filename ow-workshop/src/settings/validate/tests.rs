use ow_workshop_data::{Extension, Gamemode, Map};

use super::*;
use crate::ir::{Diagnostic, Severity};
use crate::settings::{Lobby, Main, MapRef};

fn check_with(settings: &mut Settings, options: &CompileOptions) -> Vec<Diagnostic> {
    let mut diags = Diagnostics::default();
    validate(settings, options, &mut diags);
    diags.0
}

/// Everything reported, including `settings-no-maps`.
fn check_all(mut settings: Settings) -> Vec<(Severity, &'static str)> {
    check_with(&mut settings, &CompileOptions::default())
        .iter()
        .map(|d| (d.severity, d.code()))
        .collect()
}

/// Like `check_all`, minus `settings-no-maps`: most tests build modes without maps.
fn check(settings: Settings) -> Vec<(Severity, &'static str)> {
    let mut all = check_all(settings);
    all.retain(|(_, code)| *code != "settings-no-maps");
    all
}

fn errors(settings: Settings) -> Vec<&'static str> {
    check(settings)
        .into_iter()
        .filter(|(severity, _)| *severity == Severity::Error)
        .map(|(_, code)| code)
        .collect()
}

fn main() -> Main {
    Main {
        mode_name: "Mode".to_owned(),
        description: "About it".to_owned(),
    }
}

fn tdm() -> ModeKey {
    ModeKey::Mode(Gamemode::TeamDeathmatch)
}

fn with_tdm(settings: Settings) -> Settings {
    settings.with_mode(tdm(), ModeSettings::default())
}

fn workshop_maps() -> MapSelection {
    MapSelection::Enabled(vec![
        MapRef::new(Map::WorkshopChamber),
        MapRef::new(Map::WorkshopIsland),
    ])
}

#[test]
fn empty_settings_are_an_error() {
    assert_eq!(errors(Settings::default()), ["settings-no-modes"]);
}

#[test]
fn unchecked_settings_are_not_checked() {
    assert_eq!(check(Settings::unchecked()), []);
}

#[test]
fn enabled_mode_needs_a_map() {
    let errors = |settings: Settings| -> Vec<&'static str> {
        check_all(settings)
            .into_iter()
            .filter(|(severity, _)| *severity == Severity::Error)
            .map(|(_, code)| code)
            .collect()
    };
    let no_maps = |maps: Option<MapSelection>| {
        let mode = ModeSettings::default();
        let mode = maps.map_or(mode.clone(), |m| mode.maps(m));
        errors(Settings::default().with_mode(tdm(), mode))
    };
    assert_eq!(no_maps(None), ["settings-no-maps"]);
    assert_eq!(
        no_maps(Some(MapSelection::Enabled(vec![]))),
        ["settings-no-maps"]
    );
    assert_eq!(
        no_maps(Some(MapSelection::Disabled(vec![]))),
        [] as [&str; 0]
    );
    // `General` holds shared settings and needs no map, but cannot be the only mode.
    let general = Settings::default().with_mode(ModeKey::General, ModeSettings::default());
    assert_eq!(errors(general), ["settings-no-modes"]);
    let with_general = Settings::default()
        .with_mode(tdm(), ModeSettings::default().maps(workshop_maps()))
        .with_mode(ModeKey::General, ModeSettings::default());
    assert_eq!(errors(with_general), [] as [&str; 0]);
    let disabled = Settings::default()
        .with_mode(tdm(), ModeSettings::default().disabled())
        .with_mode(ModeKey::General, ModeSettings::default());
    assert_eq!(errors(disabled), ["settings-no-modes"]);
}

#[test]
fn modes_must_not_be_empty() {
    let settings = Settings::default().with_extension(Extension::BeamSounds);
    assert_eq!(errors(settings), ["settings-no-modes"]);
    assert_eq!(errors(with_tdm(Settings::default())), [] as [&str; 0]);
}

#[test]
fn map_must_belong_to_the_mode() {
    let wrong = Settings::default().with_mode(
        ModeKey::Mode(Gamemode::Assault),
        ModeSettings::default().maps(MapSelection::Enabled(vec![MapRef::new(Map::Dorado)])),
    );
    assert_eq!(errors(wrong), ["settings-unknown-map"]);
    let right = Settings::default().with_mode(
        ModeKey::Mode(Gamemode::Escort),
        ModeSettings::default().maps(MapSelection::Disabled(vec![MapRef::new(Map::Dorado)])),
    );
    assert_eq!(errors(right), [] as [&str; 0]);
}

#[test]
fn the_shared_settings_have_no_map_list() {
    let settings = Settings::default().with_mode(
        ModeKey::General,
        ModeSettings::default().maps(workshop_maps()),
    );
    assert_eq!(
        errors(settings),
        [
            "settings-no-modes",
            "settings-unknown-map",
            "settings-unknown-map"
        ]
    );
}

#[test]
fn overwatch_1_only_map_and_mode() {
    let map = *Map::ALL
        .iter()
        .find(|m| m.only_in_ow1())
        .expect("an OW1 map");
    // The map's mode may itself be Overwatch 1 only; that is reported too.
    let mode = map.gamemodes()[0];
    let expected = if mode.only_in_ow1() { 2 } else { 1 };
    let settings = Settings::default().with_mode(
        ModeKey::Mode(mode),
        ModeSettings::default().maps(MapSelection::Enabled(vec![MapRef::new(map)])),
    );
    assert_eq!(errors(settings), vec!["settings-ow1-only"; expected]);
    let lucioball =
        Settings::default().with_mode(ModeKey::Mode(Gamemode::Lucioball), ModeSettings::default());
    assert_eq!(errors(lucioball), ["settings-ow1-only"]);
}

#[test]
fn int_settings_need_whole_numbers() {
    let fractional = with_tdm(Settings::default()).with_lobby(Lobby::default().team1_slots(1.5));
    assert_eq!(errors(fractional), ["settings-not-integer"]);
    let whole = with_tdm(Settings::default()).with_lobby(Lobby::default().team1_slots(2));
    assert_eq!(errors(whole), [] as [&str; 0]);
}

#[test]
fn number_limits() {
    let int = Kind::Int;
    assert_eq!(
        number_problem(int, None, None, 2.5),
        Some(NumberProblem::NotWhole)
    );
    assert_eq!(number_problem(Kind::Float, None, None, 2.5), None);
    assert_eq!(number_problem(int, Some(1.0), Some(5.0), 3.0), None);
    assert_eq!(
        number_problem(int, Some(1.0), Some(5.0), 0.0),
        Some(NumberProblem::TooSmall(1.0))
    );
    assert_eq!(
        number_problem(Kind::Percent, Some(0.0), Some(500.0), 501.0),
        Some(NumberProblem::TooLarge(500.0))
    );
    assert_eq!(number_problem(Kind::BoolOnOff, Some(0.0), None, -1.0), None);
}

#[test]
fn string_limits_count_scalars_and_bytes() {
    // The mode name allows 129 bytes; the description 512 characters.
    let name = |text: &str| {
        with_tdm(Settings::default()).with_main(Main {
            mode_name: text.to_owned(),
            description: String::new(),
        })
    };
    assert_eq!(errors(name(&"a".repeat(129))), [] as [&str; 0]);
    assert_eq!(errors(name(&"a".repeat(130))), ["settings-string-too-long"]);
    // 43 three-byte scalars are 129 bytes; 44 are 132.
    assert_eq!(errors(name(&"あ".repeat(43))), [] as [&str; 0]);
    assert_eq!(errors(name(&"あ".repeat(44))), ["settings-string-too-long"]);
    let description = |text: String| {
        with_tdm(Settings::default()).with_main(Main {
            mode_name: String::new(),
            description: text,
        })
    };
    assert_eq!(errors(description("あ".repeat(512))), [] as [&str; 0]);
    assert_eq!(
        errors(description("あ".repeat(513))),
        ["settings-string-too-long"]
    );
}

#[test]
fn slots_are_capped_at_twelve() {
    let lobby = |lobby: Lobby| with_tdm(Settings::default()).with_lobby(lobby);
    assert_eq!(
        errors(lobby(Lobby::default().team1_slots(6).team2_slots(6))),
        [] as [&str; 0]
    );
    assert_eq!(
        errors(lobby(Lobby::default().team1_slots(7).team2_slots(6))),
        ["settings-slots"]
    );
    assert_eq!(
        errors(lobby(Lobby::default().ffa_slots(13))),
        ["settings-slots"]
    );
    // Free-for-all and team slots are not added together.
    assert_eq!(
        errors(lobby(
            Lobby::default().ffa_slots(12).team1_slots(3).team2_slots(3)
        )),
        [] as [&str; 0]
    );
}

#[test]
fn unset_slots_default_to_the_largest_of_the_modes() {
    // Assault seats 5 + 5 by default, so 8 more in team 1 is 13.
    let settings = Settings::default()
        .with_mode(ModeKey::Mode(Gamemode::Assault), ModeSettings::default())
        .with_lobby(Lobby::default().team1_slots(8));
    assert_eq!(errors(settings), ["settings-slots"]);
    // A disabled mode doesn't count.
    let settings = Settings::default()
        .with_mode(
            ModeKey::Mode(Gamemode::Assault),
            ModeSettings::default().disabled(),
        )
        .with_mode(tdm(), ModeSettings::default())
        .with_lobby(Lobby::default().team1_slots(8));
    assert_eq!(errors(settings), [] as [&str; 0]);
}

#[test]
fn best_available_data_center_is_dropped() {
    let mut settings = Settings::default()
        .with_main(main())
        .with_mode(tdm(), ModeSettings::default().maps(workshop_maps()))
        .with_lobby(
            Lobby::default()
                .data_center_preference("bestAvailable")
                .map_rotation("paused"),
        );
    let diagnostics = check_with(&mut settings, &CompileOptions::default());
    assert_eq!(diagnostics, []);
    let lobby = settings.lobby.expect("lobby stays");
    assert_eq!(lobby.values.get("dataCenterPreference"), None);
    assert!(lobby.values.get("mapRotation").is_some());

    let mut settings =
        with_tdm(Settings::default()).with_lobby(Lobby::default().data_center_preference("brazil"));
    check_with(&mut settings, &CompileOptions::default());
    assert!(
        settings
            .lobby
            .expect("lobby")
            .values
            .get("dataCenterPreference")
            .is_some()
    );
}

fn lobby_with_slots(team: u32) -> Lobby {
    Lobby::default().team1_slots(team).team2_slots(team)
}

fn all_extensions(settings: Settings) -> Settings {
    Extension::ALL
        .iter()
        .fold(settings, |settings, e| settings.with_extension(*e))
}

fn warnings(settings: Settings) -> Vec<String> {
    let mut settings = settings;
    check_with(&mut settings, &CompileOptions::default())
        .into_iter()
        .filter(|d| d.severity == Severity::Warning)
        .map(|d| {
            assert_eq!(d.code(), "extension-points");
            d.message
        })
        .collect()
}

#[test]
fn extension_points_with_six_slots_and_workshop_maps() {
    // 4 × (12 - 6) = 24, and 16 more for workshop maps only.
    let base = Settings::default()
        .with_mode(tdm(), ModeSettings::default().maps(workshop_maps()))
        .with_lobby(lobby_with_slots(3));
    assert_eq!(available_points(&base, 6), 40);
    // All extensions cost 26.
    assert_eq!(spent_points(&all_extensions(base.clone())), 26);
    assert_eq!(warnings(all_extensions(base.clone())), [] as [String; 0]);

    // Without the workshop maps only 24 are left.
    let plain = with_tdm(Settings::default()).with_lobby(lobby_with_slots(3));
    assert_eq!(available_points(&plain, 6), 24);
    let messages = warnings(all_extensions(plain));
    assert_eq!(messages.len(), 1);
    assert!(
        messages[0].contains("26") && messages[0].contains("24"),
        "{messages:?}"
    );
}

#[test]
fn extension_points_without_a_lobby_are_zero_unless_workshop_maps() {
    let no_lobby = with_tdm(Settings::default()).with_extension(Extension::BeamSounds);
    assert_eq!(available_points(&no_lobby, 0), 0);
    assert_eq!(warnings(no_lobby).len(), 1);
    let workshop = Settings::default()
        .with_mode(tdm(), ModeSettings::default().maps(workshop_maps()))
        .with_extension(Extension::BeamSounds);
    assert_eq!(available_points(&workshop, 0), 16);
    assert_eq!(warnings(workshop), [] as [String; 0]);
}

#[test]
fn full_lobby_leaves_no_points() {
    let settings = with_tdm(Settings::default())
        .with_lobby(lobby_with_slots(6))
        .with_extension(Extension::Projectiles);
    assert_eq!(available_points(&settings, 12), 0);
    // 6 + 6 = 12 seats.
    assert_eq!(warnings(settings).len(), 1);
}

#[test]
fn workshop_maps_bonus_rules() {
    let only = |maps: Option<MapSelection>| {
        let mode = maps.map_or_else(ModeSettings::default, |m| ModeSettings::default().maps(m));
        Settings::default().with_mode(tdm(), mode)
    };
    assert!(only_workshop_maps(&only(Some(workshop_maps()))));
    // A map that isn't a workshop map.
    let mixed = MapSelection::Enabled(vec![
        MapRef::new(Map::WorkshopChamber),
        MapRef::new(Map::Dorado),
    ]);
    assert!(!only_workshop_maps(&only(Some(mixed))));
    // A disabled list means the rest of the maps are played.
    let disabled = MapSelection::Disabled(vec![MapRef::new(Map::WorkshopChamber)]);
    assert!(!only_workshop_maps(&only(Some(disabled))));
    // No lists at all, or an empty one, is not "only workshop maps".
    assert!(!only_workshop_maps(&only(None)));
    assert!(!only_workshop_maps(&only(Some(MapSelection::Enabled(
        vec![]
    )))));
    // A mode without a list changes nothing, and a disabled mode is not played.
    let extra = only(Some(workshop_maps()))
        .with_mode(ModeKey::Mode(Gamemode::Skirmish), ModeSettings::default());
    assert!(only_workshop_maps(&extra));
    let off = only(Some(workshop_maps())).with_mode(
        ModeKey::Mode(Gamemode::Assault),
        ModeSettings::default()
            .disabled()
            .maps(MapSelection::Disabled(vec![])),
    );
    assert!(only_workshop_maps(&off));
}

#[test]
fn duplicate_extensions_are_paid_once() {
    let mut settings = with_tdm(Settings::default());
    settings.extensions = vec![Extension::Projectiles, Extension::Projectiles];
    assert_eq!(spent_points(&settings), 4);
}

#[test]
fn extension_points_warning_can_be_suppressed() {
    let mut settings = Settings::default()
        .with_main(main())
        .with_mode(tdm(), ModeSettings::default().maps(workshop_maps()))
        .with_extension(Extension::Projectiles);
    let mut options = CompileOptions::default();
    options.suppress.insert(Warning::ExtensionPoints);
    assert_eq!(check_with(&mut settings, &options), []);
}

#[cfg(feature = "serde")]
#[test]
fn both_map_lists_are_rejected_when_reading() {
    let result: Result<Settings, _> =
        serde_json::from_str(r#"{"gamemodes": {"tdm": {"enabledMaps": [], "disabledMaps": []}}}"#);
    assert!(result.is_err());
}
