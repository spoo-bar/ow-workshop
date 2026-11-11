//! `WorkshopDebug`: how variables are labelled and rendered on screen and in the debug log.
//!
//! `std::fmt::Debug` formats Rust values at compile time. This is the workshop-side counterpart:
//! it describes the expression that shows a *workshop* value in game. `#[derive(GlobalVars)]` and
//! `#[derive(PlayerVars)]` implement it for the variable structs.

use std::fmt;
use std::rc::Rc;

use crate::array_to_string::array_to_string;
use crate::expr::Expr;
use crate::ir::{self, ExprKind, StrExpr, StrToken, VarId};
use crate::ty::{Any, Array, Player, Str, ValueType};

/// Items of an array shown by the default renderer.
const ARRAY_ITEMS: u32 = 6;

/// Whose variable a [`DebugField`] shows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    /// A global variable.
    Global,
    /// A player variable; the renderer receives the player.
    Player,
}

/// A variable handle that can be shown: [`Global`](crate::Global) and
/// [`PlayerVar`](crate::PlayerVar).
pub trait DebugHandle: Copy + 'static {
    /// The value type of the variable.
    type Value: ValueType;
    /// Whether the variable is global or per player.
    const SCOPE: Scope;
    /// The variable.
    fn var_id(self) -> VarId;
    /// The variable's value; `player` is ignored for a global.
    fn read(self, player: Expr<Player>) -> Expr<Self::Value>;
}

/// How to show a value on screen: implemented by the value type markers, and by
/// `#[derive(WorkshopEnum)]` enums, which show the variant name.
///
/// `Value` is the workshop type the variable holds (`Num` for an enum); to use an enum's
/// renderer for a field, write `#[ow(debug_with = State::debug_render)]`.
pub trait DebugRender {
    /// The workshop type of the values rendered.
    type Value: ValueType;

    /// The text that shows `value`.
    fn debug_render(value: Expr<Self::Value>) -> Expr<Str>;
}

/// One displayed field.
#[derive(Clone)]
#[non_exhaustive]
pub struct DebugField {
    /// The label shown next to the value.
    pub label: &'static str,
    /// Whether the variable is global or per player.
    pub scope: Scope,
    /// The text showing the variable's value; a player variable reads from the player given.
    pub value: Rc<dyn Fn(Expr<Player>) -> Expr<Str>>,
    /// The variable, for the debug log.
    pub var: VarId,
}

impl DebugField {
    /// A field showing `handle` with the default renderer of its type.
    #[must_use]
    pub fn new<H: DebugHandle>(handle: H, label: &'static str) -> Self
    where
        H::Value: DebugRender<Value = H::Value>,
    {
        Self::with(handle, label, <H::Value as DebugRender>::debug_render)
    }

    /// A field showing `handle` through `render`.
    #[must_use]
    pub fn with<H: DebugHandle>(
        handle: H,
        label: &'static str,
        render: fn(Expr<H::Value>) -> Expr<Str>,
    ) -> Self {
        Self {
            label,
            scope: H::SCOPE,
            value: Rc::new(move |player| render(handle.read(player))),
            var: handle.var_id(),
        }
    }
}

impl fmt::Debug for DebugField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DebugField")
            .field("label", &self.label)
            .field("scope", &self.scope)
            .field("var", &self.var)
            .finish_non_exhaustive()
    }
}

/// Fields are equal when they show the same variable under the same label; the renderers are
/// closures and cannot be compared.
impl PartialEq for DebugField {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label && self.scope == other.scope && self.var == other.var
    }
}

/// Describes the fields to show for a value.
pub trait WorkshopDebug {
    /// One entry per displayed field.
    fn debug_fields(&self) -> Vec<DebugField>;
}

impl WorkshopDebug for () {
    fn debug_fields(&self) -> Vec<DebugField> {
        Vec::new()
    }
}

/// What a derived `debug_fields` calls for a field with the default renderer.
#[doc(hidden)]
#[must_use]
pub fn debug_field<H: DebugHandle>(handle: H, label: &'static str) -> DebugField
where
    H::Value: DebugRender<Value = H::Value>,
{
    DebugField::new(handle, label)
}

/// What a derived `debug_fields` calls for a field with `#[ow(debug_with = ..)]`.
#[doc(hidden)]
#[must_use]
pub fn debug_field_with<H: DebugHandle>(
    handle: H,
    label: &'static str,
    render: fn(Expr<H::Value>) -> Expr<Str>,
) -> DebugField {
    DebugField::with(handle, label, render)
}

/// `Custom String("<before>{0}<after>", value)`: the game stringifies the value.
#[track_caller]
fn wrapped(before: &str, value: ir::Expr, after: &str) -> Expr<Str> {
    let mut tokens = Vec::new();
    if !before.is_empty() {
        tokens.push(StrToken::Text(before.to_owned()));
    }
    tokens.push(StrToken::Arg(0));
    if !after.is_empty() {
        tokens.push(StrToken::Text(after.to_owned()));
    }
    Expr::from_ir(ir::Expr::here(ExprKind::Str(StrExpr::Custom {
        tokens,
        args: vec![value],
    })))
}

macro_rules! render_itself {
    ($($t:ty),* $(,)?) => {$(
        impl DebugRender for $t {
            type Value = $t;

            #[track_caller]
            fn debug_render(value: Expr<$t>) -> Expr<Str> {
                wrapped("", value.into_ir(), "")
            }
        }
    )*};
}

render_itself!(
    crate::ty::Num,
    crate::ty::Int,
    crate::ty::Bool,
    crate::ty::Vector,
    crate::ty::Position,
    crate::ty::Direction,
    crate::ty::Velocity,
    crate::ty::Player,
    crate::ty::Players,
    crate::ty::Heroes,
    crate::ty::PlayerOrPosition,
    crate::ty::EntityOrPlayer,
    crate::ty::EntityId,
    crate::ty::TextId,
    crate::ty::DamageModId,
    crate::ty::HealingModId,
    crate::ty::DotId,
    crate::ty::HotId,
    crate::ty::HealthPoolId,
    crate::ty::AssistId,
    Any,
);

/// What the generated constant enums (`Hero`, `Map`, `Team`, ...) show: the game prints their
/// name.
#[track_caller]
pub(crate) fn render_constant<E: ValueType>(value: Expr<E>) -> Expr<Str> {
    wrapped("", value.into_ir(), "")
}

/// A string is shown in quotes.
impl DebugRender for Str {
    type Value = Str;

    #[track_caller]
    fn debug_render(value: Expr<Str>) -> Expr<Str> {
        wrapped("\"", value.into_ir(), "\"")
    }
}

/// Arrays are spelled out; shown directly the workshop would print only the first element.
impl<T: ValueType> DebugRender for Array<T> {
    type Value = Array<T>;

    #[track_caller]
    fn debug_render(value: Expr<Array<T>>) -> Expr<Str> {
        array_to_string(value.erase().cast::<Array<Any>>(), ARRAY_ITEMS)
    }
}

#[cfg(test)]
mod tests;
