//! Marker types for workshop value types and the subtyping between them.

use std::marker::PhantomData;

use ow_workshop_data as data;

use crate::expr::Expr;

mod sealed {
    pub trait Sealed {}
}

/// A workshop value type usable as the parameter of [`Expr`].
pub trait ValueType: sealed::Sealed {}

/// `Self` may be used where `S` is expected.
pub trait Is<S: ValueType>: ValueType {}

/// A generated constant enum used as a value; it is its own marker type.
pub trait ConstEnum: ValueType + Copy {
    /// Name of the enum in the data tables.
    const NAME: &'static str;
    /// Data key of the member.
    fn key(self) -> &'static str;
}

/// Anything that converts into an `Expr<T>`: expressions of a subtype, and Rust literals.
pub trait IntoExpr<T: ValueType> {
    /// Performs the conversion.
    fn into_expr(self) -> Expr<T>;
}

impl<S: Is<T>, T: ValueType> IntoExpr<T> for Expr<S> {
    fn into_expr(self) -> Expr<T> {
        Expr::from_ir(self.into_ir())
    }
}

impl<S: Is<T>, T: ValueType> IntoExpr<T> for &Expr<S> {
    fn into_expr(self) -> Expr<T> {
        Expr::from_ir(self.ir().clone())
    }
}

macro_rules! markers {
    ($($(#[$doc:meta])* $name:ident),* $(,)?) => {$(
        $(#[$doc])*
        #[derive(Clone, Copy, Debug)]
        pub enum $name {}
    )*};
}

markers! {
    /// A number.
    Num,
    /// A whole number; a subtype of [`Num`].
    Int,
    /// A boolean.
    Bool,
    /// A string.
    Str,
    /// A vector.
    Vector,
    /// A position vector.
    Position,
    /// A direction vector.
    Direction,
    /// A velocity vector.
    Velocity,
    /// A player.
    Player,
    /// A player or an array of players.
    Players,
    /// An entity id.
    EntityId,
    /// A text id.
    TextId,
    /// A damage modification id.
    DamageModId,
    /// A healing modification id.
    HealingModId,
    /// A damage-over-time id.
    DotId,
    /// A healing-over-time id.
    HotId,
    /// A health pool id.
    HealthPoolId,
    /// An assist id.
    AssistId,
    /// A hero or an array of heroes.
    Heroes,
    /// A player or a position.
    PlayerOrPosition,
    /// An entity id or a player.
    EntityOrPlayer,
    /// Any value; the escape hatch for workshop type tricks.
    Any,
}

/// An array of `T`.
#[derive(Clone, Copy, Debug)]
pub struct Array<T>(PhantomData<fn() -> T>, std::convert::Infallible);

impl<T: ValueType> sealed::Sealed for Array<T> {}
impl<T: ValueType> ValueType for Array<T> {}
impl<T: Is<U>, U: ValueType> Is<Array<U>> for Array<T> {}
impl<T: ValueType> Is<Any> for Array<T> {}
impl<T: Is<Player>> Is<Players> for Array<T> {}

macro_rules! plain_types {
    ($($t:ty),* $(,)?) => {$(
        impl sealed::Sealed for $t {}
        impl ValueType for $t {}
        impl Is<$t> for $t {}
        impl Is<Any> for $t {}
        impl From<Expr<$t>> for Expr<Any> {
            fn from(e: Expr<$t>) -> Self {
                Self::from_ir(e.into_ir())
            }
        }
    )*};
}

plain_types!(
    Num,
    Int,
    Bool,
    Str,
    Vector,
    Position,
    Direction,
    Velocity,
    Player,
    Players,
    Heroes,
    PlayerOrPosition,
    EntityOrPlayer,
    EntityId,
    TextId,
    DamageModId,
    HealingModId,
    DotId,
    HotId,
    HealthPoolId,
    AssistId,
);

impl sealed::Sealed for Any {}
impl ValueType for Any {}
impl Is<Any> for Any {}

impl<T: ValueType> From<Expr<Array<T>>> for Expr<Any> {
    fn from(e: Expr<Array<T>>) -> Self {
        Self::from_ir(e.into_ir())
    }
}

macro_rules! subtype {
    ($($sub:ident => $sup:ident),* $(,)?) => {$(
        impl Is<$sup> for $sub {}
        impl From<Expr<$sub>> for Expr<$sup> {
            fn from(e: Expr<$sub>) -> Self {
                Self::from_ir(e.into_ir())
            }
        }
    )*};
}

subtype! {
    Int => Num,
    Position => Vector,
    Direction => Vector,
    Velocity => Vector,
    Vector => Position,
    Vector => Direction,
    Vector => Velocity,
    Player => Players,
    Player => PlayerOrPosition,
    Position => PlayerOrPosition,
    Player => EntityOrPlayer,
    EntityId => EntityOrPlayer,
}

impl<T: Is<Hero>> Is<Heroes> for Array<T> {}
impl Is<Heroes> for Hero {}
impl<T: Is<Hero>> From<Expr<Array<T>>> for Expr<Heroes> {
    fn from(e: Expr<Array<T>>) -> Self {
        Self::from_ir(e.into_ir())
    }
}
impl From<Expr<Hero>> for Expr<Heroes> {
    fn from(e: Expr<Hero>) -> Self {
        Self::from_ir(e.into_ir())
    }
}
impl IntoExpr<Heroes> for Hero {
    fn into_expr(self) -> Expr<Heroes> {
        Expr::from(Expr::<Hero>::from(self))
    }
}

impl<T: Is<Player>> From<Expr<Array<T>>> for Expr<Players> {
    fn from(e: Expr<Array<T>>) -> Self {
        Self::from_ir(e.into_ir())
    }
}

macro_rules! const_enums {
    ($($name:ident),* $(,)?) => {$(
        pub use data::$name;
        impl sealed::Sealed for data::$name {}
        impl ValueType for data::$name {}
        impl Is<data::$name> for data::$name {}
        impl Is<Any> for data::$name {}
        impl ConstEnum for data::$name {
            const NAME: &'static str = stringify!($name);
            fn key(self) -> &'static str {
                data::$name::key(self)
            }
        }
        impl IntoExpr<data::$name> for data::$name {
            fn into_expr(self) -> Expr<data::$name> {
                Expr::constant(self)
            }
        }
        impl From<data::$name> for Expr<data::$name> {
            fn from(value: data::$name) -> Self {
                Expr::constant(value)
            }
        }
        impl From<Expr<data::$name>> for Expr<Any> {
            fn from(e: Expr<data::$name>) -> Self {
                Self::from_ir(e.into_ir())
            }
        }
    )*};
}

const_enums!(
    Hero,
    Ability,
    Map,
    Gamemode,
    AccelReeval,
    AssistReeval,
    BarrierLos,
    Beam,
    Button,
    ChaseRateReeval,
    ChaseTimeReeval,
    Clip,
    Color,
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
