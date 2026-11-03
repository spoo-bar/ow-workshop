//! Rules, events and optimizer overrides.

use std::collections::BTreeSet;

use ow_workshop_data::Hero;

use super::{Block, Expr, Location, SubId};

/// When a rule runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Event {
    /// `Ongoing - Global`.
    OngoingGlobal,
    /// `Ongoing - Each Player`.
    OngoingEachPlayer,
    /// `Player Earned Elimination`.
    PlayerEarnedElimination,
    /// `Player Dealt Final Blow`.
    PlayerDealtFinalBlow,
    /// `Player Dealt Damage`.
    PlayerDealtDamage,
    /// `Player Took Damage`.
    PlayerTookDamage,
    /// `Player Died`.
    PlayerDied,
    /// `Player Dealt Healing`.
    PlayerDealtHealing,
    /// `Player Received Healing`.
    PlayerReceivedHealing,
    /// `Player Joined Match`.
    PlayerJoinedMatch,
    /// `Player Left Match`.
    PlayerLeftMatch,
    /// `Player Dealt Knockback`.
    PlayerDealtKnockback,
    /// `Player Received Knockback`.
    PlayerReceivedKnockback,
    /// `Subroutine`: the body of the given subroutine.
    Subroutine(SubId),
}

impl Event {
    /// Whether rules of this event may carry a team and player filter.
    pub(crate) fn takes_filter(self) -> bool {
        !matches!(self, Self::OngoingGlobal | Self::Subroutine(_))
    }
}

/// Which team a player-event rule applies to.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TeamFilter {
    /// Both teams.
    #[default]
    All,
    /// Team 1 only.
    Team1,
    /// Team 2 only.
    Team2,
}

/// Which player a player-event rule applies to.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) enum PlayerSlot {
    /// Every player.
    #[default]
    All,
    /// A slot, 0–11.
    Slot(u8),
    /// Players playing a hero.
    Hero(Hero),
}

/// Team and player filter of a player-event rule.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) struct PlayerFilter {
    /// Team filter.
    pub(crate) team: TeamFilter,
    /// Player filter.
    pub(crate) player: PlayerSlot,
}

/// Highest valid player slot.
pub(crate) const MAX_SLOT: u8 = 11;

/// Optimizer settings that override the compile options for one block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct OptOverride {
    /// Whether the optimizer runs.
    pub(crate) enabled: bool,
    /// Whether it optimizes for size.
    pub(crate) size: bool,
    /// Strict-mode override; `None` inherits.
    pub(crate) strict: Option<bool>,
}

/// Names of warnings suppressed for a rule.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WarningSet(pub BTreeSet<String>);

/// One condition of a rule.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Cond {
    /// The condition.
    pub(crate) expr: Expr,
    /// Printed as `"text"` above the condition.
    pub(crate) comment: Option<String>,
}

/// A workshop rule.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Rule {
    /// Rule name, with any prefix already applied.
    pub(crate) name: String,
    /// When it runs.
    pub(crate) event: Event,
    /// Team and player filter; player events only.
    pub(crate) filter: Option<PlayerFilter>,
    /// Conditions, all of which must hold; none for subroutines.
    pub(crate) conditions: Vec<Cond>,
    /// The actions.
    pub(crate) body: Block,
    /// Emitted as disabled.
    pub(crate) disabled: bool,
    /// Kept as a separator even if empty after optimization.
    pub(crate) delimiter: bool,
    /// Name of the filler rules that push this rule onto a new editor page; `None` for no break.
    pub(crate) new_page: Option<String>,
    /// Warnings suppressed for this rule.
    pub(crate) suppressed: WarningSet,
    /// A free-form note on the rule; the emitter decides how to print it.
    pub(crate) comment: Option<String>,
    /// Where it was declared.
    pub(crate) location: Option<Location>,
}

/// Why a rule is malformed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RuleError {
    /// A filter on a global or subroutine rule.
    FilterNotAllowed,
    /// Conditions on a subroutine rule.
    SubroutineHasConditions,
    /// A player slot above [`MAX_SLOT`].
    SlotOutOfRange(u8),
}

impl Rule {
    /// A rule located at the caller, with the given event and no conditions, filter or flags.
    #[track_caller]
    pub(crate) fn new(name: impl Into<String>, event: Event, body: Block) -> Self {
        Self {
            name: name.into(),
            event,
            filter: None,
            conditions: Vec::new(),
            body,
            disabled: false,
            delimiter: false,
            new_page: None,
            suppressed: WarningSet::default(),
            comment: None,
            location: Some(Location::caller()),
        }
    }

    /// Checks the combinations the type system allows but the workshop does not.
    pub(crate) fn validate(&self) -> Result<(), RuleError> {
        if let Some(filter) = self.filter {
            if !self.event.takes_filter() {
                return Err(RuleError::FilterNotAllowed);
            }
            if let PlayerSlot::Slot(slot) = filter.player {
                if slot > MAX_SLOT {
                    return Err(RuleError::SlotOutOfRange(slot));
                }
            }
        }
        if matches!(self.event, Event::Subroutine(_)) && !self.conditions.is_empty() {
            return Err(RuleError::SubroutineHasConditions);
        }
        Ok(())
    }
}
