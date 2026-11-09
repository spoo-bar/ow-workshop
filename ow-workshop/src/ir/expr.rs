//! Untyped expressions.

use std::rc::Rc;

use ow_workshop_data::{LocalizedString, ValueId};

use super::{LabelId, Location, VarId};

/// A generated enum member, as (enum name, member key).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct ConstRef {
    /// Name of the generated enum, e.g. `Team`.
    pub(crate) enum_name: &'static str,
    /// Data key of the member, e.g. `ALL`.
    pub(crate) key: &'static str,
}

/// A comparison operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum CmpOp {
    /// `==`
    Eq,
    /// `!=`
    Ne,
    /// `<`
    Lt,
    /// `<=`
    Le,
    /// `>`
    Gt,
    /// `>=`
    Ge,
}

/// One piece of a custom string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum StrToken {
    /// Literal text.
    Text(String),
    /// Placeholder `{n}`, an index into the string's args.
    Arg(usize),
}

/// A string-valued expression.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum StrExpr {
    /// A custom string: tokens plus placeholder arguments.
    Custom {
        /// Text and placeholders, in order.
        tokens: Vec<StrToken>,
        /// Values substituted for the placeholders.
        args: Vec<Expr>,
    },
    /// A built-in localized string.
    Localized(LocalizedString),
}

/// A rewrite of the text of a string template.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StrModifier {
    /// Fullwidth lookalikes.
    Fullwidth,
    /// The large Blizzard Global font.
    Big,
    /// Lowercase letters that the game does not uppercase.
    CaseSensitive,
}

/// A string that depends on the viewer's language; removed by lowering.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Translated {
    /// Lookup key (the source-language text).
    pub(crate) key: String,
    /// Disambiguating context.
    pub(crate) context: Option<String>,
    /// Placeholder arguments.
    pub(crate) args: Vec<Expr>,
    /// Keep the array of all translations even where a display action would pick one.
    pub(crate) raw: bool,
    /// Pick the language without the player variable.
    pub(crate) spectator: bool,
}

/// The shape of an expression.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ExprKind {
    /// Number literal.
    Num(f64),
    /// Boolean literal.
    Bool(bool),
    /// `Null`.
    Null,
    /// A generated enum member.
    Const(ConstRef),
    /// A string.
    Str(StrExpr),
    /// `Vector(x, y, z)`.
    Vector([Expr; 3]),
    /// `Array(..)`.
    Array(Vec<Expr>),
    /// A call of a workshop value.
    Value {
        /// Which value.
        id: ValueId,
        /// Arguments in signature order.
        args: Vec<Expr>,
    },
    /// A global variable.
    GlobalVar(VarId),
    /// A player variable of `player`.
    PlayerVar {
        /// The owning player.
        player: Expr,
        /// The variable.
        var: VarId,
    },
    /// `Value In Array(array, index)`.
    Index {
        /// The array.
        array: Expr,
        /// The index.
        index: Expr,
    },
    /// `Current Array Element`.
    ArrayElem,
    /// `Current Array Index`.
    ArrayIndex,
    /// A comparison.
    Compare {
        /// The operator.
        op: CmpOp,
        /// Left operand.
        lhs: Expr,
        /// Right operand.
        rhs: Expr,
    },
    /// `And`.
    And(Expr, Expr),
    /// `Or`.
    Or(Expr, Expr),
    /// `Not`.
    Not(Expr),
    /// `If-Then-Else`.
    IfThenElse {
        /// Condition.
        cond: Expr,
        /// Value when true.
        then: Expr,
        /// Value when false.
        else_: Expr,
    },
    /// Unary minus.
    Neg(Expr),
    /// `Add`.
    Add(Expr, Expr),
    /// `Subtract`.
    Sub(Expr, Expr),
    /// `Multiply`.
    Mul(Expr, Expr),
    /// `Divide`.
    Div(Expr, Expr),
    /// `Modulo`.
    Mod(Expr, Expr),
    /// `Raise To Power`.
    Pow(Expr, Expr),
    /// Key-value lookup; removed by lowering.
    Dict {
        /// Key-value pairs.
        entries: Vec<(Expr, Expr)>,
        /// Value for missing keys.
        default: Option<Expr>,
    },
    /// A language-dependent string; removed by lowering.
    Translated(Translated),
    /// Picks the viewer's language out of a stored translated string; removed by lowering.
    Resolve {
        /// The stored array of translations.
        inner: Expr,
        /// Pick the language without the player variable.
        spectator: bool,
    },
    /// The raw `<` character kept in `__holygrail__`; replaced by that global when lowering.
    HolyGrail,
    /// A call of a localized (built-in phrase) string; removed by lowering.
    LocalizedCall {
        /// The phrase, or the English text that matched none.
        phrase: Result<LocalizedString, String>,
        /// Up to three placeholder arguments.
        args: Vec<Expr>,
    },
    /// A modifier applied to a string template; removed by lowering.
    StrModify {
        /// The modifier.
        modifier: StrModifier,
        /// The string it applies to.
        inner: Expr,
    },
    /// The enclosing rule's own condition; removed by lowering.
    RuleCondition,
    /// Distance from the enclosing `Skip` to a label; resolved to a number with the labels.
    LabelDistance(LabelId),
}

