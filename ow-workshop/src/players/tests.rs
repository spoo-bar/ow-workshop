use super::*;
use crate::expr::vect;
use crate::generated::api::{host_player, random};
use crate::opt::testutil::sh;
use crate::ty::{LosCheck, Team};

fn show<T>(e: &Expr<T>) -> String {
    sh(e.ir())
}

const SPAWNED: &str = "FilteredArray(GetLivingPlayers(Team.ALL), HasSpawned(@))";
const RETICLE_CANDIDATES: &str =
    "FilteredArray(GetLivingPlayers(Team.ALL), (HasSpawned(@) && (@ != HostPlayer())))";
const RETICLE_KEY: &str =
    "AngleBetweenVectors(GetFacingDirection(HostPlayer()), (@ - GetEyePosition(HostPlayer())))";

#[test]
fn closest_players_are_sorted_by_distance() {
    let center = vect(1, 2, 3);
    assert_eq!(
        show(&real_closest_players(&center).unwrap()),
        format!("SortedArray({SPAWNED}, Distance(@, V(1, 2, 3)))")
    );
    assert_eq!(
        show(&real_closest_player(&center).unwrap()),
        format!("FirstOf(SortedArray({SPAWNED}, Distance(@, V(1, 2, 3))))")
    );
}

#[test]
fn farthest_players_negate_the_distance() {
    let center = vect(1, 2, 3);
    assert_eq!(
        show(&real_farthest_players(&center).unwrap()),
        format!("SortedArray({SPAWNED}, (-1 * Distance(@, V(1, 2, 3))))")
    );
    assert_eq!(
        show(&real_farthest_player(&center).unwrap()),
        format!("FirstOf(SortedArray({SPAWNED}, (-1 * Distance(@, V(1, 2, 3)))))")
    );
}

#[test]
fn the_team_argument_replaces_all_teams() {
    let opts = RealPlayersArgs {
        team: Team::All.into(),
    };
    let default = real_closest_players(vect(0, 0, 0)).unwrap();
    let explicit = real_closest_players_with(vect(0, 0, 0), opts).unwrap();
    assert_eq!(show(&default), show(&explicit));
    let opts = RealPlayersArgs {
        team: crate::generated::api::opposite_team(Team::All),
    };
    let text = show(&real_farthest_player_with(vect(0, 0, 0), opts).unwrap());
    assert!(
        text.contains("GetLivingPlayers(GetOppositeTeam(Team.ALL))"),
        "{text}"
    );
}

#[test]
fn effective_hero_prefers_the_duplicated_hero() {
    assert_eq!(
        show(&host_player().effective_hero()),
        "(GetHeroOfDuplication(HostPlayer()) || GetHero(HostPlayer()))"
    );
}

#[test]
fn opposite_team_of_the_players_team() {
    assert_eq!(
        show(&host_player().opposite_team()),
        "GetOppositeTeam(GetTeam(HostPlayer()))"
    );
}

#[test]
fn reticle_queries_exclude_the_player_and_sort_by_angle() {
    let p = host_player();
    assert_eq!(
        show(&p.real_players_closest_to_reticle().unwrap()),
        format!("SortedArray({RETICLE_CANDIDATES}, {RETICLE_KEY})")
    );
    assert_eq!(
        show(&p.real_player_closest_to_reticle().unwrap()),
        format!("FirstOf(SortedArray({RETICLE_CANDIDATES}, {RETICLE_KEY}))")
    );
}

#[test]
fn players_in_radius_must_be_alive_and_spawned() {
    let center = vect(1, 2, 3);
    assert_eq!(
        show(&real_players_in_radius(&center, 5)),
        "FilteredArray(GetPlayersInRadius(V(1, 2, 3), 5, Team.ALL, LosCheck.OFF), \
         (IsAlive(@) && HasSpawned(@)))"
    );
    let opts = RealRadiusArgs {
        team: Team::All.into(),
        los: LosCheck::Surfaces.into(),
    };
    let text = show(&real_players_in_radius_with(&center, 5, opts));
    assert!(text.contains("LosCheck.SURFACES"), "{text}");
}

#[test]
fn players_in_view_angle_must_be_alive_and_spawned() {
    assert_eq!(
        show(&host_player().real_players_in_view_angle(Team::All, 30)),
        "FilteredArray(GetPlayersInViewAngle(HostPlayer(), Team.ALL, 30), \
         (IsAlive(@) && HasSpawned(@)))"
    );
}

#[test]
fn random_arguments_used_per_element_are_rejected() {
    let random_center = vect(random::int(0, 10), 0, 0);
    let error = Err(PlayerHelperError::RandomArgument("center"));
    assert_eq!(real_closest_players(&random_center).map(|_| ()), error);
    assert_eq!(real_closest_player(&random_center).map(|_| ()), error);
    assert_eq!(real_farthest_players(&random_center).map(|_| ()), error);
    assert_eq!(real_farthest_player(&random_center).map(|_| ()), error);

    let random_player: Expr<Player> = random::choice(crate::generated::api::all_players()).cast();
    assert_eq!(
        random_player.real_players_closest_to_reticle().map(|_| ()),
        Err(PlayerHelperError::RandomArgument("player"))
    );
    assert_eq!(
        random_player.real_player_closest_to_reticle().map(|_| ()),
        Err(PlayerHelperError::RandomArgument("player"))
    );
}
