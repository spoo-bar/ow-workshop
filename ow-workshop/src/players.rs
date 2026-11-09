//! Player queries that skip dead and unspawned players.
//!
//! The built-in `Closest Player To` and friends also return players that have not spawned,
//! which sit at the origin; these helpers filter them out.

use crate::expr::Expr;
use crate::generated::api::{
    GetLivingPlayersArgs, GetPlayersInRadiusArgs, PlayerExt, angle_between_vectors, distance,
    living_players_with, opposite_team, players_in_radius_with,
};
use crate::ty::{Array, Hero, IntoExpr, Is, LosCheck, Num, Player, Position, Team};

/// Why a player helper rejected its arguments.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PlayerHelperError {
    /// An argument is random, so its copies inside the per-element lambda would differ.
    RandomArgument(&'static str),
}

impl std::fmt::Display for PlayerHelperError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RandomArgument(arg) => {
                write!(f, "`{arg}` cannot be random: it is evaluated per player")
            }
        }
    }
}

impl std::error::Error for PlayerHelperError {}

/// Optional arguments of the team-filtered queries.
#[derive(Clone, Debug)]
pub struct RealPlayersArgs {
    /// Which team's players to consider; default: all teams.
    pub team: Expr<Team>,
}

impl Default for RealPlayersArgs {
    fn default() -> Self {
        Self {
            team: GetLivingPlayersArgs::default().team,
        }
    }
}

/// Optional arguments of [`real_players_in_radius_with`].
#[derive(Clone, Debug)]
pub struct RealRadiusArgs {
    /// Which team's players to consider; default: all teams.
    pub team: Expr<Team>,
    /// The line-of-sight check; default: off.
    pub los: Expr<LosCheck>,
}

impl Default for RealRadiusArgs {
    fn default() -> Self {
        let d = GetPlayersInRadiusArgs::default();
        Self {
            team: d.team,
            los: d.los_check,
        }
    }
}

/// The workshop treats a player as its position wherever a position is expected.
fn as_position(player: Expr<Player>) -> Expr<Position> {
    player.erase().cast()
}

fn check_not_random<T>(arg: &'static str, value: &Expr<T>) -> Result<(), PlayerHelperError> {
    if crate::opt::contains_random(value.ir()) {
        Err(PlayerHelperError::RandomArgument(arg))
    } else {
        Ok(())
    }
}

/// `Filtered Array(All Living Players(team), Has Spawned(e))`.
#[track_caller]
fn spawned_living(team: Expr<Team>) -> Expr<Array<Player>> {
    living_players_with(GetLivingPlayersArgs { team }).filter(|e, _| e.has_spawned())
}

/// Living, spawned players, nearest to `center` first.
#[track_caller]
pub fn real_closest_players(
    center: impl IntoExpr<Position>,
) -> Result<Expr<Array<Player>>, PlayerHelperError> {
    real_closest_players_with(center, RealPlayersArgs::default())
}

/// [`real_closest_players`] with a team filter.
#[track_caller]
pub fn real_closest_players_with(
    center: impl IntoExpr<Position>,
    opts: RealPlayersArgs,
) -> Result<Expr<Array<Player>>, PlayerHelperError> {
    let center = center.into_expr();
    check_not_random("center", &center)?;
    Ok(spawned_living(opts.team).sorted_by(|e, _| distance(as_position(e), &center)))
}

/// The living, spawned player nearest to `center`.
#[track_caller]
pub fn real_closest_player(
    center: impl IntoExpr<Position>,
) -> Result<Expr<Player>, PlayerHelperError> {
    real_closest_player_with(center, RealPlayersArgs::default())
}

/// [`real_closest_player`] with a team filter.
#[track_caller]
pub fn real_closest_player_with(
    center: impl IntoExpr<Position>,
    opts: RealPlayersArgs,
) -> Result<Expr<Player>, PlayerHelperError> {
    real_closest_players_with(center, opts).map(|players| players.first())
}

/// Living, spawned players, farthest from `center` first.
#[track_caller]
pub fn real_farthest_players(
    center: impl IntoExpr<Position>,
) -> Result<Expr<Array<Player>>, PlayerHelperError> {
    real_farthest_players_with(center, RealPlayersArgs::default())
}

/// [`real_farthest_players`] with a team filter.
#[track_caller]
pub fn real_farthest_players_with(
    center: impl IntoExpr<Position>,
    opts: RealPlayersArgs,
) -> Result<Expr<Array<Player>>, PlayerHelperError> {
    let center = center.into_expr();
    check_not_random("center", &center)?;
    Ok(spawned_living(opts.team)
        .sorted_by(|e, _| Expr::<Num>::from(-1) * distance(as_position(e), &center)))
}

