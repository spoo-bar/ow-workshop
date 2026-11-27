use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Not, Rem, Sub};

use crate::{CompareOp, Condition, Global, PlayerVar, ValueId, data};

/// An expression in a condition or an action argument.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// A number literal.
    Number(f64),
    /// `True` or `False`.
    Bool(bool),
    /// A workshop value, such as `Event Player` or `Has Spawned(...)`.
    Call {
        /// Which value.
        id: ValueId,
        /// Its arguments, in workshop order.
        args: Vec<Value>,
    },
    /// A named constant argument, such as `Left` or `Visible To and String`.
    Constant(&'static str),
    /// `Custom String("text", args...)` with up to three arguments.
    CustomString {
        /// The text, with `{0}`, `{1}`, `{2}` placeholders.
        text: String,
        /// The values substituted for the placeholders.
        args: Vec<Value>,
    },
    /// A read of a global variable.
    Global(Global),
    /// A read of a player variable.
    PlayerVar {
        /// Whose variable.
        player: Box<Value>,
        /// Which variable.
        var: PlayerVar,
    },
    /// `Compare(lhs, op, rhs)`.
    Compare {
        /// Left operand.
        lhs: Box<Value>,
        /// Operator.
        op: CompareOp,
        /// Right operand.
        rhs: Box<Value>,
    },
}

impl Value {
    /// A workshop value with arguments.
    pub fn call(id: ValueId, args: impl IntoIterator<Item = Value>) -> Self {
        Self::Call {
            id,
            args: args.into_iter().collect(),
        }
    }

    /// A workshop value without arguments.
    pub fn of(id: ValueId) -> Self {
        Self::call(id, [])
    }

    /// `Event Player`.
    pub fn event_player() -> Self {
        Self::of(ValueId::EventPlayer)
    }

    /// `Null`.
    pub fn null() -> Self {
        Self::of(ValueId::Null)
    }

    /// `Custom String("text", args...)`.
    pub fn string(text: impl Into<String>, args: impl IntoIterator<Item = Value>) -> Self {
        Self::CustomString {
            text: text.into(),
            args: args.into_iter().collect(),
        }
    }

    /// `Compare(self, op, rhs)`, for use inside other values.
    #[must_use]
    pub fn compare(self, op: CompareOp, rhs: impl Into<Value>) -> Self {
        Self::Compare {
            lhs: Box::new(self),
            op,
            rhs: Box::new(rhs.into()),
        }
    }

    /// `Compare(self, ==, rhs)`.
    #[must_use]
    pub fn equals(self, rhs: impl Into<Value>) -> Self {
        self.compare(CompareOp::Eq, rhs)
    }

    /// `Compare(self, !=, rhs)`.
    #[must_use]
    pub fn not_equals(self, rhs: impl Into<Value>) -> Self {
        self.compare(CompareOp::Ne, rhs)
    }

    /// `Compare(self, <, rhs)`.
    #[must_use]
    pub fn lt(self, rhs: impl Into<Value>) -> Self {
        self.compare(CompareOp::Lt, rhs)
    }

    /// `Compare(self, <=, rhs)`.
    #[must_use]
    pub fn le(self, rhs: impl Into<Value>) -> Self {
        self.compare(CompareOp::Le, rhs)
    }

    /// `Compare(self, >, rhs)`.
    #[must_use]
    pub fn gt(self, rhs: impl Into<Value>) -> Self {
        self.compare(CompareOp::Gt, rhs)
    }

    /// `Compare(self, >=, rhs)`.
    #[must_use]
    pub fn ge(self, rhs: impl Into<Value>) -> Self {
        self.compare(CompareOp::Ge, rhs)
    }

    /// The rule condition `self op rhs`.
    pub fn is(self, op: CompareOp, rhs: impl Into<Value>) -> Condition {
        Condition {
            lhs: self,
            op,
            rhs: rhs.into(),
        }
    }

    /// The rule condition `self == True`.
    pub fn is_true(self) -> Condition {
        self.is(CompareOp::Eq, true)
    }

    /// The rule condition `self == False`.
    pub fn is_false(self) -> Condition {
        self.is(CompareOp::Eq, false)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Number(value.into())
    }
}

impl From<&Global> for Value {
    fn from(var: &Global) -> Self {
        Self::Global(var.clone())
    }
}

impl fmt::Display for Value {
    /// The value as written in workshop code.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&crate::emit::value(self))
    }
}

/// `std::ops` impls that write the matching workshop value.
macro_rules! binary_ops {
    ($($op:ident $method:ident $id:ident),* $(,)?) => {$(
        impl<T: Into<Value>> $op<T> for Value {
            type Output = Value;

            /// Writes the workshop value of the same name.
            fn $method(self, rhs: T) -> Value {
                Value::call(ValueId::$id, [self, rhs.into()])
            }
        }
    )*};
}

binary_ops!(
    Add add Add,
    Sub sub Subtract,
    Mul mul Multiply,
    Div div Divide,
    Rem rem Modulo,
);

impl Neg for Value {
    type Output = Value;

    /// A negated number literal, otherwise `Multiply(-1, self)`.
    fn neg(self) -> Value {
        match self {
            Value::Number(n) => Value::Number(-n),
            other => Value::call(ValueId::Multiply, [Value::Number(-1.0), other]),
        }
    }
}

impl Not for Value {
    type Output = Value;

    /// `Not(self)`.
    fn not(self) -> Value {
        Value::call(ValueId::Not, [self])
    }
}

/// Constants written bare, such as `All Teams` or `To Nearest`.
macro_rules! bare_constants {
    ($($ty:ident),* $(,)?) => {$(
        impl From<data::$ty> for Value {
            fn from(value: data::$ty) -> Self {
                Self::Constant(value.name())
            }
        }
    )*};
}

bare_constants!(
    AccelReeval,
    AssistReeval,
    BarrierLos,
    Beam,
    ChaseRateReeval,
    ChaseTimeReeval,
    Clip,
    Comms,
    CompareOp,
    DamageReeval,
    DynamicEffect,
    Effect,
    EffectReeval,
    FacingReeval,
    HealingReeval,
    Health,
    HeroStat,
    HudPosition,
    HudReeval,
    Icon,
    IconReeval,
    Impulse,
    Invis,
    LosCheck,
    ModifyHealth,
    ModifyOp,
    OutlineVisibility,
    ProgressHudReeval,
    ProgressWorldTextReeval,
    Projectile,
    ProjectileEffectReeval,
    Relativity,
    Rounding,
    SpecVisibility,
    StartRuleBehavior,
    Stat,
    Status,
    Team,
    Throttle,
    ThrottleReeval,
    Transform,
    Wait,
    WorldTextReeval,
);

/// Constants that need their wrapper value, such as `Hero(Ana)` or `Color(White)`.
macro_rules! wrapped_constants {
    ($($ty:ident $id:ident),* $(,)?) => {$(
        impl From<data::$ty> for Value {
            fn from(value: data::$ty) -> Self {
                Self::call(ValueId::$id, [Self::Constant(value.name())])
            }
        }
    )*};
}

wrapped_constants!(
    Button Button,
    Color Color,
    Gamemode Gamemode,
    Hero Hero,
    Map Map,
);
