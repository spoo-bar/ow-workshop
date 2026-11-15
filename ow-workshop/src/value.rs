use crate::{CompareOp, Condition, Global, PlayerVar, ValueId};

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
