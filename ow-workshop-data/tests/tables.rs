//! Tests for entities, textures, font metrics, hero abilities and map symmetry.

use ow_workshop_data::{
    Ability, DEFAULT_WIDTH, Hero, Lang, Map, SPACES, Symmetry, Texture, char_width, entity,
    lowercase_metrics,
};

#[test]
fn entities_map_names_to_characters() {
    assert_eq!(entity("black_square"), Some('■'));
    assert_eq!(entity("no_such_entity"), None);
}

#[test]
fn textures_have_tags() {
    assert_eq!(Texture::Assault.tag(), "<txc00000000020af8>");
    assert_eq!(Texture::Assault.key(), "ASSAULT");
}

#[test]
fn font_widths() {
    assert_eq!(char_width('a'), 285);
    assert_eq!(char_width('\u{10ffff}'), DEFAULT_WIDTH);
    assert_eq!(char_width('\u{00ad}'), 0);
}

#[test]
fn spaces_are_sorted_by_width() {
    assert_eq!(SPACES.len(), 10);
    assert!(SPACES.windows(2).all(|pair| pair[0].0 < pair[1].0));
    assert_eq!(SPACES[0], (1, '\u{2006}'));
}

#[test]
fn lowercase_metrics_exist_for_letters() {
    let a = lowercase_metrics('a').expect("metrics for a");
    assert_eq!((a.lower, a.width, a.lower_width), ('ａ', 285, 512));
    assert_eq!(lowercase_metrics('1'), None);
}

#[test]
fn hero_abilities() {
    let abilities = Hero::Ana.abilities();
    let (_, sleep_dart) = abilities
        .iter()
        .find(|(slot, _)| *slot == Ability::Ability1)
        .unwrap();
    assert_eq!(sleep_dart.get(Lang::EnUs), "Sleep Dart");
    assert_eq!(sleep_dart.get(Lang::FrFr), "Fléchette hypodermique");
}

#[test]
fn map_symmetry() {
    assert_eq!(
        Map::Ayutthaya.symmetry(),
        Some(Symmetry::Axis { a: 0.0, b: -7.0 })
    );
    assert_eq!(Map::Busan.symmetry(), None);
}
