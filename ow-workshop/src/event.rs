//! Event types and the values each event provides.

use std::marker::PhantomData;

use ow_workshop_data::{Button, ValueId};

use crate::expr::Expr;
use crate::ir::{self, ExprKind};
use crate::ty::{Bool, Direction, Num, Player};

mod sealed {
    pub trait Sealed {}
}

/// An untyped event.
#[derive(Clone, Copy, Debug)]
pub struct RawEvent(pub(crate) ir::Event);

/// The event a rule listens to; one zero-sized type per event.
pub trait EventType: Copy + sealed::Sealed {
    /// The untyped event.
    fn raw(self) -> RawEvent;
}

/// Events of rules that may filter by team, slot or hero.
pub trait PlayerEvent: EventType {}
/// Events that provide `Event Player`.
pub trait HasEventPlayer: EventType {}
/// Events that provide the damage values.
pub trait HasDamage: EventType {}
/// Events that provide the healing values.
pub trait HasHealing: EventType {}
/// Events that provide the values shared by damage and healing.
pub trait HasDamageOrHealing: EventType {}

macro_rules! events {
    ($($(#[$doc:meta])* $name:ident => $variant:ident: [$($cap:ident),*];)*) => {$(
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct $name;
        impl sealed::Sealed for $name {}
        impl EventType for $name {
            fn raw(self) -> RawEvent {
                RawEvent(ir::Event::$variant)
            }
        }
        $( impl $cap for $name {} )*
    )*};
}

events! {
    /// `Ongoing - Global`.
    OngoingGlobal => OngoingGlobal: [];
    /// `Ongoing - Each Player`.
    OngoingEachPlayer => OngoingEachPlayer: [PlayerEvent, HasEventPlayer];
    /// `Player Joined Match`.
    PlayerJoinedMatch => PlayerJoinedMatch: [PlayerEvent, HasEventPlayer];
    /// `Player Left Match`.
    PlayerLeftMatch => PlayerLeftMatch: [PlayerEvent, HasEventPlayer];
    /// `Player Dealt Damage`.
    PlayerDealtDamage => PlayerDealtDamage: [PlayerEvent, HasEventPlayer, HasDamage, HasDamageOrHealing];
    /// `Player Took Damage`.
    PlayerTookDamage => PlayerTookDamage: [PlayerEvent, HasEventPlayer, HasDamage, HasDamageOrHealing];
    /// `Player Dealt Final Blow`.
    PlayerDealtFinalBlow => PlayerDealtFinalBlow: [PlayerEvent, HasEventPlayer, HasDamage, HasDamageOrHealing];
    /// `Player Died`.
    PlayerDied => PlayerDied: [PlayerEvent, HasEventPlayer, HasDamage, HasDamageOrHealing];
    /// `Player Earned Elimination`.
    PlayerEarnedElimination => PlayerEarnedElimination: [PlayerEvent, HasEventPlayer, HasDamage, HasDamageOrHealing];
    /// `Player Dealt Healing`.
    PlayerDealtHealing => PlayerDealtHealing: [PlayerEvent, HasEventPlayer, HasHealing, HasDamageOrHealing];
    /// `Player Received Healing`.
    PlayerReceivedHealing => PlayerReceivedHealing: [PlayerEvent, HasEventPlayer, HasHealing, HasDamageOrHealing];
    /// `Player Dealt Knockback`.
    PlayerDealtKnockback => PlayerDealtKnockback: [PlayerEvent, HasEventPlayer, HasDamage, HasHealing, HasDamageOrHealing];
    /// `Player Received Knockback`.
    PlayerReceivedKnockback => PlayerReceivedKnockback: [PlayerEvent, HasEventPlayer, HasDamage, HasHealing, HasDamageOrHealing];
}

/// The event of a subroutine body: it sees the caller's event values, which are checked at the
/// call sites instead.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SubroutineEvent;
impl sealed::Sealed for SubroutineEvent {}
impl EventType for SubroutineEvent {
    fn raw(self) -> RawEvent {
        // The subroutine's id is only known to `Builder::define`, which sets the real event.
        RawEvent(ir::Event::Subroutine(ir::SubId(u32::MAX)))
    }
}
impl HasEventPlayer for SubroutineEvent {}
impl HasDamage for SubroutineEvent {}
impl HasHealing for SubroutineEvent {}
impl HasDamageOrHealing for SubroutineEvent {}

/// Proof that a rule has event `E`; its methods are the values that event provides.
pub struct Ev<E> {
    _event: PhantomData<fn() -> E>,
}

impl<E> Clone for Ev<E> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<E> Copy for Ev<E> {}

impl<E> std::fmt::Debug for Ev<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Ev")
    }
}

impl<E> Ev<E> {
    /// The marker for a rule of event `E`.
    pub(crate) fn new() -> Self {
        Self {
            _event: PhantomData,
        }
    }
}

#[track_caller]
fn event_value<T>(id: ValueId) -> Expr<T> {
    Expr::from_ir(ir::Expr::here(ExprKind::Value {
        id,
        args: Vec::new(),
    }))
}

impl<E: HasEventPlayer> Ev<E> {
    /// `Event Player`.
    #[track_caller]
    #[must_use]
    pub fn event_player(self) -> Expr<Player> {
        event_value(ValueId::EventPlayer)
    }
}

impl<E: HasDamage> Ev<E> {
    /// `Attacker`.
    #[track_caller]
    #[must_use]
    pub fn attacker(self) -> Expr<Player> {
        event_value(ValueId::Attacker)
    }

    /// `Victim`.
    #[track_caller]
    #[must_use]
    pub fn victim(self) -> Expr<Player> {
        event_value(ValueId::Victim)
    }

    /// `Event Damage`.
    #[track_caller]
    #[must_use]
    pub fn event_damage(self) -> Expr<Num> {
        event_value(ValueId::EventDamage)
    }

    /// `Event Was Environment`.
    #[track_caller]
    #[must_use]
    pub fn was_environment(self) -> Expr<Bool> {
        event_value(ValueId::EventWasEnvironment)
    }
}

impl<E: HasHealing> Ev<E> {
    /// `Healer`.
    #[track_caller]
    #[must_use]
    pub fn healer(self) -> Expr<Player> {
        event_value(ValueId::Healer)
    }

    /// `Healee`.
    #[track_caller]
    #[must_use]
    pub fn healee(self) -> Expr<Player> {
        event_value(ValueId::Healee)
    }

    /// `Event Healing`.
    #[track_caller]
    #[must_use]
    pub fn event_healing(self) -> Expr<Num> {
        event_value(ValueId::EventHealing)
    }

    /// `Event Was Health Pack`.
    #[track_caller]
    #[must_use]
    pub fn was_health_pack(self) -> Expr<Bool> {
        event_value(ValueId::EventWasHealthPack)
    }
}

impl<E: HasDamageOrHealing> Ev<E> {
    /// `Event Was Critical Hit`.
    #[track_caller]
    #[must_use]
    pub fn was_critical_hit(self) -> Expr<Bool> {
        event_value(ValueId::EventWasCriticalHit)
    }

    /// `Event Ability`.
    #[track_caller]
    #[must_use]
    pub fn event_ability(self) -> Expr<Button> {
        event_value(ValueId::EventAbility)
    }

    /// `Event Direction`.
    #[track_caller]
    #[must_use]
    pub fn event_direction(self) -> Expr<Direction> {
        event_value(ValueId::EventDirection)
    }
}
