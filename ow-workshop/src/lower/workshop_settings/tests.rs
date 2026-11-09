use ow_workshop_data::ActionId;

use super::*;
use crate::ir::{Expr, StmtKind};
use crate::lower::testutil::{codes, options, program, run};
use crate::options::OptLevel;
use crate::workshop_setting::{combo, int};

fn setting(e: impl Into<crate::expr::Expr<crate::ty::Num>>) -> Expr {
    e.into().into_ir()
}

fn lowered(settings: Vec<Expr>) -> (Result<Diagnostics, Diagnostics>, Program) {
    let body = settings
        .into_iter()
        .map(|e| StmtKind::Action {
            id: ActionId::Wait,
            args: vec![e],
        })
        .collect();
    let mut p = program(body);
    let result = run(lower, &mut p, &options(OptLevel::Off));
    (result, p)
}

fn args(p: &Program, statement: usize) -> Vec<Expr> {
    let StmtKind::Action { args, .. } = &p.rules[0].body.stmts[statement].kind else {
        panic!("not an action");
    };
    let ExprKind::Value { args, .. } = args[0].kind() else {
        panic!("not a value");
    };
    args.clone()
}

fn text(e: &Expr) -> String {
    let ExprKind::Str(StrExpr::Custom { tokens, args }) = e.kind() else {
        panic!("not a string: {e:?}");
    };
    assert_eq!(args.len(), 0);
    tokens
        .iter()
        .map(|t| match t {
            StrToken::Text(t) => t.as_str(),
            StrToken::Arg(_) => panic!("placeholder"),
        })
        .collect()
}

/// The category and name of the `i`th statement.
fn names(p: &Program, i: usize) -> (String, String) {
    let a = args(p, i);
    (text(&a[0]), text(&a[1]))
}

fn error_codes(settings: Vec<Expr>) -> Vec<&'static str> {
    codes(&lowered(settings).0.expect_err("fails"))
}

#[test]
fn colon_and_braces_are_replaced_everywhere() {
    let (result, p) = lowered(vec![setting(int("a:b{c}", "d:e{f}", 1))]);
    result.expect("lowers");
    assert_eq!(
        names(&p, 0),
        (
            "a\u{E003A}b\u{E007B}c\u{E007D}".to_owned(),
            "d\u{E003A}e\u{E007B}f\u{E007D}".to_owned()
        )
    );
}

#[test]
fn names_also_lose_the_characters_the_game_strips() {
    let (result, p) = lowered(vec![setting(int("\"(),/;\\|", "\"(),/;\\|", 1))]);
    result.expect("lowers");
    let (category, name) = names(&p, 0);
    // The category only loses `:{}`.
    assert_eq!(category, "\"(),/;\\|");
    assert_eq!(
        name,
        "\u{E0022}\u{E0028}\u{E0029}\u{E002C}\u{E002F}\u{E003B}\u{E005C}\u{E007C}"
    );
}

#[test]
fn only_combo_options_follow_the_category_rules() {
    let (result, p) = lowered(vec![setting(combo("c", "n", ["a,b", "x:y"], 0))]);
    result.expect("lowers");
    let ExprKind::Array(options) = args(&p, 0)[3].kind().clone() else {
        panic!("no options");
    };
    let options: Vec<String> = options.iter().map(text).collect();
    assert_eq!(options, ["a,b", "x\u{E003A}y"]);
}

#[test]
fn whitespace_only_strings_get_a_filler() {
    let (result, p) = lowered(vec![setting(int("", " \t", 0))]);
    result.expect("lowers");
    assert_eq!(
        names(&p, 0),
        ("\u{3000}".to_owned(), " \t\u{3000}".to_owned())
    );
}

#[test]
fn tags_stay_plain_text() {
    let (result, p) = lowered(vec![setting(int(
        "<txC00000000008E02>",
        "<fgFF0000FF>x",
        1,
    ))]);
    result.expect("lowers");
    assert_eq!(
        names(&p, 0),
        ("<txC00000000008E02>".to_owned(), "<fgFF0000FF>x".to_owned())
    );
}

#[test]
fn equal_names_get_the_first_unused_suffix() {
    let (result, p) = lowered(
        (0..4)
            .map(|_| setting(int("Gameplay", "hero", 1)))
            .collect(),
    );
    result.expect("lowers");
    let got: Vec<String> = (0..4).map(|i| names(&p, i).1).collect();
    assert_eq!(
        got,
        ["hero", "hero\u{1}", "hero\u{1E}\u{1}", "hero\u{1F}\u{1}"]
    );
}

#[test]
fn suffixes_skip_names_that_are_taken() {
    let (result, p) = lowered(vec![
        setting(int("c", "a\u{1}", 1)),
        setting(int("c", "a", 1)),
        setting(int("c", "a", 1)),
    ]);
    result.expect("lowers");
    let got: Vec<String> = (0..3).map(|i| names(&p, i).1).collect();
    assert_eq!(got, ["a\u{1}", "a", "a\u{1E}\u{1}"]);
}

