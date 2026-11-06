use ow_workshop_data::{LocalizedString, ValueId};

use crate::emit::testutil::{expr_in, global, player_var, text};
use crate::ir::{CmpOp, Expr, ExprKind, StrExpr, StrToken};
use crate::lower::testutil::{boolean, call, expr, konst, null, num};
use ow_workshop_data::Lang;

fn custom(tokens: Vec<StrToken>, args: Vec<Expr>) -> Expr {
    expr(ExprKind::Str(StrExpr::Custom { tokens, args }))
}

fn t(s: &str) -> StrToken {
    StrToken::Text(s.to_owned())
}

#[test]
fn literals() {
    assert_eq!(text(&num(1.5)), "1.5");
    assert_eq!(text(&boolean(true)), "True");
    assert_eq!(text(&boolean(false)), "False");
    assert_eq!(text(&null()), "Null");
}

#[test]
fn zero_argument_values_have_no_parentheses() {
    assert_eq!(text(&call(ValueId::EventPlayer, vec![])), "Event Player");
}

#[test]
fn general_form() {
    let e = call(ValueId::IsAlive, vec![call(ValueId::EventPlayer, vec![])]);
    assert_eq!(text(&e), "Is Alive(Event Player)");
}

#[test]
fn variable_reads() {
    assert_eq!(text(&global(0)), "Global.A");
    let e = player_var(call(ValueId::EventPlayer, vec![]), 0);
    assert_eq!(text(&e), "(Event Player).score");
}

#[test]
fn compare_and_logic() {
    let cmp = expr(ExprKind::Compare {
        op: CmpOp::Le,
        lhs: num(1.0),
        rhs: num(2.0),
    });
    assert_eq!(text(&cmp), "Compare(1, <=, 2)");
    let and = expr(ExprKind::And(boolean(true), boolean(false)));
    assert_eq!(text(&and), "And(True, False)");
    assert_eq!(text(&expr(ExprKind::Or(num(1.0), num(2.0)))), "Or(1, 2)");
    assert_eq!(text(&expr(ExprKind::Not(boolean(true)))), "Not(True)");
}

#[test]
fn arithmetic() {
    type Make = fn(Expr, Expr) -> ExprKind;
    let pairs: [(Make, &str); 6] = [
        (ExprKind::Add, "Add"),
        (ExprKind::Sub, "Subtract"),
        (ExprKind::Mul, "Multiply"),
        (ExprKind::Div, "Divide"),
        (ExprKind::Mod, "Modulo"),
        (ExprKind::Pow, "Raise To Power"),
    ];
    for (kind, name) in pairs {
        let e = expr(kind(num(1.0), num(2.0)));
        assert_eq!(text(&e), format!("{name}(1, 2)"));
    }
    assert_eq!(
        text(&expr(ExprKind::Neg(global(0)))),
        "Multiply(-1, Global.A)"
    );
}

#[test]
fn arrays_and_vectors() {
    assert_eq!(text(&expr(ExprKind::Array(vec![]))), "Empty Array");
    let array = expr(ExprKind::Array(vec![num(1.0), num(2.0), num(3.0)]));
    assert_eq!(text(&array), "Array(1, 2, 3)");
    let vector = expr(ExprKind::Vector([num(1.0), num(0.0), num(-1.0)]));
    assert_eq!(text(&vector), "Vector(1, 0, -1)");
    let index = expr(ExprKind::Index {
        array: global(0),
        index: num(2.0),
    });
    assert_eq!(text(&index), "Value In Array(Global.A, 2)");
    assert_eq!(text(&expr(ExprKind::ArrayElem)), "Current Array Element");
    assert_eq!(text(&expr(ExprKind::ArrayIndex)), "Current Array Index");
}

#[test]
fn if_then_else() {
    let e = expr(ExprKind::IfThenElse {
        cond: boolean(true),
        then: num(1.0),
        else_: num(2.0),
    });
    assert_eq!(text(&e), "If-Then-Else(True, 1, 2)");
}

#[test]
fn wrapped_constants() {
    assert_eq!(text(&konst("Hero", "ana")), "Hero(Ana)");
    assert_eq!(text(&konst("Map", "hanamura")), "Map(Hanamura)");
    assert_eq!(text(&konst("Gamemode", "assault")), "Game Mode(Assault)");
    assert_eq!(text(&konst("Button", "INTERACT")), "Button(Interact)");
    assert_eq!(text(&konst("Color", "WHITE")), "Color(White)");
}

#[test]
fn bare_constants() {
    assert_eq!(text(&konst("Team", "1")), "Team 1");
    assert_eq!(text(&konst("Team", "2")), "Team 2");
    assert_eq!(text(&konst("Team", "ALL")), "All Teams");
    assert_eq!(text(&konst("Wait", "IGNORE_CONDITION")), "Ignore Condition");
}

#[test]
fn custom_strings_drop_trailing_nulls() {
    let one = custom(
        vec![t("Hi "), StrToken::Arg(0)],
        vec![num(1.0), null(), null()],
    );
    assert_eq!(text(&one), "Custom String(\"Hi {0}\", 1)");
    let none = custom(vec![t("plain \"q\"\n")], vec![null(), null(), null()]);
    assert_eq!(text(&none), "Custom String(\"plain \\\"q\\\"\\n\")");
}

#[test]
fn custom_string_keeps_nulls_before_the_last_argument() {
    let e = custom(
        vec![StrToken::Arg(0), StrToken::Arg(2)],
        vec![null(), null(), num(7.0)],
    );
    assert_eq!(text(&e), "Custom String(\"{0}{2}\", Null, Null, 7)");
}

#[test]
fn localized_strings() {
    let phrase = LocalizedString::from_en("Hello").unwrap_or(LocalizedString::Empty);
    let e = expr(ExprKind::Str(StrExpr::Localized(phrase)));
    let printed = text(&e);
    assert!(
        printed.starts_with("String(\"") && printed.ends_with("\")"),
        "{printed}"
    );
}

#[test]
fn values_are_localized() {
    let e = call(ValueId::EventPlayer, vec![]);
    assert_eq!(
        expr_in(Lang::JaJp, &e),
        ValueId::EventPlayer.name(Lang::JaJp)
    );
    assert_ne!(expr_in(Lang::JaJp, &e), text(&e));
    assert_eq!(
        expr_in(Lang::DeDe, &boolean(true)),
        ValueId::True.name(Lang::DeDe)
    );
}