/// An expression node: cheap to clone, with its source location.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Expr {
    kind: Rc<ExprKind>,
    loc: Option<Location>,
}

impl Expr {
    /// Creates an expression.
    pub(crate) fn new(kind: ExprKind, loc: Option<Location>) -> Self {
        Self {
            kind: Rc::new(kind),
            loc,
        }
    }

    /// Creates an expression located at the caller.
    #[track_caller]
    pub(crate) fn here(kind: ExprKind) -> Self {
        Self::new(kind, Some(Location::caller()))
    }

    /// The expression's shape.
    pub(crate) fn kind(&self) -> &ExprKind {
        &self.kind
    }

    /// Where the user wrote it, if known.
    pub(crate) fn loc(&self) -> Option<Location> {
        self.loc
    }
}

impl ExprKind {
    /// The expressions directly nested inside this one.
    pub(crate) fn children(&self) -> Vec<&Expr> {
        match self {
            Self::Num(_)
            | Self::Bool(_)
            | Self::Null
            | Self::Const(_)
            | Self::Str(StrExpr::Localized(_))
            | Self::GlobalVar(_)
            | Self::ArrayElem
            | Self::ArrayIndex
            | Self::RuleCondition
            | Self::HolyGrail
            | Self::LabelDistance(_) => Vec::new(),
            Self::Vector(parts) => parts.iter().collect(),
            Self::Array(items) => items.iter().collect(),
            Self::Str(StrExpr::Custom { args, .. }) | Self::Value { args, .. } => {
                args.iter().collect()
            }
            Self::PlayerVar { player, .. } => vec![player],
            Self::Index { array, index } => vec![array, index],
            Self::Compare { lhs, rhs, .. } => vec![lhs, rhs],
            Self::And(a, b)
            | Self::Or(a, b)
            | Self::Add(a, b)
            | Self::Sub(a, b)
            | Self::Mul(a, b)
            | Self::Div(a, b)
            | Self::Mod(a, b)
            | Self::Pow(a, b) => vec![a, b],
            Self::Not(a)
            | Self::Neg(a)
            | Self::StrModify { inner: a, .. }
            | Self::Resolve { inner: a, .. } => vec![a],
            Self::IfThenElse { cond, then, else_ } => vec![cond, then, else_],
            Self::Dict { entries, default } => entries
                .iter()
                .flat_map(|(k, v)| [k, v])
                .chain(default)
                .collect(),
            Self::Translated(t) => t.args.iter().collect(),
            Self::LocalizedCall { args, .. } => args.iter().collect(),
        }
    }
}
