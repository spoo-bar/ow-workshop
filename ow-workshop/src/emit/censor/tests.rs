use super::avoid_censor;

#[test]
fn admin_gets_a_soft_hyphen() {
    assert_eq!(avoid_censor("Admin panel"), "A\u{AD}dmin panel");
}

#[test]
fn word_boundaries_are_respected() {
    assert_eq!(avoid_censor("Bass"), "Bass");
    assert_eq!(avoid_censor("ass"), "a\u{AD}ss");
    assert_eq!(avoid_censor("a big ASS."), "a big A\u{AD}SS.");
    assert_eq!(avoid_censor("assets"), "assets");
    assert_eq!(avoid_censor("Norway Norwayx"), "N\u{AD}orway Norwayx");
}

#[test]
fn substring_words_match_anywhere() {
    assert_eq!(avoid_censor("my_account"), "my_a\u{AD}ccount");
    assert_eq!(avoid_censor("1488"), "1\u{AD}488");
    assert_eq!(avoid_censor("Shitty"), "S\u{AD}hitty");
    assert_eq!(avoid_censor("Battlenet"), "B\u{AD}attlenet");
}

#[test]
fn bnet_splits_after_two_letters() {
    assert_eq!(avoid_censor("bnet"), "bn\u{AD}et");
    assert_eq!(avoid_censor("bnets"), "bnets");
}

#[test]
fn rigger_allows_whitespace_between_letters() {
    assert_eq!(avoid_censor("rigger"), "ri\u{AD}gger");
    assert_eq!(avoid_censor("R i gg e R"), "R i \u{AD}gg e R");
    assert_eq!(avoid_censor("riggers"), "riggers");
}

#[test]
fn zero_width_characters_are_removed_first() {
    assert_eq!(avoid_censor("a\u{200B}d\u{FEFF}min"), "a\u{AD}dmin");
    assert_eq!(avoid_censor("x\u{200E}\u{200F}\u{061C}y"), "xy");
}

#[test]
fn every_match_is_rewritten() {
    assert_eq!(avoid_censor("admin admin"), "a\u{AD}dmin a\u{AD}dmin");
}

#[test]
fn clean_text_is_unchanged() {
    assert_eq!(avoid_censor("Hello, world {0}"), "Hello, world {0}");
}
