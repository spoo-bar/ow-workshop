//! Names of generated enum members.

use ow_workshop_data as data;
use ow_workshop_data::Lang;

macro_rules! lookup {
    ($($name:ident),* $(,)?) => {
        /// The localized name of the member `key` of the generated enum `enum_name`.
        pub(super) fn constant_name(enum_name: &str, key: &str, lang: Lang) -> Option<&'static str> {
            match enum_name {
                $(stringify!($name) => data::$name::ALL
                    .iter()
                    .find(|member| member.key() == key)
                    .map(|member| member.name(lang)),)*
                _ => None,
            }
        }
    };
}

lookup!(
    Hero,
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
