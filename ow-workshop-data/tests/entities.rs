//! Sanity checks for the generated hero, map, gamemode and extension enums.

use ow_workshop_data::{Extension, Gamemode, Hero, Map, Slots};

#[test]
fn heroes_are_complete_and_keyed() {
    assert_eq!(Hero::ALL.len(), 54);
    assert_eq!(Hero::Ana.key(), "ana");
    assert_eq!(Hero::WreckingBall.key(), "wreckingBall");
}

#[test]
fn map_metadata() {
    assert_eq!(Map::ALL.len(), 91);
    assert!(Map::WorkshopIsland.is_workshop_map());
    assert!(!Map::Busan.is_workshop_map());
    assert!(Map::Busan.gamemodes().contains(&Gamemode::Control));
    assert_eq!(Map::Aatlis.variants(), &[("default", "972777519512068153")]);
}

#[test]
fn gamemode_slots() {
    assert_eq!(Gamemode::ALL.len(), 35);
    assert_eq!(
        Gamemode::Assault.default_slots(),
        Slots {
            team1: Some(5),
            team2: Some(5),
            ffa: None
        }
    );
    assert_eq!(Gamemode::Deathmatch.default_slots().ffa, Some(12));
}

#[test]
fn extensions() {
    assert_eq!(Extension::ALL.len(), 11);
    assert_eq!(Extension::BeamEffects.key(), "beamEffects");
}
