//! Operators, comparisons and conditional values.
//!
//! The workshop has no operators: `a + b` is recorded as `Add(a, b)`.

use std::ops::{Add, BitAnd, BitOr, Div, Mul, Neg, Not, Rem, Sub};

use ow_workshop_data::ValueId;

use crate::expr::Expr;
use crate::ir::{self, CmpOp, ExprKind};
use crate::ty::{
    Any, Bool, Direction, Int, IntoExpr, Is, Num, Position, ValueType, Vector, Velocity,
};

/// A number or an expression: the right-hand side of an arithmetic operator.
pub trait Operand {
    /// The workshop type of the operand.
    type Ty: ValueType;
    /// The operand as a typed expression.
    fn operand(self) -> Expr<Self::Ty>;
}

impl<T: ValueType> Operand for Expr<T> {
    type Ty = T;
    fn operand(self) -> Expr<T> {
        self
    }
}

impl<T: ValueType> Operand for &Expr<T> {
    type Ty = T;
    fn operand(self) -> Expr<T> {
        self.clone()
    }
}

macro_rules! number_operands {
    ($($t:ty),*) => {$(
        impl Operand for $t {
            type Ty = Num;
            fn operand(self) -> Expr<Num> {
                self.into()
            }
        }
    )*};
}

number_operands!(f64, i32, u32);

/// Types that behave as numbers in arithmetic.
pub trait NumLike: ValueType {}
/// Types that behave as vectors in arithmetic.
pub trait VecLike: ValueType {}

impl NumLike for Num {}
impl NumLike for Int {}
impl VecLike for Vector {}
impl VecLike for Position {}
impl VecLike for Direction {}
impl VecLike for Velocity {}

/// Types ordered by `<`, `<=`, `>`, `>=`.
pub trait Ordered: ValueType {
    /// What it is compared with.
    type Rhs: ValueType;
}

impl Ordered for Num {
    type Rhs = Num;
}
impl Ordered for Int {
    type Rhs = Num;
}
impl Ordered for Any {
    type Rhs = Any;
}

/// Operator markers.
#[derive(Clone, Copy, Debug)]
pub enum AddOp {}
/// Operator markers.
#[derive(Clone, Copy, Debug)]
pub enum SubOp {}
/// Operator markers.
#[derive(Clone, Copy, Debug)]
pub enum MulOp {}
/// Operator markers.
#[derive(Clone, Copy, Debug)]
pub enum DivOp {}
/// Operator markers.
#[derive(Clone, Copy, Debug)]
pub enum RemOp {}

/// `Self <Op> R` is allowed and has type `Out`.
pub trait Apply<Op, R: ValueType>: ValueType {
    /// The result type.
    type Out: ValueType;
}

macro_rules! cross {
    ($op:ident, $out:ident, [$($l:ident),*], $rs:tt) => {
        $( cross!(@row $op, $out, $l, $rs); )*
    };
    (@row $op:ident, $out:ident, $l:ident, [$($r:ident),*]) => {
        $( impl Apply<$op, $r> for $l { type Out = $out; } )*
    };
}

macro_rules! arithmetic_rules {
    ($op:ident: vector_by_vector = $vv:tt, number_by_vector = $nv:tt) => {
        cross!($op, Num, [Num, Int], [Num, Int]);
        cross!($op, Vector, [Vector, Position, Direction, Velocity], [Num, Int]);
        arithmetic_rules!(@vv $op, $vv);
        arithmetic_rules!(@nv $op, $nv);
    };
    (@vv $op:ident, true) => {
        cross!($op, Vector, [Vector, Position, Direction, Velocity], [Vector, Position, Direction, Velocity]);
    };
    (@vv $op:ident, false) => {};
    (@nv $op:ident, true) => {
        cross!($op, Vector, [Num, Int], [Vector, Position, Direction, Velocity]);
    };
    (@nv $op:ident, false) => {};
}

