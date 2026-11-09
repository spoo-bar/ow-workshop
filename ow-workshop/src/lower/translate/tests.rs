use super::*;

#[test]
fn markers_for_placeholders() {
    assert_eq!(marker_text(0), "(0.00, 1.00, 0.00)");
    assert_eq!(marker_text(5), "(0.00, 0.00, -1.00)");
    assert_eq!(marker_text(6), "1876650.25");
    assert_eq!(marker_text(15), "1876659.25");
}

#[test]
fn placeholders_are_marked() {
    assert_eq!(
        mark_placeholders("{0} of {1} {x} {"),
        "(0.00, 1.00, 0.00) of (0.00, -1.00, 0.00) {x} {"
    );
}

#[test]
fn placeholder_indices() {
    assert_eq!(placeholders("{1} {0} {x} {} {12}"), [1, 0, 12]);
    assert_eq!(placeholder_out_of_range("{0} {2}", 2), Some(2));
    assert_eq!(placeholder_out_of_range("{0} {1}", 2), None);
}

#[test]
fn raw_text_becomes_tokens() {
    assert_eq!(
        tokens("a{0}b{1}{x}"),
        [
            StrToken::Text("a".to_owned()),
            StrToken::Arg(0),
            StrToken::Text("b".to_owned()),
            StrToken::Arg(1),
            StrToken::Text("{x}".to_owned()),
        ]
    );
}
