//! Action and value signatures and the type-compatibility rules.

use crate::{Ty, ValueId, VectorKind};

/// Whether `child` may be passed where `parent` is expected.
pub fn is_subtype(child: &Ty, parent: &Ty) -> bool {
    if child == parent {
        return true;
    }
    match (child, parent) {
        // Every member of a union child must fit some member of the parent.
        (Ty::Union(members), _) => members.iter().all(|m| is_subtype(m, parent)),
        (_, Ty::Union(members)) => members.iter().any(|m| is_subtype(child, m)),
        (Ty::ArrayOf(a), Ty::ArrayOf(b)) => is_subtype(a, b),
        // An array only fits the bare `Array`.
        (Ty::ArrayOf(_), _) => *parent == Ty::Array,
        (Ty::Vec3(ck, cc), Ty::Vec3(pk, pc)) => {
            is_subtype(&ck.ty(), &pk.ty())
                && cc.iter().zip(pc.iter()).all(|(c, p)| is_subtype(c, p))
        }
        // A vector with typed components fits `Vector` if every component is a number.
        (Ty::Vec3(_, components), _) => {
            components.iter().all(|c| is_subtype(c, &Ty::Float)) && is_subtype(&Ty::Vector, parent)
        }
        (_, Ty::ArrayOf(_) | Ty::Vec3(..)) => false,
        _ => child.parents().iter().any(|p| is_subtype(p, parent)),
    }
}

impl VectorKind {
    /// The plain type of this vector kind.
    pub fn ty(self) -> Ty {
        match self {
            Self::Vector => Ty::Vector,
            Self::Direction => Ty::Direction,
            Self::Position => Ty::Position,
            Self::Velocity => Ty::Velocity,
        }
    }
}

/// Default value of an argument.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DefaultValue {
    /// A number.
    Num(f64),
    /// A boolean.
    Bool(bool),
    /// `Null`.
    Null,
    /// A constant as (enum name, member key).
    Const(&'static str, &'static str),
    /// A value call with default arguments, e.g. all players.
    Value(ValueId, &'static [DefaultValue]),
    /// A vector literal.
    Vector([f64; 3]),
}

/// Literal swaps allowed in optimize-for-size mode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReplaceFlags(u8);

impl ReplaceFlags {
    /// No replacements.
    pub const NONE: Self = Self(0);

    /// Builds flags from bits: 1 `zero_by_false`, 2 `zero_by_null`, 4 `one_by_true`,
    /// 8 `null_vector_by_null`, 16 `empty_string_by_empty_array`.
    pub const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    /// `0` may become `False`.
    pub const fn zero_by_false(self) -> bool {
        self.0 & 1 != 0
    }

    /// `0` may become `Null`.
    pub const fn zero_by_null(self) -> bool {
        self.0 & 2 != 0
    }

    /// `1` may become `True`.
    pub const fn one_by_true(self) -> bool {
        self.0 & 4 != 0
    }

    /// `Vector(0, 0, 0)` may become `Null`.
    pub const fn null_vector_by_null(self) -> bool {
        self.0 & 8 != 0
    }

    /// An empty custom string may become `Empty Array`.
    pub const fn empty_string_by_empty_array(self) -> bool {
        self.0 & 16 != 0
    }
}

/// One argument of an action or value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Arg {
    /// The argument name (camelCase).
    pub name: &'static str,
    /// The accepted type.
    pub ty: Ty,
    /// The value used when the argument is omitted.
    pub default: Option<DefaultValue>,
    /// Smallest accepted number.
    pub min: Option<f64>,
    /// Largest accepted number.
    pub max: Option<f64>,
    /// The editor rejects number literals above this.
    pub literal_max: Option<f64>,
    /// Literal swaps allowed in this argument.
    pub replace: ReplaceFlags,
}

impl Arg {
    /// A required argument with no limits.
    pub const fn new(name: &'static str, ty: Ty) -> Self {
        Self {
            name,
            ty,
            default: None,
            min: None,
            max: None,
            literal_max: None,
            replace: ReplaceFlags::NONE,
        }
    }

    /// Sets the default value.
    #[must_use]
    pub const fn default(mut self, default: DefaultValue) -> Self {
        self.default = Some(default);
        self
    }

    /// Sets the accepted number range.
    #[must_use]
    pub const fn range(mut self, min: f64, max: f64) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }

    /// Sets the largest allowed number literal.
    #[must_use]
    pub const fn literal_max(mut self, max: f64) -> Self {
        self.literal_max = Some(max);
        self
    }

    /// Sets the allowed literal swaps.
    #[must_use]
    pub const fn replace(mut self, bits: u8) -> Self {
        self.replace = ReplaceFlags::from_bits(bits);
        self
    }
}

/// Properties of a whole signature.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SigFlags(u8);

impl SigFlags {
    /// Builds flags from bits: 1 `member`, 2 `internal`, 4 `has_literal_limit`,
    /// 8 `is_constant`, 16 `is_literal`, 32 `can_be_put_in_boolean`.
    pub const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    /// The first argument is the receiver.
    pub const fn member(self) -> bool {
        self.0 & 1 != 0
    }

    /// Compiler-only; users never call it.
    pub const fn internal(self) -> bool {
        self.0 & 2 != 0
    }

    /// Number literals are limited (see `Arg::literal_max`).
    pub const fn has_literal_limit(self) -> bool {
        self.0 & 4 != 0
    }

    /// Same value for the whole game.
    pub const fn is_constant(self) -> bool {
        self.0 & 8 != 0
    }

    /// A constant that is unique among constants under `==`.
    pub const fn is_literal(self) -> bool {
        self.0 & 16 != 0
    }

    /// Accepted where a boolean is expected.
    pub const fn can_be_put_in_boolean(self) -> bool {
        self.0 & 32 != 0
    }
}

/// The signature of an action or value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Signature {
    /// The arguments, in order.
    pub args: &'static [Arg],
    /// The result type (`Void` for actions).
    pub ret: Ty,
    /// Properties of the whole signature.
    pub flags: SigFlags,
}

impl Signature {
    /// Builds a signature.
    pub const fn new(args: &'static [Arg], ret: Ty, flags: SigFlags) -> Self {
        Self { args, ret, flags }
    }
}

impl ValueId {
    /// The value's signature.
    pub fn signature(self) -> &'static Signature {
        &crate::generated::values::SIGNATURES[self as usize]
    }
}