/// The living, spawned player farthest from `center`.
#[track_caller]
pub fn real_farthest_player(
    center: impl IntoExpr<Position>,
) -> Result<Expr<Player>, PlayerHelperError> {
    real_farthest_player_with(center, RealPlayersArgs::default())
}

/// [`real_farthest_player`] with a team filter.
#[track_caller]
pub fn real_farthest_player_with(
    center: impl IntoExpr<Position>,
    opts: RealPlayersArgs,
) -> Result<Expr<Player>, PlayerHelperError> {
    real_farthest_players_with(center, opts).map(|players| players.first())
}

/// Living, spawned players within `radius` of `center`.
#[track_caller]
#[must_use]
pub fn real_players_in_radius(
    center: impl IntoExpr<Position>,
    radius: impl IntoExpr<Num>,
) -> Expr<Array<Player>> {
    real_players_in_radius_with(center, radius, RealRadiusArgs::default())
}

/// [`real_players_in_radius`] with a team filter and line-of-sight check.
#[track_caller]
#[must_use]
pub fn real_players_in_radius_with(
    center: impl IntoExpr<Position>,
    radius: impl IntoExpr<Num>,
    opts: RealRadiusArgs,
) -> Expr<Array<Player>> {
    let args = GetPlayersInRadiusArgs {
        team: opts.team,
        los_check: opts.los,
    };
    players_in_radius_with(center, radius, args).filter(|e, _| e.is_alive() & e.has_spawned())
}

/// Helpers on a player expression.
pub trait PlayerHelpers {
    /// The hero being duplicated, else the hero played.
    #[track_caller]
    fn effective_hero(&self) -> Expr<Hero>;

    /// The team opposing this player's team.
    #[track_caller]
    fn opposite_team(&self) -> Expr<Team>;

    /// Living, spawned players other than this one, nearest to the reticle first.
    #[track_caller]
    fn real_players_closest_to_reticle(&self) -> Result<Expr<Array<Player>>, PlayerHelperError> {
        self.real_players_closest_to_reticle_with(RealPlayersArgs::default())
    }

    /// [`Self::real_players_closest_to_reticle`] with a team filter.
    #[track_caller]
    fn real_players_closest_to_reticle_with(
        &self,
        opts: RealPlayersArgs,
    ) -> Result<Expr<Array<Player>>, PlayerHelperError>;

    /// The living, spawned player (not this one) nearest to the reticle.
    #[track_caller]
    fn real_player_closest_to_reticle(&self) -> Result<Expr<Player>, PlayerHelperError> {
        self.real_player_closest_to_reticle_with(RealPlayersArgs::default())
    }

    /// [`Self::real_player_closest_to_reticle`] with a team filter.
    #[track_caller]
    fn real_player_closest_to_reticle_with(
        &self,
        opts: RealPlayersArgs,
    ) -> Result<Expr<Player>, PlayerHelperError> {
        self.real_players_closest_to_reticle_with(opts)
            .map(|players| players.first())
    }

    /// Living, spawned players of `team` within `angle` degrees of this player's view.
    #[track_caller]
    fn real_players_in_view_angle(
        &self,
        team: impl IntoExpr<Team>,
        angle: impl IntoExpr<Num>,
    ) -> Expr<Array<Player>>;
}

impl<T: Is<Player>> PlayerHelpers for Expr<T> {
    fn effective_hero(&self) -> Expr<Hero> {
        self.hero_of_duplication().or(self.hero()).erase().cast()
    }

    fn opposite_team(&self) -> Expr<Team> {
        opposite_team(self.team())
    }

    fn real_players_closest_to_reticle_with(
        &self,
        opts: RealPlayersArgs,
    ) -> Result<Expr<Array<Player>>, PlayerHelperError> {
        let player: Expr<Player> = self.clone().erase().cast();
        check_not_random("player", &player)?;
        let candidates = living_players_with(GetLivingPlayersArgs { team: opts.team })
            .filter(|e, _| e.has_spawned() & e.ne(&player));
        Ok(candidates.sorted_by(|e, _| {
            angle_between_vectors(
                player.facing_direction(),
                as_position(e) - player.eye_position(),
            )
        }))
    }

    fn real_players_in_view_angle(
        &self,
        team: impl IntoExpr<Team>,
        angle: impl IntoExpr<Num>,
    ) -> Expr<Array<Player>> {
        self.players_in_view_angle(team, angle)
            .filter(|e, _| e.is_alive() & e.has_spawned())
    }
}

#[cfg(test)]
mod tests;
