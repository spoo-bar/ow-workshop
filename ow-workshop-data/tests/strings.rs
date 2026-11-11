//! Tests for localized phrases.

use ow_workshop_data::{Lang, LocalizedString};

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