arithmetic_rules!(AddOp: vector_by_vector = true, number_by_vector = false);
arithmetic_rules!(SubOp: vector_by_vector = true, number_by_vector = false);
arithmetic_rules!(MulOp: vector_by_vector = true, number_by_vector = true);
arithmetic_rules!(DivOp: vector_by_vector = true, number_by_vector = false);
arithmetic_rules!(RemOp: vector_by_vector = false, number_by_vector = false);

macro_rules! binary_operator {
    ($Trait:ident, $method:ident, $Op:ident, $kind:ident) => {
        impl<L: ValueType, R: Operand> $Trait<R> for Expr<L>
        where
            L: Apply<$Op, R::Ty>,
        {
            type Output = Expr<<L as Apply<$Op, R::Ty>>::Out>;

            #[track_caller]
            fn $method(self, rhs: R) -> Self::Output {
                let kind = ExprKind::$kind(self.into_ir(), rhs.operand().into_ir());
                Expr::from_ir(ir::Expr::here(kind))
            }
        }

        impl<L: ValueType, R: Operand> $Trait<R> for &Expr<L>
        where
            L: Apply<$Op, R::Ty>,
        {
            type Output = Expr<<L as Apply<$Op, R::Ty>>::Out>;

            #[track_caller]
            fn $method(self, rhs: R) -> Self::Output {
                let kind = ExprKind::$kind(self.ir().clone(), rhs.operand().into_ir());
                Expr::from_ir(ir::Expr::here(kind))
            }
        }
    };
}

binary_operator!(Add, add, AddOp, Add);
binary_operator!(Sub, sub, SubOp, Sub);
binary_operator!(Mul, mul, MulOp, Mul);
binary_operator!(Div, div, DivOp, Div);
binary_operator!(Rem, rem, RemOp, Mod);

/// Types that can be negated.
pub trait Negate: ValueType {
    /// The result type.
    type Out: ValueType;
}

macro_rules! negate {
    ($($t:ident => $o:ident),*) => {$( impl Negate for $t { type Out = $o; } )*};
}

negate!(Num => Num, Int => Num, Vector => Vector, Position => Vector, Direction => Vector, Velocity => Vector);

fn negate_ir(e: ir::Expr) -> ir::Expr {
    // A negated number literal is a negative literal; anything else lowers to `Multiply(-1, x)`.
    match e.kind() {
        ExprKind::Num(n) => ir::Expr::new(ExprKind::Num(-n), e.loc()),
        _ => ir::Expr::here(ExprKind::Neg(e)),
    }
}

impl<T: Negate> Neg for Expr<T> {
    type Output = Expr<T::Out>;

    #[track_caller]
    fn neg(self) -> Self::Output {
        Expr::from_ir(negate_ir(self.into_ir()))
    }
}

impl<T: Negate> Neg for &Expr<T> {
    type Output = Expr<T::Out>;

    #[track_caller]
    fn neg(self) -> Self::Output {
        Expr::from_ir(negate_ir(self.ir().clone()))
    }
}

impl<R: IntoExpr<Bool>> BitAnd<R> for Expr<Bool> {
    type Output = Expr<Bool>;

    #[track_caller]
    fn bitand(self, rhs: R) -> Expr<Bool> {
        Expr::from_ir(ir::Expr::here(ExprKind::And(
            self.into_ir(),
            rhs.into_expr().into_ir(),
        )))
    }
}

impl<R: IntoExpr<Bool>> BitOr<R> for Expr<Bool> {
    type Output = Expr<Bool>;

    #[track_caller]
    fn bitor(self, rhs: R) -> Expr<Bool> {
        Expr::from_ir(ir::Expr::here(ExprKind::Or(
            self.into_ir(),
            rhs.into_expr().into_ir(),
        )))
    }
}

impl Not for Expr<Bool> {
    type Output = Expr<Bool>;

    #[track_caller]
    fn not(self) -> Expr<Bool> {
        Expr::from_ir(ir::Expr::here(ExprKind::Not(self.into_ir())))
    }
}

impl Not for &Expr<Bool> {
    type Output = Expr<Bool>;

    #[track_caller]
    fn not(self) -> Expr<Bool> {
        Expr::from_ir(ir::Expr::here(ExprKind::Not(self.ir().clone())))
    }
}

