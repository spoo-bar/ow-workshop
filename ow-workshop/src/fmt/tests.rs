use crate::check::has_placeholder_lookalike;
use crate::ir::{ExprKind, StrExpr, StrToken};
use crate::prelude::*;

fn template_of(e: &Expr<Str>) -> (Vec<StrToken>, usize) {
    match e.ir().kind() {
        ExprKind::Str(StrExpr::Custom { tokens, args }) => (tokens.clone(), args.len()),
        other => panic!("not a template: {other:?}"),
    }
}

fn text(s: &str) -> StrToken {
    StrToken::Text(s.to_owned())
}

#[test]
fn positional_placeholders() {
    let (a, b) = (Expr::<Num>::from(1), Expr::<Num>::from(2));
    let (tokens, args) = template_of(&ws_fmt!("Score: {} / {}", a, b));
    assert_eq!(
        tokens,
        [
            text("Score: "),
            StrToken::Arg(0),
            text(" / "),
            StrToken::Arg(1)
        ]
    );
    assert_eq!(args, 2);
}

#[test]
fn numbered_placeholders_may_repeat() {
    let (tokens, args) = template_of(&ws_fmt!("{0}{1}{0}", 1, "x"));
    assert_eq!(
        tokens,
        [StrToken::Arg(0), StrToken::Arg(1), StrToken::Arg(0)]
    );
    assert_eq!(args, 2);
}

#[test]
fn identifiers_are_captured_and_named_args_work() {
    let score = Expr::<Num>::from(7);
    let (tokens, args) = template_of(&ws_fmt!("Score: {score}"));
    assert_eq!(tokens, [text("Score: "), StrToken::Arg(0)]);
    assert_eq!(args, 1);
    let (tokens, args) = template_of(&ws_fmt!("{a}-{a}-{b}", b = 2, a = 1));
    assert_eq!(
        tokens,
        [
            StrToken::Arg(1),
            text("-"),
            StrToken::Arg(1),
            text("-"),
            StrToken::Arg(0)
        ]
    );
    assert_eq!(args, 2);
}

#[test]
fn doubled_braces_are_literal_and_text_merges() {
    let (tokens, args) = template_of(&ws_fmt!("a{{b}}c"));
    assert_eq!(tokens, [text("a{b}c")]);
    assert_eq!(args, 0);
}

#[test]
fn plain_strings_are_one_text_token_without_carriage_returns() {
    let s: Expr<Str> = "a\r\nb".into();
    assert_eq!(template_of(&s).0, [text("a\nb")]);
    let (tokens, _) = template_of(&ws_fmt!("x\r\ny"));
    assert_eq!(tokens, [text("x\ny")]);
}

#[test]
fn template_has_a_source_location() {
    assert!(ws_fmt!("hi").ir().loc().is_some());
}

#[test]
fn placeholder_lookalikes() {
    assert!(has_placeholder_lookalike("a {0} b"));
    assert!(has_placeholder_lookalike("{12}"));
    assert!(!has_placeholder_lookalike("{}"));
    assert!(!has_placeholder_lookalike("{a}"));
    assert!(!has_placeholder_lookalike("{0"));
}
