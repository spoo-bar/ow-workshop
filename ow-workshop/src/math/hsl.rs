//! `hsl`: a color from hue, saturation and lightness.

use crate::expr::Expr;
use crate::generated::api::{RgbArgs, max, min, rgb_with};
use crate::ir::ExprKind;
use crate::ty::{Color, IntoExpr, Num};

/// Why [`hsl`] rejected its arguments.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum HslError {
    /// A literal argument is outside its range.
    OutOfRange {
        /// The argument's name.
        arg: &'static str,
        /// The offending value.
        value: f64,
    },
    /// An argument is random, so its copies in the formula would differ.
    RandomArgument(&'static str),
}

impl std::fmt::Display for HslError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfRange { arg, value } => write!(f, "hsl: `{arg}` is {value}, out of range"),
            Self::RandomArgument(arg) => write!(f, "hsl: `{arg}` cannot be random"),
        }
    }
}

impl std::error::Error for HslError {}

/// Optional arguments of [`hsl_with`].
#[derive(Clone, Debug)]
pub struct HslArgs {
    /// Opacity, 0 to 255; default 255.
    pub alpha: Expr<Num>,
}

impl Default for HslArgs {
    fn default() -> Self {
        Self { alpha: 255.into() }
    }
}

/// A fully opaque color; `h` is 0 to 360, `s` and `l` are 0 to 1.
#[track_caller]
pub fn hsl(
    h: impl IntoExpr<Num>,
    s: impl IntoExpr<Num>,
    l: impl IntoExpr<Num>,
) -> Result<Expr<Color>, HslError> {
    hsl_with(h, s, l, HslArgs::default())
}

/// A color from hue, saturation, lightness and opacity.
///
/// Literal arguments are range-checked; `h`, `s` and `l` appear several times in the result,
/// so they cannot be random.
#[track_caller]
pub fn hsl_with(
    h: impl IntoExpr<Num>,
    s: impl IntoExpr<Num>,
    l: impl IntoExpr<Num>,
    opts: HslArgs,
) -> Result<Expr<Color>, HslError> {
    let (h, s, l) = (h.into_expr(), s.into_expr(), l.into_expr());
    for (arg, value, max) in [("h", &h, 360.0), ("s", &s, 1.0), ("l", &l, 1.0)] {
        check(arg, value, max)?;
        if crate::opt::contains_random(value.ir()) {
            return Err(HslError::RandomArgument(arg));
        }
    }
    check("a", &opts.alpha, 255.0)?;
    let rgb = RgbArgs { alpha: opts.alpha };
    Ok(rgb_with(
        channel(0, &h, &s, &l) * 255,
        channel(8, &h, &s, &l) * 255,
        channel(4, &h, &s, &l) * 255,
        rgb,
    ))
}

fn check(arg: &'static str, value: &Expr<Num>, max: f64) -> Result<(), HslError> {
    match value.ir().kind() {
        ExprKind::Num(n) if !(0.0..=max).contains(n) => {
            Err(HslError::OutOfRange { arg, value: *n })
        }
        _ => Ok(()),
    }
}

/// `l - s * Min(l, 1 - l) * Max(Min(Min(k - 3, 9 - k), 1), -1)` with `k = (n + h / 30) % 12`.
fn channel(offset: i32, h: &Expr<Num>, s: &Expr<Num>, l: &Expr<Num>) -> Expr<Num> {
    let phase = (Expr::<Num>::from(offset) + h / 30) % 12;
    let ramp = max(min(min(&phase - 3, Expr::<Num>::from(9) - &phase), 1), -1);
    l - s * min(l, Expr::<Num>::from(1) - l) * ramp
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opt::testutil::sh;

    fn text(c: Result<Expr<Color>, HslError>) -> String {
        sh(c.expect("valid").ir())
    }

    #[test]
    fn expands_to_a_custom_color() {
        let out = text(hsl(0, 1, 0.5));
        assert!(out.starts_with("Rgb(("), "{out}");
        assert!(out.ends_with(", 255)"), "{out}");
        // Red, green and blue use offsets 0, 8 and 4.
        assert!(out.contains("((0 + (0 / 30)) % 12)"), "{out}");
        assert!(out.contains("((8 + (0 / 30)) % 12)"), "{out}");
        assert!(out.contains("((4 + (0 / 30)) % 12)"), "{out}");
    }

    #[test]
    fn alpha_is_passed_through() {
        let out = text(hsl_with(0, 1, 0.5, HslArgs { alpha: 128.into() }));
        assert!(out.ends_with(", 128)"), "{out}");
    }

    #[test]
    fn literals_are_range_checked() {
        let bad = |r: Result<Expr<Color>, HslError>| r.unwrap_err();
        assert_eq!(
            bad(hsl(361, 0, 0)),
            HslError::OutOfRange {
                arg: "h",
                value: 361.0
            }
        );
        assert_eq!(
            bad(hsl(0, 1.5, 0)),
            HslError::OutOfRange {
                arg: "s",
                value: 1.5
            }
        );
        assert_eq!(
            bad(hsl(0, 0, -0.1)),
            HslError::OutOfRange {
                arg: "l",
                value: -0.1
            }
        );
        assert_eq!(
            bad(hsl_with(0, 0, 0, HslArgs { alpha: 256.into() })),
            HslError::OutOfRange {
                arg: "a",
                value: 256.0
            }
        );
        assert!(hsl(360, 1, 1).is_ok());
    }

    #[test]
    fn random_arguments_are_rejected() {
        let r = crate::generated::api::random::real(0, 360);
        assert_eq!(hsl(r, 1, 0.5).unwrap_err(), HslError::RandomArgument("h"));
    }
}