impl<T: Is<Any>> Expr<T> {
    /// `And`; the workshop accepts any truthy value.
    #[track_caller]
    #[must_use]
    pub fn and(&self, other: impl IntoExpr<Any>) -> Expr<Bool> {
        let kind = ExprKind::And(self.ir().clone(), other.into_expr().into_ir());
        Expr::from_ir(ir::Expr::here(kind))
    }

    /// `Or`; the workshop accepts any truthy value.
    #[track_caller]
    #[must_use]
    pub fn or(&self, other: impl IntoExpr<Any>) -> Expr<Bool> {
        let kind = ExprKind::Or(self.ir().clone(), other.into_expr().into_ir());
        Expr::from_ir(ir::Expr::here(kind))
    }
}

impl<T: ValueType> Expr<T> {
    #[track_caller]
    fn compare(&self, op: CmpOp, other: ir::Expr) -> Expr<Bool> {
        let kind = ExprKind::Compare {
            op,
            lhs: self.ir().clone(),
            rhs: other,
        };
        Expr::from_ir(ir::Expr::here(kind))
    }

    /// `==`.
    #[track_caller]
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn eq(&self, other: impl IntoExpr<T>) -> Expr<Bool> {
        self.compare(CmpOp::Eq, other.into_expr().into_ir())
    }

    /// `!=`.
    #[track_caller]
    #[must_use]
    pub fn ne(&self, other: impl IntoExpr<T>) -> Expr<Bool> {
        self.compare(CmpOp::Ne, other.into_expr().into_ir())
    }
}

impl<T: Ordered> Expr<T> {
    /// `<`.
    #[track_caller]
    #[must_use]
    pub fn lt(&self, other: impl IntoExpr<T::Rhs>) -> Expr<Bool> {
        self.compare(CmpOp::Lt, other.into_expr().into_ir())
    }

    /// `<=`.
    #[track_caller]
    #[must_use]
    pub fn le(&self, other: impl IntoExpr<T::Rhs>) -> Expr<Bool> {
        self.compare(CmpOp::Le, other.into_expr().into_ir())
    }

    /// `>`.
    #[track_caller]
    #[must_use]
    pub fn gt(&self, other: impl IntoExpr<T::Rhs>) -> Expr<Bool> {
        self.compare(CmpOp::Gt, other.into_expr().into_ir())
    }

    /// `>=`.
    #[track_caller]
    #[must_use]
    pub fn ge(&self, other: impl IntoExpr<T::Rhs>) -> Expr<Bool> {
        self.compare(CmpOp::Ge, other.into_expr().into_ir())
    }
}

impl<T: NumLike> Expr<T> {
    /// `Raise To Power`.
    #[track_caller]
    #[must_use]
    pub fn pow(&self, exponent: impl IntoExpr<Num>) -> Expr<Num> {
        let kind = ExprKind::Pow(self.ir().clone(), exponent.into_expr().into_ir());
        Expr::from_ir(ir::Expr::here(kind))
    }
}

impl<T: VecLike> Expr<T> {
    #[track_caller]
    fn component(&self, id: ValueId) -> Expr<Num> {
        let kind = ExprKind::Value {
            id,
            args: vec![self.ir().clone()],
        };
        Expr::from_ir(ir::Expr::here(kind))
    }

    /// `X Component Of`.
    #[track_caller]
    #[must_use]
    pub fn x(&self) -> Expr<Num> {
        self.component(ValueId::XComponentOf)
    }

    /// `Y Component Of`.
    #[track_caller]
    #[must_use]
    pub fn y(&self) -> Expr<Num> {
        self.component(ValueId::YComponentOf)
    }

    /// `Z Component Of`.
    #[track_caller]
    #[must_use]
    pub fn z(&self) -> Expr<Num> {
        self.component(ValueId::ZComponentOf)
    }
}

