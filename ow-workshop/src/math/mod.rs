//! Rounding, constants and helpers such as `sign`, `lerp`, `ln` and `hsl`.

mod consts;
mod directions;
mod funcs;
mod hsl;

pub use consts::*;
pub use funcs::{lerp, line_intersects_sphere, ln, log, sign};
pub use hsl::{HslArgs, HslError, hsl, hsl_with};

use ow_workshop_data::{Rounding, ValueId};

use crate::expr::Expr;
use crate::ir::{self, ExprKind};
use crate::ty::{IntoExpr, Num};

#[track_caller]
fn round_with(value: impl IntoExpr<Num>, mode: Rounding) -> Expr<Num> {
    let args = vec![
        value.into_expr().into_ir(),
        Expr::<Rounding>::from(mode).into_ir(),
    ];
    Expr::from_ir(ir::Expr::here(ExprKind::Value {
        id: ValueId::Round,
        args,
    }))
}

/// `Round To Integer(value, Up)`.
#[track_caller]
#[must_use]
pub fn ceil(value: impl IntoExpr<Num>) -> Expr<Num> {
    round_with(value, Rounding::RoundUp)
}

/// `Round To Integer(value, Down)`.
#[track_caller]
#[must_use]
pub fn floor(value: impl IntoExpr<Num>) -> Expr<Num> {
    round_with(value, Rounding::RoundDown)
}

/// `Round To Integer(value, To Nearest)`.
#[track_caller]
#[must_use]
pub fn round(value: impl IntoExpr<Num>) -> Expr<Num> {
    round_with(value, Rounding::RoundToNearest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounding_modes() {
        for (e, key) in [
            (ceil(1.5), Rounding::RoundUp.key()),
            (floor(1.5), Rounding::RoundDown.key()),
            (round(1.5), Rounding::RoundToNearest.key()),
        ] {
            let ExprKind::Value { id, args } = e.ir().kind() else {
                panic!("not a value")
            };
            assert_eq!(*id, ValueId::Round);
            assert!(
                matches!(args[1].kind(), ExprKind::Const(c) if c.enum_name == "Rounding" && c.key == key)
            );
        }
    }
}
