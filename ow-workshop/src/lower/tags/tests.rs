use super::*;
use crate::ir::{StmtKind, VarDecl};
use crate::lower::testutil::*;
use crate::options::{CompileOptions, OptLevel};

fn text(s: &str) -> StrToken {
    StrToken::Text(s.to_owned())
}

fn tmpl(tokens: Vec<StrToken>, args: Vec<Expr>) -> Expr {
    expr(ExprKind::Str(StrExpr::Custom { tokens, args }))
}

fn lowered(e: Expr, tags: bool) -> (String, Vec<&'static str>) {
    let mut p = program(vec![StmtKind::Action {
        id: ow_workshop_data::ActionId::Wait,
        args: vec![e],
    }]);
    p.globals.declare(VarDecl::new("a"));
    p.globals.declare(VarDecl::new(crate::tags::HOLYGRAIL));
    let options = CompileOptions {
        tags,
        ..options(OptLevel::Off)
    };
    let d = run(lower, &mut p, &options).expect("lowers");
    let StmtKind::Action { args, .. } = &p.rules[0].body.stmts[0].kind else {
        panic!("not an action");
    };
    (show(&args[0]), codes(&d))
}

#[test]
fn the_less_than_of_each_tag_becomes_the_holygrail() {
    let (shown, codes) = lowered(tmpl(vec![text("<fgFF0000FF>Red</fg>")], vec![]), true);
    assert_eq!(shown, "\"{0}fgFF0000FF>Red{1}/fg>\"(g1, g1)");
    assert_eq!(codes, Vec::<&str>::new());
}

#[test]
fn tx_ids_are_autocorrected_after_the_holygrail() {
    let (shown, _) = lowered(tmpl(vec![text("<tx2DD21>")], vec![]), true);
    assert_eq!(shown, "\"{0}txC0000000002DD21>\"(g1)");
}

#[test]
fn placeholders_inside_tags_are_kept() {
    let a = expr(ExprKind::GlobalVar(crate::ir::VarId(0)));
    let (shown, _) = lowered(
        tmpl(vec![text("<tx"), StrToken::Arg(0), text(">")], vec![a]),
        true,
    );
    assert_eq!(shown, "\"{0}tx{1}>\"(g1, g0)");
}

#[test]
fn malformed_texture_ids_warn() {
    let (_, codes) = lowered(tmpl(vec![text("<txC12345678>")], vec![]), true);
    assert_eq!(codes, ["malformatted-tx"]);
}

#[test]
fn tags_while_disabled_warn_and_stay_text() {
    let (shown, codes) = lowered(tmpl(vec![text("<fg00FF00FF>x")], vec![]), false);
    assert_eq!(shown, "\"<fg00FF00FF>x\"()");
    assert_eq!(codes, ["tags"]);
}

#[test]
fn other_text_is_untouched() {
    let (shown, codes) = lowered(tmpl(vec![text("a < b and <b>")], vec![]), true);
    assert_eq!(shown, "\"a < b and <b>\"()");
    assert_eq!(codes, Vec::<&str>::new());
}