/// `If-Then-Else`.
#[track_caller]
#[must_use]
pub fn if_then_else<T: ValueType>(
    cond: impl IntoExpr<Bool>,
    then: Expr<T>,
    else_: impl IntoExpr<T>,
) -> Expr<T> {
    let kind = ExprKind::IfThenElse {
        cond: cond.into_expr().into_ir(),
        then: then.into_ir(),
        else_: else_.into_expr().into_ir(),
    };
    Expr::from_ir(ir::Expr::here(kind))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::vect;

    fn n(v: i32) -> Expr<Num> {
        v.into()
    }

    fn num(v: f64) -> ir::Expr {
        ir::Expr::new(ExprKind::Num(v), None)
    }

    #[test]
    fn arithmetic_records_the_operator() {
        assert_eq!(
            (n(1) + n(2)).ir().kind(),
            &ExprKind::Add(num(1.0), num(2.0))
        );
        assert_eq!((n(1) - 2).ir().kind(), &ExprKind::Sub(num(1.0), num(2.0)));
        assert_eq!((n(3) * 2.5).ir().kind(), &ExprKind::Mul(num(3.0), num(2.5)));
        assert_eq!(
            (n(3) / n(2)).ir().kind(),
            &ExprKind::Div(num(3.0), num(2.0))
        );
        assert_eq!((n(3) % 2).ir().kind(), &ExprKind::Mod(num(3.0), num(2.0)));
        assert_eq!(n(3).pow(2).ir().kind(), &ExprKind::Pow(num(3.0), num(2.0)));
    }

    #[test]
    fn negating_a_literal_is_a_literal() {
        assert_eq!((-n(5)).ir().kind(), &ExprKind::Num(-5.0));
        assert!(matches!((-vect(1, 2, 3)).ir().kind(), ExprKind::Neg(_)));
        let x = n(1) + n(2);
        assert!(matches!((-x).ir().kind(), ExprKind::Neg(_)));
    }

    #[test]
    fn boolean_operators() {
        let t: Expr<Bool> = true.into();
        let f: Expr<Bool> = false.into();
        assert!(matches!(
            (t.clone() & f.clone()).ir().kind(),
            ExprKind::And(..)
        ));
        assert!(matches!((t.clone() | false).ir().kind(), ExprKind::Or(..)));
        assert!(matches!((!t.clone()).ir().kind(), ExprKind::Not(_)));
        assert!(matches!((!&t).ir().kind(), ExprKind::Not(_)));
        assert!(matches!(n(1).and(&t).ir().kind(), ExprKind::And(..)));
        assert!(matches!(n(0).or(n(1)).ir().kind(), ExprKind::Or(..)));
    }

    #[test]
    fn comparisons() {
        let cases = [
            (n(1).eq(2), CmpOp::Eq),
            (n(1).ne(2), CmpOp::Ne),
            (n(1).lt(2), CmpOp::Lt),
            (n(1).le(2), CmpOp::Le),
            (n(1).gt(2), CmpOp::Gt),
            (n(1).ge(2), CmpOp::Ge),
        ];
        for (e, expected) in cases {
            let ExprKind::Compare { op, lhs, rhs } = e.ir().kind() else {
                panic!("not a comparison");
            };
            assert_eq!(*op, expected);
            assert_eq!((lhs, rhs), (&num(1.0), &num(2.0)));
        }
        let b: Expr<Bool> = true.into();
        assert!(matches!(
            b.eq(false).ir().kind(),
            ExprKind::Compare { op: CmpOp::Eq, .. }
        ));
    }

    #[test]
    fn if_then_else_and_components() {
        let e = if_then_else(true, n(1), 2);
        assert!(matches!(e.ir().kind(), ExprKind::IfThenElse { .. }));
        let v = vect(1, 2, 3);
        for (c, id) in [
            (v.x(), ValueId::XComponentOf),
            (v.y(), ValueId::YComponentOf),
            (v.z(), ValueId::ZComponentOf),
        ] {
            assert!(matches!(c.ir().kind(), ExprKind::Value { id: got, .. } if *got == id));
        }
    }

    #[test]
    fn operators_record_the_call_site() {
        let e = n(1) + n(2);
        assert!(e.ir().loc().is_some_and(|l| l.file.ends_with("ops.rs")));
    }
}
