//! Localized structural words.

use ow_workshop_data::{Hero, Keyword, Lang};

use crate::ir::{Event, PlayerSlot, TeamFilter};

/// The keyword that names `event`.
pub(super) fn event(event: Event) -> Keyword {
    match event {
        Event::OngoingGlobal => Keyword::EventGlobal,
        Event::OngoingEachPlayer => Keyword::EventEachPlayer,
        Event::PlayerEarnedElimination => Keyword::EventPlayerEarnedElimination,
        Event::PlayerDealtFinalBlow => Keyword::EventPlayerDealtFinalBlow,
        Event::PlayerDealtDamage => Keyword::EventPlayerDealtDamage,
        Event::PlayerTookDamage => Keyword::EventPlayerTookDamage,
        Event::PlayerDied => Keyword::EventPlayerDied,
        Event::PlayerDealtHealing => Keyword::EventPlayerDealtHealing,
        Event::PlayerReceivedHealing => Keyword::EventPlayerReceivedHealing,
        Event::PlayerJoinedMatch => Keyword::EventPlayerJoined,
        Event::PlayerLeftMatch => Keyword::EventPlayerLeft,
        Event::PlayerDealtKnockback => Keyword::EventPlayerDealtKnockback,
        Event::PlayerReceivedKnockback => Keyword::EventPlayerReceivedKnockback,
        Event::Subroutine(_) => Keyword::EventSubroutine,
    }
}

/// The keyword of a team filter.
pub(super) fn team(team: TeamFilter) -> Keyword {
    match team {
        TeamFilter::All => Keyword::EventTeamAll,
        TeamFilter::Team1 => Keyword::EventTeamTeam1,
        TeamFilter::Team2 => Keyword::EventTeamTeam2,
    }
}

/// The keyword of a player filter, if the data has one.
pub(super) fn player(player: PlayerSlot) -> Option<Keyword> {
    match player {
        PlayerSlot::All => Some(Keyword::EventPlayerAll),
        PlayerSlot::Slot(slot) => find(&format!("slot{slot}")),
        PlayerSlot::Hero(hero) => hero_keyword(hero),
    }
}

fn hero_keyword(hero: Hero) -> Option<Keyword> {
    find(hero.key())
}

fn find(suffix: &str) -> Option<Keyword> {
    Keyword::ALL
        .iter()
        .copied()
        .find(|k| k.key().strip_prefix("event_player.") == Some(suffix))
}

/// The localized word for `keyword`.
pub(super) fn word(keyword: Keyword, lang: Lang) -> &'static str {
    keyword.name(lang)
}
