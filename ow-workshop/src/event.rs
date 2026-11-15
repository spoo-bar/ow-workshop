use ow_workshop_data::Hero;

/// What triggers a rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    /// `Ongoing - Global`.
    OngoingGlobal,
    /// An event that runs once per matching player.
    Player {
        /// Which player event.
        kind: PlayerEvent,
        /// Which team's players it applies to.
        team: TeamFilter,
        /// Which players it applies to.
        player: PlayerFilter,
    },
}

impl Event {
    /// `Ongoing - Each Player; All; All;`.
    pub fn each_player() -> Self {
        Self::player(PlayerEvent::OngoingEachPlayer)
    }

    /// A player event for every team and player.
    pub fn player(kind: PlayerEvent) -> Self {
        Self::Player {
            kind,
            team: TeamFilter::All,
            player: PlayerFilter::All,
        }
    }

    /// Restricts a player event to one hero. No effect on `Ongoing - Global`.
    #[must_use]
    pub fn hero(self, hero: Hero) -> Self {
        match self {
            Self::OngoingGlobal => self,
            Self::Player { kind, team, .. } => Self::Player {
                kind,
                team,
                player: PlayerFilter::Hero(hero),
            },
        }
    }
}

/// The player events.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerEvent {
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
}

impl PlayerEvent {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::OngoingEachPlayer => "Ongoing - Each Player",
            Self::PlayerEarnedElimination => "Player Earned Elimination",
            Self::PlayerDealtFinalBlow => "Player Dealt Final Blow",
            Self::PlayerDealtDamage => "Player Dealt Damage",
            Self::PlayerTookDamage => "Player Took Damage",
            Self::PlayerDied => "Player Died",
            Self::PlayerDealtHealing => "Player Dealt Healing",
            Self::PlayerReceivedHealing => "Player Received Healing",
            Self::PlayerJoinedMatch => "Player Joined Match",
            Self::PlayerLeftMatch => "Player Left Match",
            Self::PlayerDealtKnockback => "Player Dealt Knockback",
            Self::PlayerReceivedKnockback => "Player Received Knockback",
        }
    }
}

/// The team line of a player event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TeamFilter {
    /// `All`.
    All,
    /// `Team 1`.
    Team1,
    /// `Team 2`.
    Team2,
}

/// The player line of a player event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerFilter {
    /// `All`.
    All,
    /// `Slot N`, 0 to 11.
    Slot(u8),
    /// One hero.
    Hero(Hero),
}
