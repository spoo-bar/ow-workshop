//! Typed expressions and their literals.

use std::marker::PhantomData;

use crate::ir::{self, ConstRef, ExprKind, StrExpr};
use crate::ty::{Any, Array, Bool, ConstEnum, Int, IntoExpr, Is, Num, Str, ValueType, Vector};

/// A workshop value of type `T`. Cheap to clone; it has no `PartialEq` (use `eq`, `ne`, …).
pub struct Expr<T> {
    ir: ir::Expr,
    _type: PhantomData<fn() -> T>,
}

impl<T> Clone for Expr<T> {
    fn clone(&self) -> Self {
        Self {
            ir: self.ir.clone(),
            _type: PhantomData,
        }
    }
}

impl<T> std::fmt::Debug for Expr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Expr").field(&self.ir).finish()
    }
}

// Used by the operator, variable and statement APIs.
#[allow(dead_code)]
impl<T> Expr<T> {
    /// Wraps an untyped expression.
    pub(crate) fn from_ir(ir: ir::Expr) -> Self {
        Self {
            ir,
            _type: PhantomData,
        }
    }

    /// A literal without a source location.
    pub(crate) fn literal(kind: ExprKind) -> Self {
        Self::from_ir(ir::Expr::new(kind, None))
    }

    /// The untyped expression.
    pub(crate) fn ir(&self) -> &ir::Expr {
        &self.ir
    }

    /// Unwraps the untyped expression.
    pub(crate) fn into_ir(self) -> ir::Expr {
        self.ir
    }
}

impl<T: ConstEnum> Expr<T> {
    /// A member of a generated constant enum.
    pub(crate) fn constant(value: T) -> Self {
        Self::literal(ExprKind::Const(ConstRef {
            enum_name: T::NAME,
            key: value.key(),
        }))
    }
}

impl Expr<Any> {
    /// Reinterprets the value as `T`; the workshop does not check this.
    #[must_use]
    pub fn cast<T: ValueType>(self) -> Expr<T> {
        Expr::from_ir(self.into_ir())
    }
}

impl<T: ValueType> Expr<T> {
    /// Forgets the static type.
    #[must_use]
    pub fn erase(self) -> Expr<Any> {
        Expr::from_ir(self.into_ir())
    }
}

/// `Null`, as any type.
#[track_caller]
#[must_use]
pub fn null<T: ValueType>() -> Expr<T> {
    Expr::from_ir(ir::Expr::here(ExprKind::Null))
}

/// `Vector(x, y, z)`.
#[track_caller]
#[must_use]
pub fn vect(x: impl IntoExpr<Num>, y: impl IntoExpr<Num>, z: impl IntoExpr<Num>) -> Expr<Vector> {
    let parts = [x.into_expr(), y.into_expr(), z.into_expr()].map(Expr::into_ir);
    Expr::from_ir(ir::Expr::here(ExprKind::Vector(parts)))
}

macro_rules! number_literals {
    ($($t:ty),*) => {$(
        impl From<$t> for Expr<Num> {
            fn from(n: $t) -> Self {
                Self::literal(ExprKind::Num(f64::from(n)))
            }
        }
        impl IntoExpr<Num> for $t {
            fn into_expr(self) -> Expr<Num> {
                self.into()
            }
        }
    )*};
}

number_literals!(f64, i32, u32);

macro_rules! int_literals {
    ($($t:ty),*) => {$(
        impl From<$t> for Expr<Int> {
            fn from(n: $t) -> Self {
                Self::literal(ExprKind::Num(f64::from(n)))
            }
        }
        impl IntoExpr<Int> for $t {
            fn into_expr(self) -> Expr<Int> {
                self.into()
            }
        }
    )*};
}

int_literals!(i32, u32);

macro_rules! any_literals {
    ($($t:ty => $m:ty),*) => {$(
        impl IntoExpr<Any> for $t {
            fn into_expr(self) -> Expr<Any> {
                Expr::<$m>::from(self).erase()
            }
        }
    )*};
}

