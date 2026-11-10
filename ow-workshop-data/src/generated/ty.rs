//! The type grammar of action and value arguments.
/// A type in a signature: a named type, an array, a union or a typed vector.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Ty {
    /// `AccelReeval`
    AccelReeval,
    /// `Array`
    Array,
    /// `AssistId`
    AssistId,
    /// `AssistReeval`
    AssistReeval,
    /// `BarrierLos`
    BarrierLos,
    /// `Beam`
    Beam,
    /// `BoolLiteral`
    BoolLiteral,
    /// `Button`
    Button,
    /// `ButtonLiteral`
    ButtonLiteral,
    /// `ChaseRateReeval`
    ChaseRateReeval,
    /// `ChaseTimeReeval`
    ChaseTimeReeval,
    /// `Clip`
    Clip,
    /// `Color`
    Color,
    /// `ColorLiteral`
    ColorLiteral,
    /// `Comms`
    Comms,
    /// `CompareOp`
    CompareOp,
    /// `CustomStringLiteral`
    CustomStringLiteral,
    /// `DamageModificationId`
    DamageModificationId,
    /// `DamageReeval`
    DamageReeval,
    /// `Dict`
    Dict,
    /// `DictElem`
    DictElem,
    /// `DictKey`
    DictKey,
    /// `Direction`
    Direction,
    /// `DotId`
    DotId,
    /// `DynamicEffect`
    DynamicEffect,
    /// `Effect`
    Effect,
    /// `EffectReeval`
    EffectReeval,
    /// `EntityId`
    EntityId,
    /// `FacingReeval`
    FacingReeval,
    /// `FloatLiteral`
    FloatLiteral,
    /// `Gamemode`
    Gamemode,
    /// `GamemodeLiteral`
    GamemodeLiteral,
    /// `GlobalVariable`
    GlobalVariable,
    /// `HealingModificationId`
    HealingModificationId,
    /// `HealingReeval`
    HealingReeval,
    /// `Health`
    Health,
    /// `HealthPoolId`
    HealthPoolId,
    /// `Hero`
    Hero,
    /// `HeroLiteral`
    HeroLiteral,
    /// `HeroStat`
    HeroStat,
    /// `HotId`
    HotId,
    /// `HudPosition`
    HudPosition,
    /// `HudReeval`
    HudReeval,
    /// `Icon`
    Icon,
    /// `IconReeval`
    IconReeval,
    /// `Impulse`
    Impulse,
    /// `IntLiteral`
    IntLiteral,
    /// `Invis`
    Invis,
    /// `Label`
    Label,
    /// `Lambda`
    Lambda,
    /// `LocalizedStringLiteral`
    LocalizedStringLiteral,
    /// `LosCheck`
    LosCheck,
    /// `Map`
    Map,
    /// `MapLiteral`
    MapLiteral,
    /// `ModifyHealth`
    ModifyHealth,
    /// `ModifyOp`
    ModifyOp,
    /// `Object`
    Object,
    /// `OutlineVisibility`
    OutlineVisibility,
    /// `Player`
    Player,
    /// `PlayerVariable`
    PlayerVariable,
    /// `Position`
    Position,
    /// `ProgressHudReeval`
    ProgressHudReeval,
    /// `ProgressWorldTextReeval`
    ProgressWorldTextReeval,
    /// `Projectile`
    Projectile,
    /// `ProjectileEffectReeval`
    ProjectileEffectReeval,
    /// `Raycast`
    Raycast,
    /// `Relativity`
    Relativity,
    /// `Rounding`
    Rounding,
    /// `SignedFloatLiteral`
    SignedFloatLiteral,
    /// `SignedIntLiteral`
    SignedIntLiteral,
    /// `SpecVisibility`
    SpecVisibility,
    /// `StartRuleBehavior`
    StartRuleBehavior,
    /// `Stat`
    Stat,
    /// `Status`
    Status,
    /// `String`
    String,
    /// `StringLiteral`
    StringLiteral,
    /// `Subroutine`
    Subroutine,
    /// `Team`
    Team,
    /// `TeamLiteral`
    TeamLiteral,
    /// `TextId`
    TextId,
    /// `Throttle`
    Throttle,
    /// `ThrottleReeval`
    ThrottleReeval,
    /// `Transform`
    Transform,
    /// `Type`
    Type,
    /// `UnsignedFloatLiteral`
    UnsignedFloatLiteral,
    /// `UnsignedIntLiteral`
    UnsignedIntLiteral,
    /// `Value`
    Value,
    /// `Vector`
    Vector,
    /// `Velocity`
    Velocity,
    /// `Wait`
    Wait,
    /// `WorldTextReeval`
    WorldTextReeval,
    /// `bool`
    Bool,
    /// `float`
    Float,
    /// `int`
    Int,
    /// `signed float`
    SignedFloat,
    /// `signed int`
    SignedInt,
    /// `unsigned float`
    UnsignedFloat,
    /// `unsigned int`
    UnsignedInt,
    /// `void`
    Void,
    /// An array with a known element type.
    ArrayOf(&'static Ty),
    /// Any of the listed types.
    Union(&'static [Ty]),
    /// A vector with per-component types.
    Vec3(VectorKind, &'static [Ty; 3]),
}
/// Which vector-like type a `Ty::Vec3` stands for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VectorKind {
    /// `Vector`
    Vector,
    /// `Direction`
    Direction,
    /// `Position`
    Position,
    /// `Velocity`
    Velocity,
}
impl Ty {
    /// The types this type is a direct subtype of.
    pub fn parents(self) -> &'static [Ty] {
        match self {
            Self::ButtonLiteral => &[Self::Button],
            Self::ColorLiteral => &[Self::Color],
            Self::Vector => &[Self::Direction, Self::Position, Self::Velocity],
            Self::SignedFloatLiteral | Self::UnsignedFloatLiteral => {
                &[Self::FloatLiteral]
            }
            Self::IntLiteral => &[Self::FloatLiteral, Self::Int],
            Self::GamemodeLiteral => &[Self::Gamemode],
            Self::HeroLiteral => &[Self::Hero],
            Self::SignedIntLiteral => &[Self::IntLiteral, Self::SignedFloatLiteral],
            Self::UnsignedIntLiteral => &[Self::IntLiteral, Self::UnsignedFloatLiteral],
            Self::MapLiteral => &[Self::Map],
            Self::AssistId
            | Self::Button
            | Self::Color
            | Self::DamageModificationId
            | Self::Direction
            | Self::DotId
            | Self::EntityId
            | Self::Gamemode
            | Self::HealingModificationId
            | Self::HealthPoolId
            | Self::Hero
            | Self::HotId
            | Self::Map
            | Self::Player
            | Self::Position
            | Self::String
            | Self::Team
            | Self::TextId
            | Self::Velocity
            | Self::Bool
            | Self::Float => &[Self::Object],
            Self::StringLiteral => &[Self::String],
            Self::CustomStringLiteral | Self::LocalizedStringLiteral => {
                &[Self::StringLiteral]
            }
            Self::TeamLiteral => &[Self::Team],
            Self::BoolLiteral => &[Self::Bool],
            Self::FloatLiteral | Self::Int | Self::SignedFloat | Self::UnsignedFloat => {
                &[Self::Float]
            }
            Self::SignedInt => &[Self::Int, Self::SignedFloat],
            Self::UnsignedInt => &[Self::Int, Self::UnsignedFloat],
            _ => &[],
        }
    }
}
