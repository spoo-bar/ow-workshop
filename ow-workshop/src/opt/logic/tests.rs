use ow_workshop_data::ValueId as V;

use crate::ir::CmpOp::{Eq, Ge, Gt, Le, Lt, Ne};
use crate::lower::testutil::{konst, null, num};
use crate::opt::testutil::*;

/// A boolean the optimizer knows nothing else about.
fn b() -> crate::ir::Expr {
    val(V::IsGameInProgress, vec![])
}

#[test]
fn not_of_constants() {
    for falsy in [fals(), null(), num(0.0), arr(vec![])] {
        assert_eq!(speed(&not(falsy)), "true");
    }
    for truthy in [tru(), num(2.0), konst("Hero", "ANA")] {
        assert_eq!(speed(&not(truthy)), "false");
    }
    assert_eq!(speed(&not(g(0))), "!g0");
}

#[test]
fn double_not_needs_a_bool() {
    assert_eq!(speed(&not(not(b()))), "IsGameInProgress()");
    assert_eq!(speed(&not(not(cmp(Eq, g(0), g(1))))), "(g0 == g1)");
    assert_eq!(speed(&not(not(g(0)))), "!!g0");
}

#[test]
fn not_alive_is_dead() {
    assert_eq!(speed(&not(val(V::IsAlive, vec![g(0)]))), "IsDead(g0)");
    assert_eq!(speed(&not(val(V::IsDead, vec![g(0)]))), "IsAlive(g0)");
}

#[test]
fn not_inverts_comparisons() {
    for (op, inverse) in [
        (Eq, "!="),
        (Ne, "=="),
        (Gt, "<="),
        (Ge, "<"),
        (Lt, ">="),
        (Le, ">"),
    ] {
        assert_eq!(
            speed(&not(cmp(op, g(0), g(1)))),
            format!("(g0 {inverse} g1)")
        );
    }
}

#[test]
fn and_with_a_falsy_operand() {
    assert_eq!(speed(&and(fals(), g(0))), "false");
    assert_eq!(speed(&and(null(), g(0))), "Null");
    assert_eq!(speed(&and(g(0), fals())), "false");
    assert_eq!(strict(&and(fals(), g(0))), "false");
    assert_eq!(strict(&and(g(0), fals())), "(g0 && false)");
}

#[test]
fn and_with_a_truthy_operand() {
    assert_eq!(speed(&and(tru(), g(0))), "g0");
    assert_eq!(speed(&and(g(0), tru())), "g0");
    assert_eq!(strict(&and(tru(), g(0))), "g0");
    assert_eq!(strict(&and(g(0), tru())), "(g0 && true)");
}

#[test]
fn and_of_equal_or_opposite_operands() {
    assert_eq!(speed(&and(g(0), g(0))), "g0");
    assert_eq!(strict(&and(g(0), g(0))), "g0");
    assert_eq!(speed(&and(g(0), not(g(0)))), "false");
    assert_eq!(speed(&and(not(g(0)), g(0))), "false");
    assert_eq!(strict(&and(g(0), not(g(0)))), "(g0 && !g0)");
}

#[test]
fn and_of_nots_becomes_not_or() {
    assert_eq!(speed(&and(not(g(0)), not(g(1)))), "!(g0 || g1)");
    assert_eq!(strict(&and(not(g(0)), not(g(1)))), "!(g0 || g1)");
}

#[test]
fn or_with_a_falsy_operand() {
    assert_eq!(speed(&or(fals(), g(0))), "g0");
    assert_eq!(speed(&or(g(0), fals())), "g0");
    assert_eq!(strict(&or(fals(), g(0))), "g0");
    assert_eq!(strict(&or(g(0), fals())), "(g0 || false)");
}

#[test]
fn or_with_a_truthy_operand() {
    assert_eq!(speed(&or(tru(), g(0))), "true");
    assert_eq!(speed(&or(g(0), tru())), "true");
    assert_eq!(speed(&or(num(2.0), g(0))), "2");
    assert_eq!(strict(&or(tru(), g(0))), "true");
    assert_eq!(strict(&or(g(0), tru())), "(g0 || true)");
}

#[test]
fn or_of_equal_or_opposite_operands() {
    assert_eq!(speed(&or(g(0), g(0))), "g0");
    assert_eq!(speed(&or(g(0), not(g(0)))), "true");
    assert_eq!(speed(&or(not(g(0)), g(0))), "true");
    assert_eq!(strict(&or(g(0), not(g(0)))), "(g0 || !g0)");
}

#[test]
fn or_of_nots_becomes_not_and() {
    assert_eq!(speed(&or(not(g(0)), not(g(1)))), "!(g0 && g1)");
}

#[test]
fn numeric_comparisons_fold() {
    for (op, a, c, expected) in [
        (Eq, 1.0, 1.0, "true"),
        (Eq, 1.0, 0.0, "false"),
        (Ne, 1.0, 0.0, "true"),
        (Lt, 1.0, 2.0, "true"),
        (Lt, 1.0, 0.0, "false"),
        (Le, 1.0, 0.0, "false"),
        (Le, 1.0, 1.0, "true"),
        (Gt, 1.0, 0.0, "true"),
        (Ge, 1.0, 2.0, "false"),
    ] {
        assert_eq!(speed(&cmp(op, num(a), num(c))), expected);
    }
}

