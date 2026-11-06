use super::*;
use crate::ir::{StmtKind, VarId};
use crate::lower::testutil::*;
use crate::prelude::{Expr as TExpr, Str, ws_fmt};

#[test]
fn fullwidth_maps_ascii_and_signs() {
    assert_eq!(fullwidth("Ab!~ "), ("Ａｂ！～\u{2001}".to_owned(), true));
    assert_eq!(fullwidth("¥₩¢£¯¬¦"), ("￥￦￠￡￣￢￤".to_owned(), true));
    assert_eq!(fullwidth("é"), ("é".to_owned(), false));
    assert!(!fullwidth("a\n").1);
}

#[test]
fn big_replaces_only_the_first_match() {
    assert_eq!(big_letter("xyz"), Some("Χyz".to_owned()));
    assert_eq!(big_letter("Cat"), Some("CΑt".to_owned()));
    assert_eq!(big_letter("c d"), Some("c\u{2028}d".to_owned()));
    assert_eq!(big_letter("a.b"), Some("Α.b".to_owned()));
    assert_eq!(big_letter("cfg"), None);
}

#[test]
fn case_sensitive_replaces_in_order() {
    assert_eq!(case_sensitive("hi"), "\u{4BB}\u{456}");
    assert_eq!(case_sensitive("l"), "I");
    assert_eq!(case_sensitive("æ"), "\u{4D5}");
    assert_eq!(case_sensitive("nj"), "\u{1CC}");
    // `e` and `n` before a digit or symbol take a grave accent.
    assert_eq!(case_sensitive("e1"), "\u{450}1");
    assert_eq!(case_sensitive("n!"), "\u{1F9}!");
    assert_eq!(case_sensitive("en"), "\u{1EB9}\u{1E47}");
    assert_eq!(case_sensitive("x a y"), "\u{4B3} \u{FF41} \u{1EF5}");
}

fn modified(e: TExpr<Str>) -> Result<(String, Vec<&'static str>), Vec<&'static str>> {
    let mut p = program(vec![StmtKind::Action {
        id: ow_workshop_data::ActionId::Wait,
        args: vec![e.into_ir()],
    }]);
    match run(apply, &mut p, &options(crate::options::OptLevel::Off)) {
        Ok(d) => {
            let StmtKind::Action { args, .. } = &p.rules[0].body.stmts[0].kind else {
                panic!("not an action");
            };
            Ok((show(&args[0]), codes(&d)))
        }
        Err(d) => Err(codes(&d)),
    }
}

#[test]
fn modifiers_leave_placeholders_alone() {
    let a = TExpr::<crate::prelude::Num>::from(1);
    let (shown, codes) = modified(ws_fmt!("ab{}cd", a).fullwidth()).expect("ok");
    assert_eq!(shown, "\"ａｂ{0}ｃｄ\"(1)");
    assert_eq!(codes, Vec::<&str>::new());
}

#[test]
fn chained_modifiers_apply_in_order() {
    let (shown, _) = modified(ws_fmt!("hi").case_sensitive().fullwidth()).expect("ok");
    assert!(shown.contains('\u{4BB}'));
}

#[test]
fn partial_fullwidth_warns() {
    let (shown, codes) = modified(ws_fmt!("aé").fullwidth()).expect("ok");
    assert_eq!(shown, "\"ａé\"()");
    assert_eq!(codes, ["not-total-fullwidth"]);
}

#[test]
fn big_without_a_candidate_is_an_error() {
    assert_eq!(
        modified(ws_fmt!("cfg").big()),
        Err(vec!["big-letters-no-match"])
    );
}

#[test]
fn modifiers_on_non_templates_are_errors() {
    let not_a_template = TExpr::<Str>::from_ir(expr(crate::ir::ExprKind::GlobalVar(VarId(1))));
    assert_eq!(
        modified(not_a_template.fullwidth()),
        Err(vec!["string-modifier-target"])
    );
}
