use super::*;
use crate::ir::{StmtKind, VarId};
use crate::lower::testutil::*;
use crate::options::OptLevel;

fn text(s: &str) -> StrToken {
    StrToken::Text(s.to_owned())
}

fn tmpl(tokens: Vec<StrToken>, args: Vec<Expr>) -> Expr {
    expr(ExprKind::Str(StrExpr::Custom { tokens, args }))
}

fn normalized(e: Expr, level: OptLevel) -> (String, Vec<&'static str>) {
    let mut p = program(vec![StmtKind::Action {
        id: ow_workshop_data::ActionId::Wait,
        args: vec![e],
    }]);
    let diags = run(normalize, &mut p, &options(level)).expect("runs");
    let StmtKind::Action { args, .. } = &p.rules[0].body.stmts[0].kind else {
        panic!("not an action");
    };
    (show(&args[0]), codes(&diags))
}

#[test]
fn numbered_references_are_unnumbered() {
    let t = tmpl(
        vec![StrToken::Arg(0), StrToken::Arg(1), StrToken::Arg(0)],
        vec![call_a(), call_b()],
    );
    let (shown, _) = normalized(t, OptLevel::Off);
    assert_eq!(shown, "\"{0}{1}{2}\"(g1, g2, g1)");
}

fn call_a() -> Expr {
    expr(ExprKind::GlobalVar(VarId(1)))
}

fn call_b() -> Expr {
    expr(ExprKind::GlobalVar(VarId(2)))
}

#[test]
fn nested_templates_are_spliced() {
    let inner = tmpl(vec![text("b"), StrToken::Arg(0)], vec![call_b()]);
    let t = tmpl(
        vec![text("a"), StrToken::Arg(0), text("c"), StrToken::Arg(1)],
        vec![inner, call_a()],
    );
    let (shown, _) = normalized(t, OptLevel::Speed);
    assert_eq!(shown, "\"ab{0}c{1}\"(g2, g1)");
}

#[test]
fn nested_templates_stay_when_not_optimizing() {
    let inner = tmpl(vec![text("b")], vec![]);
    let t = tmpl(vec![StrToken::Arg(0)], vec![inner]);
    let (shown, _) = normalized(t, OptLevel::Off);
    assert_eq!(shown, "\"{0}\"(\"b\"())");
}

#[test]
fn numbers_and_null_become_text() {
    let t = tmpl(
        vec![
            StrToken::Arg(0),
            text(" "),
            StrToken::Arg(1),
            text(" "),
            StrToken::Arg(2),
            text(" "),
            StrToken::Arg(3),
            text(" "),
            StrToken::Arg(4),
        ],
        vec![
            num(3.5),
            num(3.0),
            num(-2.0),
            expr(ExprKind::Null),
            num(2e7),
        ],
    );
    let (shown, _) = normalized(t, OptLevel::Speed);
    assert_eq!(shown, "\"3.50 3 -2 0 {0}\"(20000000)");
}

#[test]
fn to_fixed_rounds_half_away_from_zero() {
    assert_eq!(to_fixed_2(0.125), "0.13");
    assert_eq!(to_fixed_2(-0.125), "-0.13");
    assert_eq!(to_fixed_2(2.675), "2.67");
    assert_eq!(to_fixed_2(1.005), "1");
    assert_eq!(to_fixed_2(0.0), "0");
    assert_eq!(to_fixed_2(10.1), "10.10");
    assert_eq!(to_fixed_2(-0.001), "-0");
}

#[test]
fn arrays_warn_and_use_the_first_element() {
    let t = tmpl(
        vec![StrToken::Arg(0), StrToken::Arg(1)],
        vec![
            expr(ExprKind::Array(vec![num(7.0), num(8.0)])),
            expr(ExprKind::Array(vec![])),
        ],
    );
    let (shown, codes) = normalized(t, OptLevel::Speed);
    assert_eq!(shown, "\"7\"()");
    assert_eq!(codes, ["array-in-string", "array-in-string"]);
}

#[test]
fn adjacent_text_merges() {
    let t = tmpl(vec![text("a"), text("b"), text(""), text("c")], vec![]);
    let (shown, _) = normalized(t, OptLevel::Off);
    assert_eq!(shown, "\"abc\"()");
}

#[test]
fn ws_fmt_with_a_number_literal_folds_to_text() {
    let t = crate::prelude::ws_fmt!("x={}", 3.5);
    let (shown, _) = normalized(t.into_ir(), OptLevel::Speed);
    assert_eq!(shown, "\"x=3.50\"()");
}
