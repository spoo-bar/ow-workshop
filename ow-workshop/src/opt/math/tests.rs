use ow_workshop_data::ValueId as V;

use crate::lower::testutil::num;
use crate::opt::testutil::*;

fn f(id: V, args: Vec<crate::ir::Expr>) -> String {
    speed(&val(id, args))
}

#[test]
fn unary_math_folds() {
    assert_eq!(f(V::Abs, vec![num(-3.0)]), "3");
    assert_eq!(f(V::Sqrt, vec![num(9.0)]), "3");
    assert_eq!(f(V::Sqrt, vec![num(-4.0)]), "0");
    assert_eq!(f(V::Abs, vec![g(0)]), "Abs(g0)");
}

#[test]
fn rounding_modes() {
    let round = |x: f64, key| f(V::Round, vec![num(x), rounding(key)]);
    assert_eq!(round(2.5, "__roundDown__"), "2");
    assert_eq!(round(2.1, "__roundUp__"), "3");
    assert_eq!(round(2.5, "__roundToNearest__"), "3");
    assert_eq!(round(-2.5, "__roundDown__"), "-3");
    assert_eq!(
        f(V::Round, vec![g(0), rounding("__roundUp__")]),
        "Round(g0, Rounding.__roundUp__)"
    );
}

#[test]
fn trigonometry_folds() {
    assert_eq!(f(V::Sin, vec![num(0.0)]), "0");
    assert_eq!(f(V::Cos, vec![num(0.0)]), "1");
    assert_eq!(f(V::Tan, vec![num(0.0)]), "0");
    assert_eq!(f(V::SinDeg, vec![num(90.0)]), "1");
    assert_eq!(f(V::CosDeg, vec![num(0.0)]), "1");
    assert_eq!(f(V::TanDeg, vec![num(0.0)]), "0");
}

#[test]
fn inverse_trigonometry_clamps_its_input() {
    assert_eq!(f(V::Asin, vec![num(0.0)]), "0");
    assert_eq!(f(V::AsinDeg, vec![num(1.0)]), "90");
    assert_eq!(f(V::AsinDeg, vec![num(5.0)]), "90");
    assert_eq!(f(V::AcosDeg, vec![num(-5.0)]), "180");
    assert_eq!(f(V::Acos, vec![num(1.0)]), "0");
    assert_eq!(f(V::Atan2Deg, vec![num(1.0), num(1.0)]), "45");
    assert_eq!(f(V::Atan2, vec![num(0.0), num(1.0)]), "0");
}

#[test]
fn distance_and_magnitude() {
    assert_eq!(
        f(V::Distance, vec![v3(0.0, 0.0, 0.0), v3(3.0, 4.0, 0.0)]),
        "5"
    );
    assert_eq!(f(V::Magnitude, vec![v3(0.0, 3.0, 4.0)]), "5");
    assert_eq!(
        f(V::Distance, vec![v3(0.0, 0.0, 0.0), g(0)]),
        "Magnitude(g0)"
    );
    assert_eq!(
        f(V::Distance, vec![g(0), v3(0.0, 0.0, 0.0)]),
        "Magnitude(g0)"
    );
    assert_eq!(f(V::Distance, vec![g(0), g(1)]), "Distance(g0, g1)");
}

#[test]
fn normalize_and_direction() {
    assert_eq!(f(V::Normalize, vec![v3(0.0, 0.0, 0.0)]), "V(0, 0, 0)");
    assert_eq!(f(V::Normalize, vec![v3(0.0, 5.0, 0.0)]), "V(0, 1, 0)");
    assert_eq!(
        f(V::Normalize, vec![val(V::VectorTowards, vec![g(0), g(1)])]),
        "DirectionTowards(g0, g1)"
    );
    assert_eq!(
        f(
            V::DirectionTowards,
            vec![v3(1.0, 1.0, 1.0), v3(1.0, 4.0, 1.0)]
        ),
        "V(0, 1, 0)"
    );
    assert_eq!(
        f(V::VectorTowards, vec![v3(1.0, 2.0, 3.0), v3(4.0, 6.0, 8.0)]),
        "V(3, 4, 5)"
    );
}

#[test]
fn products() {
    assert_eq!(
        f(V::CrossProduct, vec![v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0)]),
        "V(0, 0, 1)"
    );
    assert_eq!(
        f(V::DotProduct, vec![v3(1.0, 2.0, 3.0), v3(4.0, 5.0, 6.0)]),
        "32"
    );
    assert_eq!(
        f(V::Sqrt, vec![val(V::DotProduct, vec![g(0), g(0)])]),
        "Magnitude(g0)"
    );
    assert_eq!(
        f(V::Sqrt, vec![val(V::DotProduct, vec![g(0), g(1)])]),
        "Sqrt(DotProduct(g0, g1))"
    );
}

#[test]
fn min_and_max() {
    assert_eq!(f(V::Min, vec![num(3.0), num(2.0)]), "2");
    assert_eq!(f(V::Max, vec![num(3.0), num(2.0)]), "3");
    // Only the literal that can win is kept.
    let nested = |id| val(id, vec![val(id, vec![g(0), num(3.0)]), num(5.0)]);
    assert_eq!(speed(&nested(V::Min)), "Min(g0, 3)");
    assert_eq!(speed(&nested(V::Max)), "Max(g0, 5)");
    assert_eq!(f(V::Min, vec![g(0), num(3.0)]), "Min(g0, 3)");
    assert_eq!(f(V::Min, vec![g(0), g(1)]), "Min(g0, g1)");
}

#[test]
fn nothing_folds_when_the_optimizer_is_off() {
    assert_eq!(off(&val(V::Abs, vec![num(-3.0)])), "Abs(-3)");
    assert_eq!(off(&add(num(1.0), num(2.0))), "(1 + 2)");
}
