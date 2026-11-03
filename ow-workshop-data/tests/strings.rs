//! Tests for localized phrases.

use ow_workshop_data::{Lang, LocalizedString};

#[test]
fn phrases_have_translations() {
    assert_eq!(LocalizedString::Hello.text(Lang::EnUs), "Hello");
    assert_eq!(LocalizedString::Hello.text(Lang::FrFr), "Bonjour");
    assert_eq!(LocalizedString::Hello.text(Lang::KoKr), "Hello");
}

#[test]
fn lookup_by_english_text() {
    assert_eq!(
        LocalizedString::from_en("Hello"),
        Some(LocalizedString::Hello)
    );
    assert_eq!(
        LocalizedString::from_en("{0}: {1}"),
        Some(LocalizedString::SymAColonSpB)
    );
    assert_eq!(LocalizedString::from_en(""), Some(LocalizedString::Empty));
    assert_eq!(LocalizedString::from_en("Not a phrase"), None);
}

#[test]
fn every_phrase_round_trips() {
    assert_eq!(LocalizedString::ALL.len(), 721);
    for phrase in LocalizedString::ALL {
        assert_eq!(
            LocalizedString::from_en(phrase.text(Lang::EnUs)),
            Some(*phrase)
        );
    }
}

#[test]
fn symbol_and_word_phrases_stay_apart() {
    assert_ne!(
        LocalizedString::from_en("*"),
        LocalizedString::from_en("Star")
    );
}
