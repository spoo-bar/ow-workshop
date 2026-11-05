use ow_workshop_data::{LocalizedString, ValueId as V};

use crate::ir::{Expr, ExprKind, StrExpr};
use crate::lower::testutil::{konst, num};
use crate::opt::testutil::*;

fn s(id: V, args: Vec<Expr>) -> String {
    speed(&val(id, args))
}

fn localized(text: &str) -> Expr {
    let phrase = LocalizedString::from_en(text).expect("a game phrase");
    Expr::new(ExprKind::Str(StrExpr::Localized(phrase)), None)
}

#[test]
fn string_contains_folds() {
    assert_eq!(s(V::StrContains, vec![cstr("hello"), cstr("ell")]), "true");
    assert_eq!(s(V::StrContains, vec![cstr("hello"), cstr("xyz")]), "false");
    assert_eq!(
        s(V::StrContains, vec![cstr("hello"), g(0)]),
        "StrContains(\"hello\"(), g0)"
    );
}

#[test]
fn string_length_counts_characters() {
    assert_eq!(s(V::StrLen, vec![cstr("héllo")]), "5");
    assert_eq!(s(V::StrLen, vec![cstr("")]), "0");
}

#[test]
fn char_in_string() {
    assert_eq!(s(V::CharAt, vec![cstr("abc"), num(1.0)]), "\"b\"()");
    assert_eq!(
        s(V::CharAt, vec![cstr("abc"), num(-4.0)]),
        "\"a\"()",
        "negative counts as 0"
    );
    assert_eq!(
        s(V::CharAt, vec![cstr("abc"), num(9.0)]),
        "\"\"()",
        "out of range is empty"
    );
}

#[test]
fn string_slice() {
    assert_eq!(
        s(V::Substring, vec![cstr("abcdef"), num(1.0), num(3.0)]),
        "\"bcd\"()"
    );
    assert_eq!(
        s(V::Substring, vec![cstr("abcdef"), num(-2.0), num(2.0)]),
        "\"ab\"()"
    );
    assert_eq!(
        s(V::Substring, vec![cstr("abc"), num(1.0), num(-2.0)]),
        "\"\"()"
    );
}

#[test]
fn string_replace_replaces_all() {
    assert_eq!(
        s(V::Replace, vec![cstr("aXbXc"), cstr("X"), cstr("--")]),
        "\"a--b--c\"()"
    );
    assert_eq!(
        s(V::Replace, vec![cstr("abc"), cstr(""), cstr("-")]),
        "Replace(\"abc\"(), \"\"(), \"-\"())"
    );
}

#[test]
fn string_split_is_never_folded() {
    assert_eq!(
        s(V::Split, vec![cstr("a,b"), cstr(",")]),
        "Split(\"a,b\"(), \",\"())"
    );
}

#[test]
fn strings_with_placeholders_are_not_folded() {
    let formatted = Expr::new(
        ExprKind::Str(StrExpr::Custom {
            tokens: vec![crate::ir::StrToken::Arg(0)],
            args: vec![g(0)],
        }),
        None,
    );
    assert_eq!(s(V::StrLen, vec![formatted]), "StrLen(\"{0}\"(g0))");
}

#[test]
fn strings_are_not_folded_in_strict_mode() {
    let e = val(V::StrLen, vec![cstr("abc")]);
    assert_eq!(strict(&e), "StrLen(\"abc\"())");
}

#[test]
fn language_neutral_phrases_become_custom_strings() {
    assert_eq!(speed(&localized("*")), "\"*\"()");
    assert_eq!(speed(&localized("----------")), "\"----------\"()");
    assert_eq!(speed(&localized("")), "\"\"()");
}

#[test]
fn translated_phrases_stay_localized() {
    let e = localized("!");
    assert_eq!(speed(&e), sh(&e));
}

#[test]
fn language_neutral_phrases_with_arguments() {
    let call = |args| val(V::LocalizedString, args);
    let e = call(vec![localized("{0} -> {1}"), g(0), g(1), g(2)]);
    assert_eq!(speed(&e), "\"{0} -> {1}\"(g0, g1)");
    let short = call(vec![localized("({0})"), g(0)]);
    assert_eq!(speed(&short), "\"({0})\"(g0)");
    let missing_arg = call(vec![localized("{0} -> {1}"), g(0)]);
    assert_eq!(speed(&missing_arg), sh(&missing_arg));
}

#[test]
fn opposite_team() {
    let f = |key| s(V::GetOppositeTeam, vec![konst("Team", key)]);
    assert_eq!(f("1"), "Team.2");
    assert_eq!(f("2"), "Team.1");
    assert_eq!(f("ALL"), "Team.ALL");
    assert_eq!(s(V::GetOppositeTeam, vec![g(0)]), "GetOppositeTeam(g0)");
}

#[test]
fn custom_colors_matching_a_built_in_color() {
    let rgb = |r, g_, b, a| val(V::Rgb, vec![num(r), num(g_), num(b), num(a)]);
    assert_eq!(speed(&rgb(200.0, 0.0, 19.0, 255.0)), "Color.RED");
    assert_eq!(speed(&rgb(1.0, 2.0, 3.0, 255.0)), "Rgb(1, 2, 3, 255)");
    // Non-strict only.
    assert_eq!(
        strict(&rgb(200.0, 0.0, 19.0, 255.0)),
        "Rgb(200, 0, 19, 255)"
    );
}

#[test]
fn direction_constants_become_vectors_while_optimizing() {
    assert_eq!(s(V::VectorLeft, vec![]), "V(1, 0, 0)");
    assert_eq!(s(V::VectorRight, vec![]), "V(-1, 0, 0)");
    assert_eq!(s(V::VectorUp, vec![]), "V(0, 1, 0)");
    assert_eq!(s(V::VectorDown, vec![]), "V(0, -1, 0)");
    assert_eq!(s(V::VectorForward, vec![]), "V(0, 0, 1)");
    assert_eq!(s(V::VectorBackward, vec![]), "V(0, 0, -1)");
    assert_eq!(off(&val(V::VectorLeft, vec![])), "VectorLeft()");
    // So that they fold: `Left * 2`.
    assert_eq!(
        speed(&mul(val(V::VectorLeft, vec![]), num(2.0))),
        "V(2, 0, 0)"
    );
}
