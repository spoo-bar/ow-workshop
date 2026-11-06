//! A shallow static type for rewrites that only hold for numbers, bools or vectors.

use ow_workshop_data::Ty;

use crate::ir::{Expr, ExprKind};

/// What an expression is known to evaluate to; `Unknown` when the workshop might coerce it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    /// Could be anything.
    Unknown,
    /// A number; `unsigned` when it cannot be negative.
    Number { unsigned: bool },
    /// A boolean.
    Bool,
    /// A vector.
    Vector,
}

impl Kind {
    /// Whether this is a number of any sign.
    pub(crate) fn is_number(self) -> bool {
        matches!(self, Self::Number { .. })
    }

    /// Whether this is a number that cannot be negative.
    pub(crate) fn is_unsigned(self) -> bool {
        self == Self::Number { unsigned: true }
    }
}

fn of_ty(ty: &Ty) -> Kind {
    use Ty as T;
    match ty {
        T::UnsignedFloat | T::UnsignedInt | T::UnsignedFloatLiteral | T::UnsignedIntLiteral => {
            Kind::Number { unsigned: true }
        }
        T::Float
        | T::Int
        | T::SignedFloat
        | T::SignedInt
        | T::FloatLiteral
        | T::IntLiteral
        | T::SignedFloatLiteral
        | T::SignedIntLiteral => Kind::Number { unsigned: false },
        T::Bool | T::BoolLiteral => Kind::Bool,
        T::Vector | T::Direction | T::Position | T::Velocity | T::Vec3(..) => Kind::Vector,
        _ => Kind::Unknown,
    }
}

/// The static kind of `expr`.
pub(crate) fn kind(expr: &Expr) -> Kind {
    use ExprKind as K;
    let number = |unsigned| Kind::Number { unsigned };
    match expr.kind() {
        K::Num(n) => number(*n >= 0.0),
        K::Bool(_) | K::Compare { .. } | K::And(..) | K::Or(..) | K::Not(_) => Kind::Bool,
        K::Vector(_) => Kind::Vector,
        K::Value { id, .. } => of_ty(&id.signature().ret),
        K::Neg(a) => {
            if kind(a).is_number() {
                number(false)
            } else {
                Kind::Unknown
            }
        }
        K::Add(a, b) | K::Sub(a, b) | K::Mul(a, b) | K::Div(a, b) | K::Mod(a, b) | K::Pow(a, b) => {
            let (ka, kb) = (kind(a), kind(b));
            if ka == Kind::Vector && kb == Kind::Vector {
                return Kind::Vector;
            }
            if !ka.is_number() || !kb.is_number() {
                return Kind::Unknown;
            }
            let both = ka.is_unsigned() && kb.is_unsigned();
            number(match expr.kind() {
                K::Add(..) | K::Mul(..) | K::Div(..) => both,
                K::Mod(..) | K::Pow(..) => ka.is_unsigned(),
                _ => false,
            })
        }
        K::IfThenElse { then, else_, .. } => {
            let (a, b) = (kind(then), kind(else_));
            if a == b {
                a
            } else if a.is_number() && b.is_number() {
                number(false)
            } else {
                Kind::Unknown
            }
        }
        _ => Kind::Unknown,
    }
}