#[test]
fn comparing_a_value_with_itself() {
    for (op, expected) in [
        (Eq, "true"),
        (Le, "true"),
        (Ge, "true"),
        (Ne, "false"),
        (Lt, "false"),
        (Gt, "false"),
    ] {
        assert_eq!(speed(&cmp(op, g(0), g(0))), expected);
        assert_eq!(strict(&cmp(op, g(0), g(0))), expected);
    }
}

#[test]
fn random_values_are_not_comparable_with_themselves() {
    let roll = val(V::RandomRandint, vec![num(1.0), num(9.0)]);
    assert_eq!(
        speed(&cmp(Eq, roll.clone(), roll)),
        "(RandomRandint(1, 9) == RandomRandint(1, 9))"
    );
}

#[test]
fn different_literals_are_not_equal() {
    let ana = konst("Hero", "ANA");
    let kings_row = konst("Map", "KINGS_ROW");
    assert_eq!(speed(&cmp(Eq, ana.clone(), kings_row.clone())), "false");
    assert_eq!(speed(&cmp(Ne, ana.clone(), kings_row)), "true");
    assert_eq!(speed(&cmp(Eq, ana.clone(), ana)), "true");
}

#[test]
fn custom_strings_are_literals_only_when_not_strict() {
    let e = cmp(Eq, cstr("am"), cstr("**"));
    assert_eq!(speed(&e), "false");
    assert_eq!(strict(&e), "(\"am\"() == \"**\"())");
}

#[test]
fn a_literal_is_not_compared_with_a_formatted_string() {
    let formatted = crate::ir::Expr::new(
        crate::ir::ExprKind::Str(crate::ir::StrExpr::Custom {
            tokens: vec![
                crate::ir::StrToken::Text("azer".to_owned()),
                crate::ir::StrToken::Arg(0),
            ],
            args: vec![g(0)],
        }),
        None,
    );
    let e = cmp(Eq, konst("Hero", "ANA"), formatted);
    assert_eq!(speed(&e), speed(&e.clone()));
    assert!(speed(&e).starts_with("(Hero.ANA == "));
}

#[test]
fn comparing_with_a_falsy_value() {
    assert_eq!(speed(&cmp(Eq, b(), fals())), "!IsGameInProgress()");
    assert_eq!(speed(&cmp(Eq, fals(), b())), "!IsGameInProgress()");
    assert_eq!(speed(&cmp(Eq, g(0), num(0.0))), "!g0");
    assert_eq!(speed(&cmp(Ne, b(), fals())), "IsGameInProgress()");
    // `!=` only simplifies for bools: `g0 != 0` is not `g0`.
    assert_eq!(speed(&cmp(Ne, g(0), num(0.0))), "(g0 != 0)");
}

#[test]
fn comparing_with_true() {
    assert_eq!(speed(&cmp(Eq, b(), tru())), "IsGameInProgress()");
    assert_eq!(speed(&cmp(Ne, b(), tru())), "!IsGameInProgress()");
    assert_eq!(speed(&cmp(Eq, g(0), tru())), "(g0 == true)");
}

#[test]
fn if_then_else_with_constant_conditions() {
    assert_eq!(speed(&ite(tru(), g(0), g(1))), "g0");
    assert_eq!(speed(&ite(num(3.0), g(0), g(1))), "g0");
    assert_eq!(speed(&ite(fals(), g(0), g(1))), "g1");
    assert_eq!(speed(&ite(null(), g(0), g(1))), "g1");
}

#[test]
fn if_then_else_with_equal_branches() {
    assert_eq!(speed(&ite(g(2), g(0), g(0))), "g0");
    assert_eq!(speed(&ite(g(2), g(0), g(1))), "(g2 ? g0 : g1)");
}

#[test]
fn if_then_else_swaps_a_negated_condition() {
    assert_eq!(speed(&ite(not(g(2)), g(0), g(1))), "(g2 ? g1 : g0)");
}

#[test]
fn nothing_changes_with_the_optimizer_off() {
    assert_eq!(off(&and(tru(), g(0))), "(true && g0)");
    assert_eq!(off(&not(cmp(Eq, g(0), g(1)))), "!(g0 == g1)");
}

/// The `strict optimizations` rule of the operators fixture: none of these may change.
#[test]
fn the_strict_fixture_rule_is_unchanged() {
    let unchanged = [
        cmp(Eq, cstr("am"), cstr("**")),
        add(g(1), num(0.0)),
        add(g(1), v3(0.0, 0.0, 0.0)),
        mul(g(1), num(1.0)),
        div(g(1), num(1.0)),
        div(num(0.0), g(1)),
        and(g(1), fals()),
        and(g(1), tru()),
        and(g(1), not(g(1))),
        or(g(1), fals()),
        or(g(1), tru()),
        or(g(1), not(g(1))),
    ];
    for e in &unchanged {
        assert_eq!(strict(e), sh(e), "{}", sh(e));
    }
    // `B - B` and `B * 0` keep the multiplication by zero; `len(B) * 0` is `0`.
    assert_eq!(strict(&sub(g(1), g(1))), "(g1 * 0)");
    assert_eq!(strict(&mul(g(1), num(0.0))), "(g1 * 0)");
    assert_eq!(strict(&mul(val(V::Len, vec![g(1)]), num(0.0))), "0");
}
