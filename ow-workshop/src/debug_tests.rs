use ow_workshop_data::ActionId;

use super::*;
use crate::ir::{ExprKind, StmtKind, StrExpr};
use crate::opt::testutil::sh;
use crate::prelude::*;

#[derive(GlobalVars)]
#[allow(dead_code)]
struct Globals {
    a: Global<Num>,
    list: Global<Array<Num>>,
}

fn globals() -> Globals {
    Globals::declare(&mut VarRegistry::default())
}

/// The subheader argument of the only HUD text recorded, with the text modifier unwrapped.
fn shown(tags: bool, record: impl FnOnce(&mut Block)) -> ir::Expr {
    let mut block = Block::new();
    block.tags = tags;
    record(&mut block);
    let stmts = block.into_ir().stmts;
    assert_eq!(stmts.len(), 1);
    let StmtKind::Action { id, args } = &stmts[0].kind else {
        panic!("not an action");
    };
    assert_eq!(*id, ActionId::HudText);
    assert_eq!(sh(&args[0]), "GetPlayers(Team.ALL)");
    assert_eq!(sh(&args[5]), "-9999");
    assert_eq!(sh(&args[3]), "Null");
    args[2].clone()
}

/// The text template and its arguments, below the `case_sensitive` modifier.
fn template(expr: &ir::Expr) -> (String, Vec<ir::Expr>) {
    let ExprKind::StrModify { modifier, inner } = expr.kind() else {
        panic!("not modified: {expr:?}");
    };
    assert_eq!(*modifier, ir::StrModifier::CaseSensitive);
    let ExprKind::Str(StrExpr::Custom { tokens, args }) = inner.kind() else {
        panic!("not a template: {inner:?}");
    };
    let text = tokens
        .iter()
        .map(|t| match t {
            StrToken::Text(t) => t.clone(),
            StrToken::Arg(n) => format!("{{{n}}}"),
        })
        .collect();
    (text, args.clone())
}

#[test]
fn shows_the_source_and_the_value() {
    let g = globals();
    let out = shown(false, |b| debug!(b, g.a.get() + 2));
    let (text, args) = template(&out);
    assert_eq!(
        text,
        format!("g.a.get() + 2\u{2028}= {{0}}{}", " ".repeat(170))
    );
    assert_eq!(args.iter().map(sh).collect::<Vec<_>>(), ["(g0 + 2)"]);
}

#[test]
fn colors_the_parts_when_tags_are_on() {
    let g = globals();
    let out = shown(true, |b| debug!(b, g.a));
    let (text, _) = template(&out);
    assert_eq!(
        text,
        format!(
            "<FGD0FF00FF>g.a\u{2028}<FG99F3FFFF>= <FGFFFF00FF>{{0}}{}",
            " ".repeat(170)
        )
    );
}

#[test]
// The braces are the point: they must not turn into placeholders.
#[allow(unused_braces)]
fn braces_in_the_source_do_not_become_placeholders() {
    let out = shown(false, |b| debug!(b, vect(1, 2, 3).x().eq({ 1 })));
    let (text, args) = template(&out);
    assert!(
        text.contains('\u{E007B}') && text.contains('\u{E007D}'),
        "{text}"
    );
    assert!(!text.contains("{ 1 }"), "{text}");
    assert_eq!(args.len(), 1);
}

#[test]
fn arrays_are_spelled_out() {
    let g = globals();
    let out = shown(false, |b| debug!(b, g.list));
    let (_, args) = template(&out);
    // `array_to_string(list, 6)`: a mapped array over a mapped array.
    assert!(
        sh(&args[0]).starts_with("MappedArray(MappedArray("),
        "{}",
        sh(&args[0])
    );

    let plain = shown(false, |b| debug!(b, g.a));
    assert_eq!(sh(&template(&plain).1[0]), "g0");
}

#[test]
fn a_value_already_spelled_out_is_not_wrapped_twice() {
    let g = globals();
    let spelled = array_to_string(g.list.get(), 3);
    let out = shown(false, |b| debug!(b, spelled));
    let (_, args) = template(&out);
    assert_eq!(
        sh(&args[0])
            .matches("MappedArray(MappedArray([MappedArray")
            .count(),
        1
    );
}

#[test]
fn long_sources_are_cut_to_the_hud_width() {
    let long = "x".repeat(20_000);
    let plain = source_text(&long, false);
    assert!(plain.ends_with('…'));
    let width = |t: &str| t.chars().map(char_width).sum::<u32>();
    assert!(width(&plain) <= MAX_SOURCE_WIDTH, "{}", width(&plain));
    assert!(width(&plain) > MAX_SOURCE_WIDTH - 2 * char_width('x'));

    let tagged = source_text(&long, true);
    assert!(tagged.ends_with("</FG>…"));
    assert!(width(&tagged) <= MAX_SOURCE_WIDTH);

    assert_eq!(source_text("short", false), "short");
}

#[test]
fn newlines_in_the_source_become_spaces() {
    assert_eq!(source_text("a\nb\r\nc", false), "a b c");
}
