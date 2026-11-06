use ow_workshop_data::{SPACES, char_width, lowercase_metrics};

use super::*;

/// The width of one character: space characters come from the space table.
fn width_of(c: char) -> u32 {
    SPACES
        .iter()
        .find(|(_, sc)| *sc == c)
        .map_or_else(|| char_width(c), |(w, _)| *w)
}

fn pen(text: &str) -> u32 {
    text.chars().map(width_of).sum()
}

fn render(pieces: &[Piece]) -> String {
    pieces
        .iter()
        .map(|p| match p {
            Piece::Text(t) => t.clone(),
            Piece::Arg(i) => format!("{{{i}}}"),
        })
        .collect()
}

fn cased(text: &str, n: usize) -> Vec<String> {
    cased_template(&[StrToken::Text(text.to_owned())], n)
        .iter()
        .map(|p| render(p))
        .collect()
}

#[test]
fn text_without_lowercase_letters_is_unchanged() {
    assert_eq!(cased("123 ABC", 2), ["123 ABC\u{AD}", "123 ABC\u{AD}"]);
}

#[test]
fn two_texts_split_neighbouring_letters() {
    let [first, second] = <[String; 2]>::try_from(cased("ab", 2)).expect("two outputs");
    let (a, b) = (
        lowercase_metrics('a').expect("a").lower,
        lowercase_metrics('b').expect("b").lower,
    );
    // Neighbouring letters never share an output.
    assert_ne!(first.contains(a), first.contains(b));
    assert_ne!(second.contains(a), second.contains(b));
    assert!(first.contains(a) || second.contains(a));
    assert!(first.contains(b) || second.contains(b));
}

#[test]
fn the_outputs_are_equally_wide() {
    let outputs = cased("hello world", 3);
    let widths: Vec<u32> = outputs
        .iter()
        .map(|o| pen(o.trim_end_matches('\u{AD}')))
        .collect();
    assert!(widths.iter().all(|w| *w == widths[0]), "{widths:?}");
}

#[test]
fn letters_land_where_the_uppercase_letter_would() {
    // The text is shifted so the first glyph's bearing fits; what is left is the uppercase bearing.
    let [out, _] = <[String; 2]>::try_from(cased("a", 2)).expect("two outputs");
    let glyph = lowercase_metrics('a').expect("a");
    let pad = out.split(glyph.lower).next().expect("before the glyph");
    assert_eq!(pen(pad), u32::try_from(glyph.xmin).expect("positive"));
}

#[test]
fn each_line_ends_with_a_soft_hyphen_and_starts_after_two_spaces() {
    let outputs = cased("A\nB", 2);
    for out in outputs {
        let lines: Vec<&str> = out.split('\n').collect();
        assert_eq!(lines.len(), 2);
        assert!(lines.iter().all(|l| l.ends_with('\u{AD}')));
        assert!(lines[1].starts_with("  "));
    }
}

#[test]
fn fg_tags_go_to_every_output() {
    for out in cased("<fgFF0000FF>x", 2) {
        assert!(out.contains("<fgFF0000FF>"), "{out:?}");
    }
}

#[test]
fn placeholders_appear_in_one_output() {
    let outputs = cased_template(&[StrToken::Text("a".to_owned()), StrToken::Arg(0)], 2);
    let with_arg = outputs
        .iter()
        .filter(|o| o.iter().any(|p| matches!(p, Piece::Arg(_))))
        .count();
    assert_eq!(with_arg, 1);
}

#[test]
fn widths_use_the_font_table() {
    assert_eq!(char_width('A'), 342);
}

#[test]
fn n_must_be_between_2_and_6() {
    let text = Expr::<Str>::from("a");
    assert_eq!(
        cased_texts(1, text.clone()).map(|t| t.len()),
        Err(CasedTextError::Count(1))
    );
    assert_eq!(
        cased_texts(7, text.clone()).map(|t| t.len()),
        Err(CasedTextError::Count(7))
    );
    assert_eq!(cased_texts(6, text).map(|t| t.len()), Ok(6));
}
