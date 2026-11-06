use super::*;

/// The width of space characters, from the space table.
fn spaces_width(s: &str) -> u32 {
    s.chars()
        .map(|c| {
            SPACES
                .iter()
                .find(|(_, sc)| *sc == c)
                .map_or(0, |(w, _)| *w)
        })
        .sum()
}

#[test]
fn visual_len_sums_character_widths() {
    assert_eq!(visual_len(""), Ok(0));
    assert_eq!(visual_len("A"), Ok(342));
    assert_eq!(visual_len("AB"), Ok(684));
    assert_eq!(visual_len("a\nb"), Err(TextWidthError::Newline));
}

#[test]
fn unlisted_characters_are_512_wide() {
    assert_eq!(visual_len("\u{3042}"), Ok(512));
}

#[test]
fn spaces_add_up_exactly_with_the_fewest_characters() {
    assert_eq!(spaces_for(0), "");
    assert_eq!(spaces_for(142), " ");
    // 143 = 142 + 1
    let s = spaces_for(143);
    assert_eq!(s.chars().count(), 2);
    assert_eq!(spaces_width(&s), 143);
    for width in [1, 2, 3, 70, 71, 300, 513, 1024, 2047] {
        let s = spaces_for(width);
        assert_eq!(spaces_width(&s), width, "width {width}");
    }
    // 1024 = 512 + 512
    assert_eq!(spaces_for(1024).chars().count(), 2);
}

#[test]
fn spaces_for_str_matches_the_text() {
    let s = spaces_for_str("Hello").expect("no newline");
    assert_eq!(Ok(spaces_width(&s)), visual_len("Hello"));
    assert_eq!(spaces_for_str("a\nb"), Err(TextWidthError::Newline));
}
