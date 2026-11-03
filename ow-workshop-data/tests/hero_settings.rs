//! Tests for hero settings.

use ow_workshop_data::settings::{HERO_GENERAL, Kind, hero_settings};
use ow_workshop_data::{Hero, Lang};

#[test]
fn ashe_has_an_ammo_clip_setting() {
    let section = hero_settings(Hero::Ashe).expect("ashe section");
    assert_eq!(section.name(Lang::EnUs), "Ashe");
    let setting = section.get("ammoClipSize%").expect("ammoClipSize%");
    assert_eq!(setting.kind, Kind::Percent);
    assert_eq!(setting.name(Lang::EnUs), "Ammunition Clip Size Scalar");
}

#[test]
fn every_hero_has_a_section() {
    for hero in Hero::ALL {
        assert_eq!(hero_settings(*hero).expect("section").key, hero.key());
    }
}

#[test]
fn include_lists_restrict_general_settings() {
    let ult = HERO_GENERAL.get("ultDuration%").expect("ultDuration%");
    assert!(ult.applies_to(Hero::Ashe));
    assert!(!ult.applies_to(Hero::Ana));
    let melee = HERO_GENERAL.get("enableMelee").expect("enableMelee");
    assert!(melee.applies_to(Hero::Ana));
}
