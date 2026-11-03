//! Tests for language codes and localized names.

use ow_workshop_data::{ActionId, Hero, Keyword, Lang, Map, Team, ValueId};

#[test]
fn lang_codes_round_trip() {
    assert_eq!(Lang::default(), Lang::EnUs);
    for lang in Lang::ALL {
        assert_eq!(lang.code().parse::<Lang>(), Ok(lang));
    }
    assert_eq!("fr-FR".parse::<Lang>(), Ok(Lang::FrFr));
    assert!("xx-XX".parse::<Lang>().is_err());
}

#[test]
fn lang_order_matches_discriminants() {
    for (index, lang) in Lang::ALL.into_iter().enumerate() {
        assert_eq!(lang as usize, index);
    }
}

#[test]
fn keywords_are_localized() {
    assert_eq!(Keyword::Rule.name(Lang::EnUs), "rule");
    assert_eq!(Keyword::Rule.name(Lang::FrFr), "règle");
    assert_eq!(
        Keyword::EventGlobal.name(Lang::FrFr),
        "Toute la partie - Tout le monde"
    );
}

#[test]
fn action_names_in_other_scripts() {
    assert_eq!(ActionId::AbortIf.name(Lang::JaJp), "中止する条件");
    assert_eq!(ActionId::AbortIf.name(Lang::ZhCn), "根据条件中止");
    assert_eq!(ActionId::Teleport.name(Lang::JaJp), "テレポート");
    assert_eq!(ActionId::Teleport.name(Lang::ZhCn), "传送");
}

#[test]
fn missing_translations_fall_back_to_english() {
    assert_eq!(Map::Aatlis.name(Lang::ZhCn), "阿特利斯");
    assert_eq!(Map::Aatlis.name(Lang::FrFr), "Aatlis");
    // The source marks never-translated names with `<en>`; they must not leak.
    assert_eq!(Hero::Ana.name(Lang::TrTr), "Ana");
}

#[test]
fn every_name_resolves_in_every_language() {
    for lang in Lang::ALL {
        assert!(
            ValueId::ALL
                .iter()
                .all(|v| !v.name(lang).is_empty() || v.key().is_empty())
        );
        assert!(Team::ALL.iter().all(|t| !t.name(lang).is_empty()));
    }
}
