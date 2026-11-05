use ow_workshop_data::ValueId;

use crate::lower::testutil::num;
use crate::opt::testutil::*;

/// `Absolute Value(g0)`: a number that cannot be negative.
fn unsigned() -> crate::ir::Expr {
    val(ValueId::Abs, vec![g(0)])
}

/// `Sine(g0)`: a number of unknown sign.
fn signed() -> crate::ir::Expr {
    val(ValueId::Sin, vec![g(0)])
}

#[test]
fn add_folds_numbers_and_vectors() {
    assert_eq!(speed(&add(num(1.0), num(2.0))), "3");
    assert_eq!(
        speed(&add(v3(1.0, 2.0, 3.0), v3(4.0, 5.0, 6.0))),
        "V(5, 7, 9)"
    );
    assert_eq!(strict(&add(num(1.0), num(2.0))), "3");
}

#[test]
fn add_zero_is_non_strict() {
    for zero in [num(0.0), v3(0.0, 0.0, 0.0)] {
        assert_eq!(speed(&add(g(0), zero.clone())), "g0");
        assert_eq!(speed(&add(zero.clone(), g(0))), "g0");
        assert_eq!(
            strict(&add(g(0), zero.clone())),
            format!("(g0 + {})", sh(&zero))
        );
    }
}

#[test]
fn add_same_doubles() {
    assert_eq!(speed(&add(g(0), g(0))), "(2 * g0)");
    assert_eq!(strict(&add(g(0), g(0))), "(2 * g0)");
    assert_eq!(speed(&add(g(0), g(1))), "(g0 + g1)");
}

#[test]
fn add_of_random_is_not_doubled() {
    let roll = val(ValueId::RandomRandint, vec![num(1.0), num(5.0)]);
    assert_eq!(
        speed(&add(roll.clone(), roll)),
        "(RandomRandint(1, 5) + RandomRandint(1, 5))"
    );
}

#[test]
fn sub_folds_and_drops_zero() {
    assert_eq!(speed(&sub(num(5.0), num(7.0))), "-2");
    assert_eq!(
        speed(&sub(v3(5.0, 5.0, 5.0), v3(1.0, 2.0, 3.0))),
        "V(4, 3, 2)"
    );
    for mode in [speed, strict] {
        assert_eq!(mode(&sub(g(0), num(0.0))), "g0");
        assert_eq!(mode(&sub(g(0), v3(0.0, 0.0, 0.0))), "g0");
    }
}

#[test]
fn sub_same_is_a_times_zero() {
    // Not plain `0`: `A` may be a vector, which strict mode keeps.
    assert_eq!(strict(&sub(g(0), g(0))), "(g0 * 0)");
    // Non-strict, `A * 0` is `0` for any `A`.
    assert_eq!(speed(&sub(g(0), g(0))), "0");
    // For a number it folds to 0 even in strict mode.
    assert_eq!(strict(&sub(signed(), signed())), "0");
}

#[test]
fn mul_folds() {
    assert_eq!(speed(&mul(num(3.0), num(4.0))), "12");
    assert_eq!(
        speed(&mul(v3(1.0, 2.0, 3.0), v3(2.0, 2.0, 2.0))),
        "V(2, 4, 6)"
    );
    assert_eq!(speed(&mul(num(2.0), v3(1.0, 2.0, 3.0))), "V(2, 4, 6)");
    assert_eq!(speed(&mul(v3(1.0, 2.0, 3.0), num(2.0))), "V(2, 4, 6)");
}

#[test]
fn mul_one_is_non_strict() {
    assert_eq!(speed(&mul(g(0), num(1.0))), "g0");
    assert_eq!(speed(&mul(num(1.0), g(0))), "g0");
    assert_eq!(strict(&mul(g(0), num(1.0))), "(g0 * 1)");
}

#[test]
fn mul_zero_depends_on_type() {
    assert_eq!(speed(&mul(g(0), num(0.0))), "0");
    assert_eq!(speed(&mul(num(0.0), g(0))), "0");
    assert_eq!(strict(&mul(g(0), num(0.0))), "(g0 * 0)");
    assert_eq!(strict(&mul(signed(), num(0.0))), "0");
    assert_eq!(strict(&mul(num(0.0), unsigned())), "0");
}

#[test]
fn mul_same_unsigned_squares() {
    assert_eq!(speed(&mul(unsigned(), unsigned())), "(Abs(g0) ** 2)");
    assert_eq!(strict(&mul(unsigned(), unsigned())), "(Abs(g0) ** 2)");
    assert_eq!(speed(&mul(signed(), signed())), "(Sin(g0) * Sin(g0))");
    assert_eq!(speed(&mul(g(0), g(0))), "(g0 * g0)");
}