#[test]
fn the_suffix_order() {
    let first: Vec<String> = suffixes().take(4).collect();
    assert_eq!(first, ["\u{1}", "\u{1E}\u{1}", "\u{1F}\u{1}", "\u{2}"]);
    // 8 + 16 codes, three spellings each.
    assert_eq!(suffixes().count(), 72);
    assert_eq!(suffixes().last().as_deref(), Some("\u{1F}\u{1D}"));
}

#[test]
fn categories_are_not_deduplicated() {
    let (result, p) = lowered(vec![
        setting(int("same", "a", 1)),
        setting(int("same", "b", 1)),
    ]);
    result.expect("lowers");
    assert_eq!(names(&p, 1), ("same".to_owned(), "b".to_owned()));
}

#[test]
fn at_most_128_settings() {
    let many = |n: usize| -> Vec<Expr> {
        (0..n)
            .map(|i| setting(int("c", &format!("n{i}"), 1)))
            .collect()
    };
    lowered(many(128)).0.expect("128 are fine");
    assert_eq!(error_codes(many(129)), ["too-many-workshop-settings"]);
    assert_eq!(error_codes(many(200)), ["too-many-workshop-settings"]);
}

#[test]
fn length_is_checked_before_and_after_rewriting() {
    let long = |n: usize| "a".repeat(n);
    lowered(vec![setting(int(&long(128), &long(128), 1))])
        .0
        .expect("128 characters are fine");
    assert_eq!(
        error_codes(vec![setting(int(&long(129), "n", 1))]),
        ["workshop-setting-too-long"]
    );
    // 128 spaces become 129 characters with the filler.
    assert_eq!(
        error_codes(vec![setting(int(&" ".repeat(128), "n", 1))]),
        ["workshop-setting-too-long"]
    );
    // The suffix of a repeated name counts too.
    assert_eq!(
        error_codes(vec![
            setting(int("c", &long(128), 1)),
            setting(int("c", &long(128), 1)),
        ]),
        ["workshop-setting-too-long"]
    );
    assert_eq!(
        error_codes(vec![setting(combo("c", "n", [long(129)], 0))]),
        ["workshop-setting-too-long"]
    );
}

#[test]
fn strings_must_be_plain_literals() {
    let with_placeholder = crate::ir::Expr::new(
        ExprKind::Str(StrExpr::Custom {
            tokens: vec![StrToken::Text("a ".to_owned()), StrToken::Arg(0)],
            args: vec![crate::lower::testutil::num(1.0)],
        }),
        None,
    );
    let ExprKind::Value { id, args } = setting(int("c", "n", 1)).kind().clone() else {
        panic!("not a value");
    };
    let mut bad = args;
    bad[1] = with_placeholder;
    let e = crate::lower::testutil::call(id, bad);
    assert_eq!(error_codes(vec![e]), ["workshop-setting-not-literal"]);
}

#[test]
fn combo_default_other_than_zero_warns() {
    let (result, _) = lowered(vec![setting(combo("c", "n", ["a", "b"], 1))]);
    assert_eq!(
        codes(&result.expect("lowers")),
        ["workshop-setting-enum-default"]
    );
    let (result, _) = lowered(vec![setting(combo("c", "n", ["a", "b"], 0))]);
    assert_eq!(codes(&result.expect("lowers")), [] as [&str; 0]);
}

#[test]
fn the_combo_warning_can_be_suppressed() {
    let mut p = program(vec![StmtKind::Action {
        id: ActionId::Wait,
        args: vec![setting(combo("c", "n", ["a", "b"], 1))],
    }]);
    let mut options = options(OptLevel::Off);
    options
        .suppress
        .0
        .insert("workshop-setting-enum-default".to_owned());
    let diags = run(lower, &mut p, &options).expect("lowers");
    assert_eq!(codes(&diags), [] as [&str; 0]);
}

#[test]
fn other_values_are_untouched() {
    let other = crate::lower::testutil::call(
        ValueId::Add,
        vec![
            crate::lower::testutil::num(1.0),
            crate::lower::testutil::num(2.0),
        ],
    );
    let (result, p) = lowered(vec![other.clone(), setting(int("c", "n", 0))]);
    result.expect("lowers");
    let StmtKind::Action { args, .. } = &p.rules[0].body.stmts[0].kind else {
        panic!("not an action");
    };
    assert_eq!(args[0], other);
}

#[test]
fn sanitize_leaves_other_characters_alone() {
    assert_eq!(sanitize("Hello, wörld!", Part::Category), "Hello, wörld!");
    assert_eq!(sanitize("a b", Part::Option), "a b");
}
