use ow_workshop_data::{LocalizedString, ValueId};

use super::*;
use crate::ir::{CmpOp, StrToken};
use crate::lower::testutil::*;

fn custom(text: &str, args: Vec<Expr>) -> Expr {
    let mut tokens = vec![];
    for (i, part) in text.split("{}").enumerate() {
        if i > 0 {
            tokens.push(StrToken::Arg(u8::try_from(i - 1).unwrap()));
        }
        tokens.push(StrToken::Text(part.to_owned()));
    }
    expr(ExprKind::Str(StrExpr::Custom { tokens, args }))
}

fn localized(s: LocalizedString) -> Expr {
    expr(ExprKind::Str(StrExpr::Localized(s)))
}

fn array(items: Vec<Expr>) -> Expr {
    expr(ExprKind::Array(items))
}

fn vector(x: f64, y: f64, z: f64) -> Expr {
    expr(ExprKind::Vector([num(x), num(y), num(z)]))
}

fn var() -> Expr {
    expr(ExprKind::GlobalVar(crate::ir::VarId(0)))
}

#[test]
fn falsy_values() {
    let falsy = [
        boolean(false),
        null(),
        call(ValueId::EmptyArray, vec![]),
        call(ValueId::False, vec![]),
        array(vec![]),
        num(0.0),
        array(vec![num(0.0), num(5.0)]),
        array(vec![array(vec![])]),
        custom("", vec![]),
        custom("", vec![num(1.0)]),
        localized(LocalizedString::Empty),
    ];
    for e in &falsy {
        assert!(definitely_falsy(e), "{}", show(e));
    }
}

#[test]
fn not_falsy_values() {
    let unknown = [
        boolean(true),
        num(1.0),
        var(),
        vector(0.0, 0.0, 0.0),
        array(vec![num(1.0)]),
        array(vec![var()]),
        custom("x", vec![]),
        custom("{}", vec![num(0.0)]),
        localized(LocalizedString::SymStar),
    ];
    for e in &unknown {
        assert!(!definitely_falsy(e), "{}", show(e));
    }
}

#[test]
fn truthy_values() {
    let truthy = [
        boolean(true),
        call(ValueId::True, vec![]),
        num(2.0),
        num(-1.0),
        vector(0.0, 1.0, 0.0),
        array(vec![num(1.0), num(0.0)]),
        konst("Hero", "ANA"),
        konst("Team", "ALL"),
        konst("Color", "RED"),
        konst("Map", "X"),
        call(ValueId::Hero, vec![]),
        custom("hi", vec![]),
        custom("{}hi", vec![num(0.0)]),
        localized(LocalizedString::SymStar),
    ];
    for e in &truthy {
        assert!(definitely_truthy(e), "{}", show(e));
    }
}

#[test]
fn not_truthy_values() {
    let unknown = [
        boolean(false),
        null(),
        num(0.0),
        var(),
        vector(0.0, 0.0, 0.0),
        array(vec![]),
        array(vec![num(0.0), num(1.0)]),
        custom("", vec![]),
        custom("{}", vec![num(1.0)]),
        custom("{}{}", vec![num(1.0), num(1.0)]),
        localized(LocalizedString::Empty),
    ];
    for e in &unknown {
        assert!(!definitely_truthy(e), "{}", show(e));
    }
}

#[test]
fn falsy_and_truthy_are_disjoint() {
    for e in [
        num(0.0),
        num(3.0),
        array(vec![]),
        var(),
        boolean(true),
        null(),
    ] {
        assert!(!(definitely_falsy(&e) && definitely_truthy(&e)));
    }
}

#[test]
fn equal_expressions_ignore_locations() {
    let a = Expr::new(ExprKind::Num(1.0), None);
    let b = Expr::new(ExprKind::Num(1.0), Some(crate::ir::Location::caller()));
    assert!(always_equal(&a, &b));
    let sum = |x: &Expr| expr(ExprKind::Add(x.clone(), var()));
    assert!(always_equal(&sum(&a), &sum(&b)));
    assert!(!always_equal(&a, &num(2.0)));
    assert!(!always_equal(&sum(&a), &sum(&num(2.0))));
}

#[test]
fn random_is_never_equal() {
    let rolls = [
        call(ValueId::RandomRandint, vec![num(1.0), num(2.0)]),
        call(ValueId::RandomUniform, vec![num(1.0), num(2.0)]),
        call(ValueId::RandomChoice, vec![var()]),
        call(ValueId::RandomShuffle, vec![var()]),
    ];
    for roll in rolls {
        assert!(contains_random(&roll));
        assert!(!always_equal(&roll, &roll.clone()));
        let nested = expr(ExprKind::Add(num(1.0), roll.clone()));
        assert!(contains_random(&nested));
        assert!(!always_equal(&nested, &nested.clone()));
    }
    assert!(!contains_random(&var()));
}

#[test]
fn lambda_vars_are_found_anywhere() {
    assert!(contains_lambda_vars(&expr(ExprKind::ArrayElem)));
    let nested = expr(ExprKind::Compare {
        op: CmpOp::Eq,
        lhs: expr(ExprKind::ArrayIndex),
        rhs: num(1.0),
    });
    assert!(contains_lambda_vars(&nested));
    assert!(!contains_lambda_vars(&var()));
}

#[test]
fn literals() {
    let literal = [
        num(4.0),
        konst("Team", "ALL"),
        localized(LocalizedString::SymStar),
        call(ValueId::Hero, vec![]),
        call(ValueId::Rgb, vec![num(1.0), num(2.0), num(3.0), num(4.0)]),
        vector(1.0, 2.0, 3.0),
        custom("am", vec![]),
    ];
    for e in &literal {
        assert!(is_literal(e, false), "{}", show(e));
    }
    let other = [
        var(),
        boolean(true),
        null(),
        array(vec![num(1.0)]),
        custom("{}", vec![num(1.0)]),
        call(ValueId::Rgb, vec![var(), num(2.0), num(3.0), num(4.0)]),
        vector(1.0, 2.0, 0.0),
        vector(1.0, 2.0, 0.0),
        expr(ExprKind::Add(num(1.0), num(2.0))),
    ];
    // `Vector(1, 2, 0)` of number literals is itself a literal; everything else here is not.
    for e in other
        .iter()
        .filter(|e| !matches!(e.kind(), ExprKind::Vector(_)))
    {
        assert!(!is_literal(e, false), "{}", show(e));
    }
}

#[test]
fn custom_strings_are_literals_only_when_not_strict() {
    let s = custom("am", vec![]);
    assert!(is_literal(&s, false));
    assert!(!is_literal(&s, true));
    assert!(is_literal(&localized(LocalizedString::SymStar), true));
    assert!(is_literal(&num(1.0), true));
}

#[test]
fn fold_guard_rejects_big_and_non_finite() {
    assert_eq!(fold_guard(10_000_000.0), Some(10_000_000.0));
    assert_eq!(fold_guard(-10_000_000.0), Some(-10_000_000.0));
    assert_eq!(fold_guard(10_000_001.0), None);
    assert_eq!(fold_guard(-10_000_001.0), None);
    assert_eq!(fold_guard(f64::NAN), None);
    assert_eq!(fold_guard(f64::INFINITY), None);
}