#[test]
fn div_folds() {
    assert_eq!(speed(&div(num(6.0), num(4.0))), "1.5");
    assert_eq!(
        speed(&div(v3(2.0, 4.0, 8.0), v3(2.0, 2.0, 2.0))),
        "V(1, 2, 4)"
    );
    assert_eq!(speed(&div(v3(2.0, 4.0, 8.0), num(2.0))), "V(1, 2, 4)");
}

#[test]
fn div_identities_are_non_strict() {
    assert_eq!(speed(&div(g(0), num(1.0))), "g0");
    assert_eq!(strict(&div(g(0), num(1.0))), "(g0 / 1)");
    assert_eq!(speed(&div(g(0), num(0.0))), "0");
    assert_eq!(speed(&div(num(0.0), g(0))), "0");
    assert_eq!(strict(&div(g(0), num(0.0))), "(g0 / 0)");
    assert_eq!(strict(&div(num(0.0), g(0))), "(0 / g0)");
}

#[test]
fn div_number_by_vector_warns_without_folding() {
    let e = div(num(1.0), v3(1.0, 2.0, 3.0));
    assert_eq!(speed(&e), "(1 / V(1, 2, 3))");
    assert_eq!(warnings(&e), ["type-check"]);
}

#[test]
fn modulo_folds_with_the_sign_of_the_dividend() {
    assert_eq!(speed(&modulo(num(7.0), num(3.0))), "1");
    assert_eq!(speed(&modulo(num(-7.0), num(3.0))), "-1");
    assert_eq!(speed(&modulo(num(7.0), num(-3.0))), "1");
    assert_eq!(speed(&modulo(num(-7.0), num(-3.0))), "-1");
}

#[test]
fn modulo_identities() {
    for mode in [speed, strict] {
        assert_eq!(mode(&modulo(g(0), g(0))), "0");
        assert_eq!(mode(&modulo(g(0), num(0.0))), "0");
        assert_eq!(mode(&modulo(num(0.0), g(0))), "0");
        assert_eq!(mode(&modulo(num(5.0), num(0.0))), "0");
    }
}

#[test]
fn pow_folds_and_negative_base_is_zero() {
    assert_eq!(speed(&pow(num(2.0), num(10.0))), "1024");
    assert_eq!(speed(&pow(num(-2.0), num(2.0))), "0");
    assert_eq!(speed(&pow(num(-2.0), g(0))), "0");
    assert_eq!(
        speed(&pow(num(10.0), num(9.0))),
        "(10 ** 9)",
        "the 10M guard"
    );
}

#[test]
fn pow_identities() {
    assert_eq!(speed(&pow(g(0), num(1.0))), "g0");
    assert_eq!(speed(&pow(num(0.0), g(0))), "0");
    assert_eq!(speed(&pow(num(1.0), g(0))), "1");
    assert_eq!(strict(&pow(g(0), num(1.0))), "g0");
}

#[test]
fn folds_obey_the_ten_million_guard() {
    assert_eq!(speed(&mul(num(10_000.0), num(1_000.0))), "10000000");
    assert_eq!(speed(&mul(num(10_000.0), num(1_001.0))), "(10000 * 1001)");
    assert_eq!(speed(&add(num(9_999_999.0), num(2.0))), "(9999999 + 2)");
}

#[test]
fn negate_literals_even_with_the_optimizer_off() {
    for mode in [off, speed, strict] {
        assert_eq!(mode(&neg(num(5.0))), "-5");
        assert_eq!(mode(&neg(num(-5.0))), "5");
        assert_eq!(mode(&neg(num(0.0))), "0");
    }
    assert_eq!(
        off(&neg(g(0))),
        "-g0",
        "other negations wait for the emitter"
    );
}

#[test]
fn negate_pushes_into_literal_factors() {
    assert_eq!(speed(&neg(mul(num(2.0), g(0)))), "(-2 * g0)");
    assert_eq!(speed(&neg(mul(g(0), num(2.0)))), "(g0 * -2)");
    assert_eq!(speed(&neg(div(num(2.0), g(0)))), "(-2 / g0)");
    assert_eq!(speed(&neg(div(g(0), num(2.0)))), "(g0 / -2)");
    assert_eq!(speed(&neg(modulo(num(2.0), g(0)))), "(-2 % g0)");
}

#[test]
fn negate_vectors_and_the_rest() {
    assert_eq!(speed(&neg(v3(1.0, -2.0, 0.0))), "V(-1, 2, 0)");
    assert_eq!(speed(&neg(g(0))), "(-1 * g0)");
    assert_eq!(speed(&neg(modulo(g(0), num(2.0)))), "(-1 * (g0 % 2))");
}
