//! Tests for the custom game settings schema.

use ow_workshop_data::settings::{GAMEMODE_GENERAL, Kind, LOBBY, MAIN, gamemode_settings};
use ow_workshop_data::{Gamemode, Lang};

#[test]
fn respawn_time_is_a_gamemode_percent_setting() {
    let setting = GAMEMODE_GENERAL.get("respawnTime%").expect("respawnTime%");
    assert_eq!(setting.kind, Kind::Percent);
    assert_eq!(setting.name(Lang::EnUs), "Respawn Time Scalar");
    assert_eq!(setting.name(Lang::FrFr), "Temps de réapparition");
}

#[test]
fn main_and_lobby_settings() {
    assert_eq!(MAIN.name(Lang::EnUs), "main");
    let name = MAIN.get("modeName").expect("modeName");
    assert!(matches!(
        name.kind,
        Kind::String {
            max_chars: Some(_),
            ..
        } | Kind::String {
            max_bytes: Some(_),
            ..
        }
    ));
    let Kind::Combo(choices) = LOBBY.get("mapRotation").expect("mapRotation").kind else {
        panic!("mapRotation is a combo");
    };
    assert!(choices.iter().any(|c| c.key == "afterMirrorMatch"));
    assert_eq!(
        LOBBY.get("swapTeamsAfterMatch").map(|s| s.kind),
        Some(Kind::BoolYesNo)
    );
}

#[test]
fn every_mode_has_a_section() {
    for mode in Gamemode::ALL {
        let section = gamemode_settings(*mode).expect("section");
        assert_eq!(section.key, mode.key());
    }
}