any_literals!(f64 => Num, i32 => Num, u32 => Num, bool => Bool, &str => Str, String => Str);

impl From<bool> for Expr<Bool> {
    fn from(b: bool) -> Self {
        Self::literal(ExprKind::Bool(b))
    }
}

impl IntoExpr<Bool> for bool {
    fn into_expr(self) -> Expr<Bool> {
        self.into()
    }
}

impl From<&str> for Expr<Str> {
    fn from(s: &str) -> Self {
        let mut tokens = Vec::new();
        crate::fmt::push_text(&mut tokens, s);
        Self::literal(ExprKind::Str(StrExpr::Custom {
            tokens,
            args: Vec::new(),
        }))
    }
}

impl From<String> for Expr<Str> {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

impl IntoExpr<Str> for &str {
    fn into_expr(self) -> Expr<Str> {
        self.into()
    }
}

impl IntoExpr<Str> for String {
    fn into_expr(self) -> Expr<Str> {
        self.into()
    }
}

impl<T: ValueType, S: IntoExpr<T>> IntoExpr<Array<T>> for Vec<S> {
    fn into_expr(self) -> Expr<Array<T>> {
        let items = self.into_iter().map(|s| s.into_expr().into_ir()).collect();
        Expr::literal(ExprKind::Array(items))
    }
}

impl<T: ValueType, S: IntoExpr<T>, const N: usize> IntoExpr<Array<T>> for [S; N] {
    fn into_expr(self) -> Expr<Array<T>> {
        Vec::from(self).into_expr()
    }
}

impl<T: Is<T>> From<Vec<Expr<T>>> for Expr<Array<T>> {
    fn from(items: Vec<Expr<T>>) -> Self {
        items.into_expr()
    }
}

impl<T: Is<T>, const N: usize> From<[Expr<T>; N]> for Expr<Array<T>> {
    fn from(items: [Expr<T>; N]) -> Self {
        items.into_expr()
    }
}

#[cfg(test)]
mod tests {
    use ow_workshop_data::{Hero, Team};

    use super::*;
    use crate::ir::StrToken;
    use crate::ty::Player;

    #[test]
    fn bool_and_string_literals() {
        let b: Expr<Bool> = true.into();
        assert_eq!(b.ir().kind(), &ExprKind::Bool(true));
        let s: Expr<Str> = "hi".into();
        assert_eq!(
            s.ir().kind(),
            &ExprKind::Str(StrExpr::Custom {
                tokens: vec![StrToken::Text("hi".to_owned())],
                args: vec![],
            })
        );
    }

    #[test]
    fn enum_literals_record_the_member() {
        let h: Expr<Hero> = Hero::Ana.into();
        assert_eq!(
            h.ir().kind(),
            &ExprKind::Const(ConstRef {
                enum_name: "Hero",
                key: Hero::Ana.key(),
            })
        );
        let t = Team::All.into_expr();
        assert!(matches!(t.ir().kind(), ExprKind::Const(c) if c.enum_name == "Team"));
    }

    #[test]
    fn arrays_convert_elementwise() {
        let a: Expr<Array<Num>> = [1, 2, 3].into_expr();
        let ExprKind::Array(items) = a.ir().kind() else {
            panic!("not an array");
        };
        assert_eq!(items.len(), 3);
        let b: Expr<Array<Num>> = vec![Expr::<Num>::from(1), 2.into()].into();
        assert!(matches!(b.ir().kind(), ExprKind::Array(v) if v.len() == 2));
    }

    #[test]
    fn null_vect_cast_and_erase() {
        let n: Expr<Player> = null();
        assert_eq!(n.ir().kind(), &ExprKind::Null);
        assert!(n.ir().loc().is_some());
        let v = vect(1, 2, 3);
        assert!(matches!(v.ir().kind(), ExprKind::Vector(_)));
        let any = v.clone().erase();
        let back: Expr<Vector> = any.cast();
        assert_eq!(back.ir(), v.ir());
    }
}
