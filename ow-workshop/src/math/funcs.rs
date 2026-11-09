//! Sign, interpolation, logarithms and sphere tests.

use super::INFINITY;
use crate::expr::Expr;
use crate::generated::api::{angle_between_vectors, asin_deg, distance};
use crate::ir::{self, ExprKind};
use crate::ty::{Bool, Direction, IntoExpr, Num, Position};

/// −1, 0 or 1 by the sign of `x`; relies on the game clamping huge products.
#[track_caller]
#[must_use]
pub fn sign(x: impl IntoExpr<Num>) -> Expr<Num> {
    x.into_expr() * INFINITY * INFINITY / INFINITY / 10
}

/// `a` at `t = 0`, `b` at `t = 1`, linearly in between.
#[track_caller]
#[must_use]
pub fn lerp(a: impl IntoExpr<Num>, b: impl IntoExpr<Num>, t: impl IntoExpr<Num>) -> Expr<Num> {
    let t = t.into_expr();
    a.into_expr() * (Expr::<Num>::from(1) - &t) + b.into_expr() * t
}

/// Natural logarithm; a positive literal is folded at build time.
#[track_caller]
#[must_use]
pub fn ln(x: impl IntoExpr<Num>) -> Expr<Num> {
    let x = x.into_expr();
    if let ExprKind::Num(n) = x.ir().kind() {
        let folded = n.ln();
        if *n > 0.0 && folded.is_finite() {
            return Expr::from_ir(ir::Expr::here(ExprKind::Num(folded)));
        }
    }
    Expr::<Num>::from(10000) * (x.pow(0.0001) - 1)
}

/// Logarithm of `x` in `base`.
#[track_caller]
#[must_use]
pub fn log(x: impl IntoExpr<Num>, base: impl IntoExpr<Num>) -> Expr<Num> {
    ln(x) / ln(base)
}

/// Whether the line from `start` along `dir` hits the sphere at `center` with radius `r`.
#[track_caller]
#[must_use]
pub fn line_intersects_sphere(
    start: impl IntoExpr<Position>,
    dir: impl IntoExpr<Direction>,
    center: impl IntoExpr<Position>,
    r: impl IntoExpr<Num>,
) -> Expr<Bool> {
    let start = start.into_expr();
    // The workshop's vector types are interchangeable; only the typed API tells them apart.
    let start_as_direction: Expr<Direction> = start.clone().erase().cast();
    angle_between_vectors(start_as_direction, dir)
        .le(asin_deg(r.into_expr() / distance(start, center)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::vect;
    use crate::math::{E, PI};
    use crate::opt::testutil::sh;

    fn show<T>(e: &Expr<T>) -> String {
        sh(e.ir())
    }

    #[test]
    fn sign_relies_on_clamping() {
        let x = Expr::<Num>::from(5);
        assert_eq!(
            show(&sign(x)),
            "((((5 * 999999999999) * 999999999999) / 999999999999) / 10)"
        );
    }

    #[test]
    fn lerp_blends_linearly() {
        let t = Expr::<Num>::from(0.25);
        assert_eq!(show(&lerp(10, 20, t)), "((10 * (1 - 0.25)) + (20 * 0.25))");
    }

    #[test]
    fn ln_folds_literals() {
        assert_eq!(show(&ln(E)), "1");
        assert_eq!(show(&ln(1)), "0");
        assert_eq!(show(&ln(PI)), format!("{}", PI.ln()));
    }

    #[test]
    fn ln_of_a_non_literal_uses_the_power_trick() {
        let x = crate::generated::api::match_time();
        let text = show(&ln(x));
        assert!(text.starts_with("(10000 * (("), "{text}");
        assert!(text.ends_with(" ** 0.0001) - 1))"), "{text}");
    }

    #[test]
    fn ln_does_not_fold_what_has_no_real_logarithm() {
        assert!(show(&ln(0)).contains("** 0.0001"));
        assert!(show(&ln(-1)).contains("** 0.0001"));
    }

    #[test]
    fn log_divides_two_logarithms() {
        assert_eq!(show(&log(E, E)), "(1 / 1)");
    }

    #[test]
    fn line_intersects_sphere_compares_angles() {
        let e = line_intersects_sphere(vect(0, 0, 0), vect(0, 0, 1), vect(0, 0, 5), 2);
        assert_eq!(
            show(&e),
            "(AngleBetweenVectors(V(0, 0, 0), V(0, 0, 1)) <= AsinDeg((2 / Distance(V(0, 0, 0), V(0, 0, 5)))))"
        );
    }
}
