//! Value constructors print the same text as the reference scripts.

use std::collections::HashSet;

use ow_workshop::data::{
    Button, Color, Comms, Gamemode, Health, Hero, Icon, Map, Rounding, Status, Team,
};
use ow_workshop::values::*;
use ow_workshop::{CompareOp, Global, PlayerVar, Value, ValueId};

fn global(name: &'static str) -> Global {
    Global {
        name,
        index: 0,
        debug: None,
    }
}

fn player_var(name: &'static str) -> PlayerVar {
    PlayerVar {
        name,
        index: 0,
        debug: None,
    }
}

fn ep() -> Value {
    event_player()
}

fn assert_text(cases: Vec<(Value, &str)>) {
    for (value, text) in cases {
        assert_eq!(value.to_string(), text);
    }
}

#[test]
fn every_value_has_a_constructor() {
    let covered: HashSet<_> = COVERED.iter().collect();
    assert_eq!(covered.len(), COVERED.len(), "a value is listed twice");
    for id in ValueId::ALL {
        assert!(covered.contains(id), "no constructor for {id:?}");
    }
}

#[test]
fn zero_argument_values_print_without_parentheses() {
    assert_text(vec![
        (event_player(), "Event Player"),
        (null(), "Null"),
        (all_heroes(), "All Heroes"),
        (is_game_in_progress(), "Is Game In Progress"),
        (current_array_element(), "Current Array Element"),
        (host_player(), "Host Player"),
        (local_player(), "Local Player"),
        (attacker(), "Attacker"),
        (victim(), "Victim"),
        (empty_array(), "Empty Array"),
        (current_game_mode(), "Current Game Mode"),
        (current_map(), "Current Map"),
        (objective_index(), "Objective Index"),
        (payload_position(), "Payload Position"),
        (payload_progress_percentage(), "Payload Progress Percentage"),
        (last_created_entity(), "Last Created Entity"),
        (last_text_id(), "Last Text ID"),
        (text_count(), "Text Count"),
        (entity_count(), "Entity Count"),
        (match_time(), "Match Time"),
        (total_time_elapsed(), "Total Time Elapsed"),
        (server_load(), "Server Load"),
        (server_load_average(), "Server Load Average"),
        (server_load_peak(), "Server Load Peak"),
        (is_assembling_heroes(), "Is Assembling Heroes"),
        (is_between_rounds(), "Is Between Rounds"),
        (is_in_setup(), "Is In Setup"),
        (is_match_complete(), "Is Match Complete"),
        (event_ability(), "Event Ability"),
        (event_damage(), "Event Damage"),
        (true_(), "True"),
        (false_(), "False"),
        (up(), "Up"),
        (down(), "Down"),
        (left(), "Left"),
        (right(), "Right"),
        (forward(), "Forward"),
        (backward(), "Backward"),
    ]);
}

#[test]
fn math() {
    let p = player_var;
    let fall_velocity = || p("fallVelocity").of(ep());
    assert_text(vec![
        (
            divide(fall_velocity() * fall_velocity(), 15),
            "Divide(Multiply((Event Player).fallVelocity, (Event Player).fallVelocity), 15)",
        ),
        (
            modulo(add(p("mode").of(ep()), 1), 2),
            "Modulo(Add((Event Player).mode, 1), 2)",
        ),
        (
            subtract(ammo(ep(), 0), 0.1),
            "Subtract(Ammo(Event Player, 0), 0.1)",
        ),
        (
            multiply(
                divide(global("statFallDeaths").get(), global("statDeaths").get()),
                100,
            ),
            "Multiply(Divide(Global.statFallDeaths, Global.statDeaths), 100)",
        ),
        (
            max(0, p("fallDamageTaken").of(ep())),
            "Max(0, (Event Player).fallDamageTaken)",
        ),
        (min(random_integer(0, 1), 4), "Min(Random Integer(0, 1), 4)"),
        (
            random_real(
                subtract(1, global("RANDOM_SPREAD_PERCENTAGE").get() / 100),
                add(1, global("RANDOM_SPREAD_PERCENTAGE").get() / 100),
            ),
            "Random Real(Subtract(1, Divide(Global.RANDOM_SPREAD_PERCENTAGE, 100)), \
             Add(1, Divide(Global.RANDOM_SPREAD_PERCENTAGE, 100)))",
        ),
        (
            round_to_integer(
                global("statFallDamageTaken").get(),
                Rounding::RoundToNearest,
            ),
            "Round To Integer(Global.statFallDamageTaken, To Nearest)",
        ),
        (
            round_to_integer(max_ammo(ep(), 0) * 4, Rounding::RoundUp),
            "Round To Integer(Multiply(Max Ammo(Event Player, 0), 4), Up)",
        ),
        (number(0.5), "0.5"),
    ]);
}

#[test]
fn operators() {
    assert_text(vec![
        (
            max(0, -vertical_speed_of(ep())),
            "Max(0, Multiply(-1, Vertical Speed Of(Event Player)))",
        ),
        (
            count_of(all_heroes()) - 1,
            "Subtract(Count Of(All Heroes), 1)",
        ),
        (-Value::from(60), "-60"),
        (
            position_of(ep()) + direction_from_angles(0, 0) * 0.01,
            "Add(Position Of(Event Player), Multiply(Direction From Angles(0, 0), 0.01))",
        ),
        (
            (player_var("debug_feed_num").of(ep())) % global("DEBUG_FEED_MAX").get(),
            "Modulo((Event Player).debug_feed_num, Global.DEBUG_FEED_MAX)",
        ),
        (!is_on_ground(ep()), "Not(Is On Ground(Event Player))"),
    ]);
}

#[test]
fn logic_and_compare() {
    let p = |name| player_var(name).of(ep());
    assert_text(vec![
        (
            and(global("LOG_STATS").get(), is_game_in_progress()),
            "And(Global.LOG_STATS, Is Game In Progress)",
        ),
        (
            or(not(is_game_in_progress()), p("switchedHero").equals(true)),
            "Or(Not(Is Game In Progress), Compare((Event Player).switchedHero, ==, True))",
        ),
        (
            p("fallImmunityState").not_equals(0),
            "Compare((Event Player).fallImmunityState, !=, 0)",
        ),
        (
            compare(text_count(), CompareOp::Gt, 128),
            "Compare(Text Count, >, 128)",
        ),
        (
            magnitude_of(p("pointOffset")).lt(14),
            "Compare(Magnitude Of((Event Player).pointOffset), <, 14)",
        ),
        (
            p("heroIndex").ge(0),
            "Compare((Event Player).heroIndex, >=, 0)",
        ),
        (
            ability_cooldown(ep(), Button::Ability2).gt(0),
            "Compare(Ability Cooldown(Event Player, Button(Ability 2)), >, 0)",
        ),
        (
            ultimate_charge_percent(ep()).le(100),
            "Compare(Ultimate Charge Percent(Event Player), <=, 100)",
        ),
        (
            if_then_else(team_of(ep()).equals(Team::Two), 6, 0),
            "If-Then-Else(Compare(Team Of(Event Player), ==, Team 2), 6, 0)",
        ),
        (
            if_then_else(
                current_game_mode().equals(game_mode(Gamemode::Deathmatch)),
                Team::All,
                Team::One,
            ),
            "If-Then-Else(Compare(Current Game Mode, ==, Game Mode(Deathmatch)), All Teams, Team 1)",
        ),
    ]);
}

#[test]
fn arrays() {
    let g = |name| global(name).get();
    let pos = || position_of(ep());
    assert_text(vec![
        (
            array([0.into(), 1.into(), 2.into(), 3.into(), 4.into()]),
            "Array(0, 1, 2, 3, 4)",
        ),
        (
            random_value_in_array(array([Hero::Ana.into(), Hero::Mercy.into()])),
            "Random Value In Array(Array(Hero(Ana), Hero(Mercy)))",
        ),
        (
            randomized_array(array([0.into(), 1.into()])),
            "Randomized Array(Array(0, 1))",
        ),
        (
            array_contains(g("vectors"), g("v")),
            "Array Contains(Global.vectors, Global.v)",
        ),
        (
            filtered_array(
                all_players(Team::All),
                is_dummy_bot(current_array_element()),
            ),
            "Filtered Array(All Players(All Teams), Is Dummy Bot(Current Array Element))",
        ),
        (
            mapped_array(array([0.into()]), current_array_index()),
            "Mapped Array(Array(0), Current Array Index)",
        ),
        (
            first_of(sorted_array(
                g("AMMO_PACK_POSITIONS"),
                distance_between(pos(), current_array_element()),
            )),
            "First Of(Sorted Array(Global.AMMO_PACK_POSITIONS, \
             Distance Between(Position Of(Event Player), Current Array Element)))",
        ),
        (
            is_true_for_any(
                g("AMMO_PACK_POSITIONS"),
                distance_between(pos(), current_array_element()).lt(1.1),
            ),
            "Is True For Any(Global.AMMO_PACK_POSITIONS, \
             Compare(Distance Between(Position Of(Event Player), Current Array Element), <, 1.1))",
        ),
        (
            is_true_for_all(all_players(Team::All), is_alive(current_array_element())),
            "Is True For All(All Players(All Teams), Is Alive(Current Array Element))",
        ),
        (
            index_of_array_value(all_heroes(), hero_of(ep())),
            "Index Of Array Value(All Heroes, Hero Of(Event Player))",
        ),
        (
            value_in_array(g("vectors"), g("i")),
            "Value In Array(Global.vectors, Global.i)",
        ),
        (
            remove_from_array(all_heroes(), hero_of(ep())),
            "Remove From Array(All Heroes, Hero Of(Event Player))",
        ),
        (
            append_to_array(g("vectors"), g("v")),
            "Append To Array(Global.vectors, Global.v)",
        ),
        (
            array_slice(g("C_AMMO_PACKS"), first_of(g("SLICE")), 2),
            "Array Slice(Global.C_AMMO_PACKS, First Of(Global.SLICE), 2)",
        ),
        (
            count_of(spawn_points(Team::One)),
            "Count Of(Spawn Points(Team 1))",
        ),
        (last_of(g("vectors")), "Last Of(Global.vectors)"),
        (
            string_split(first_of(last_created_entity()), empty_array()),
            "String Split(First Of(Last Created Entity), Empty Array)",
        ),
    ]);
}

#[test]
fn vectors() {
    let g = |name| global(name).get();
    assert_text(vec![
        (
            vector(z_component_of(g("n")), 0, -x_component_of(g("n"))),
            "Vector(Z Component Of(Global.n), 0, Multiply(-1, X Component Of(Global.n)))",
        ),
        (
            vector(
                round_to_integer(x_component_of(g("p")), Rounding::RoundToNearest),
                y_component_of(g("p")),
                0,
            ),
            "Vector(Round To Integer(X Component Of(Global.p), To Nearest), Y Component Of(Global.p), 0)",
        ),
        (
            cross_product(facing_direction_of(host_player()), up()),
            "Cross Product(Facing Direction Of(Host Player), Up)",
        ),
        (
            dot_product(g("n"), g("n")),
            "Dot Product(Global.n, Global.n)",
        ),
        (
            direction_from_angles(
                horizontal_angle_from_direction(facing_direction_of(ep())),
                0,
            ),
            "Direction From Angles(Horizontal Angle From Direction(Facing Direction Of(Event Player)), 0)",
        ),
        (
            angle_between_vectors(
                facing_direction_of(ep()),
                direction_towards(eye_position(ep()), g("p")),
            ),
            "Angle Between Vectors(Facing Direction Of(Event Player), \
             Direction Towards(Eye Position(Event Player), Global.p))",
        ),
        (normalize(g("n")), "Normalize(Global.n)"),
        (magnitude_of(g("n")), "Magnitude Of(Global.n)"),
        (
            sine_from_radians(total_time_elapsed()),
            "Sine From Radians(Total Time Elapsed)",
        ),
    ]);
}

#[test]
fn player_state() {
    assert_text(vec![
        (has_spawned(ep()), "Has Spawned(Event Player)"),
        (is_alive(ep()), "Is Alive(Event Player)"),
        (is_dead(ep()), "Is Dead(Event Player)"),
        (is_on_ground(ep()), "Is On Ground(Event Player)"),
        (is_on_wall(ep()), "Is On Wall(Event Player)"),
        (is_on_objective(ep()), "Is On Objective(Event Player)"),
        (is_in_spawn_room(ep()), "Is In Spawn Room(Event Player)"),
        (is_dummy_bot(ep()), "Is Dummy Bot(Event Player)"),
        (is_duplicating(ep()), "Is Duplicating(Event Player)"),
        (
            is_in_alternate_form(ep()),
            "Is In Alternate Form(Event Player)",
        ),
        (is_firing_primary(ep()), "Is Firing Primary(Event Player)"),
        (
            is_firing_secondary(ep()),
            "Is Firing Secondary(Event Player)",
        ),
        (is_meleeing(ep()), "Is Meleeing(Event Player)"),
        (is_reloading(ep()), "Is Reloading(Event Player)"),
        (is_using_ability_1(ep()), "Is Using Ability 1(Event Player)"),
        (is_using_ability_2(ep()), "Is Using Ability 2(Event Player)"),
        (is_using_ultimate(ep()), "Is Using Ultimate(Event Player)"),
        (
            is_button_held(ep(), Button::Melee),
            "Is Button Held(Event Player, Button(Melee))",
        ),
        (
            is_communicating(ep(), Comms::Hello),
            "Is Communicating(Event Player, Hello)",
        ),
        (
            has_status(ep(), Status::PhasedOut),
            "Has Status(Event Player, Phased Out)",
        ),
        (
            max_health_of_type(ep(), Health::Armor),
            "Max Health Of Type(Event Player, Armor)",
        ),
        (
            health_of_type(ep(), Health::Shields),
            "Health Of Type(Event Player, Shields)",
        ),
        (health(ep()), "Health(Event Player)"),
        (max_health(ep()), "Max Health(Event Player)"),
        (ammo(ep(), 0), "Ammo(Event Player, 0)"),
        (max_ammo(ep(), 0), "Max Ammo(Event Player, 0)"),
        (weapon(ep()), "Weapon(Event Player)"),
        (hero_of(attacker()), "Hero Of(Attacker)"),
        (
            hero_being_duplicated(ep()),
            "Hero Being Duplicated(Event Player)",
        ),
        (allowed_heroes(ep()), "Allowed Heroes(Event Player)"),
        (position_of(host_player()), "Position Of(Host Player)"),
        (eye_position(ep()), "Eye Position(Event Player)"),
        (
            facing_direction_of(ep()),
            "Facing Direction Of(Event Player)",
        ),
        (speed_of(ep()), "Speed Of(Event Player)"),
        (vertical_speed_of(ep()), "Vertical Speed Of(Event Player)"),
        (slot_of(ep()), "Slot Of(Event Player)"),
        (team_of(host_player()), "Team Of(Host Player)"),
        (
            ultimate_charge_percent(ep()),
            "Ultimate Charge Percent(Event Player)",
        ),
        (
            evaluate_once(hero_of(ep())),
            "Evaluate Once(Hero Of(Event Player))",
        ),
        (
            update_every_frame(facing_direction_of(local_player())),
            "Update Every Frame(Facing Direction Of(Local Player))",
        ),
    ]);
}

#[test]
fn world_queries_and_ray_casts() {
    let ahead = || eye_position(host_player()) + facing_direction_of(host_player()) * 100;
    assert_text(vec![
        (
            ray_cast_hit_position(eye_position(host_player()), ahead(), null(), null(), false),
            "Ray Cast Hit Position(Eye Position(Host Player), Add(Eye Position(Host Player), \
             Multiply(Facing Direction Of(Host Player), 100)), Null, Null, False)",
        ),
        (
            ray_cast_hit_player(
                eye_position(host_player()),
                ahead(),
                all_players(Team::All),
                ep(),
                true,
            ),
            "Ray Cast Hit Player(Eye Position(Host Player), Add(Eye Position(Host Player), \
             Multiply(Facing Direction Of(Host Player), 100)), All Players(All Teams), Event Player, True)",
        ),
        (
            all_living_players(team_of(ep())),
            "All Living Players(Team Of(Event Player))",
        ),
        (number_of_players(Team::All), "Number Of Players(All Teams)"),
        (number_of_slots(Team::Two), "Number Of Slots(Team 2)"),
        (
            opposite_team_of(team_of(ep())),
            "Opposite Team Of(Team Of(Event Player))",
        ),
        (
            team_score(team_of(host_player())),
            "Team Score(Team Of(Host Player))",
        ),
        (
            objective_position(objective_index()),
            "Objective Position(Objective Index)",
        ),
        (
            entity_exists(value_in_array(
                global("g_PlayerEntities").get(),
                global("s").get(),
            )),
            "Entity Exists(Value In Array(Global.g_PlayerEntities, Global.s))",
        ),
        (
            current_map().equals(Map::Ilios),
            "Compare(Current Map, ==, Map(Ilios))",
        ),
    ]);
}

#[test]
fn wrappers_and_strings() {
    assert_text(vec![
        (hero(Hero::Sigma), "Hero(Sigma)"),
        (Hero::Ana.into(), "Hero(Ana)"),
        (button(Button::Melee), "Button(Melee)"),
        (color(Color::Orange), "Color(Orange)"),
        (Color::Orange.into(), "Color(Orange)"),
        (map(Map::Ilios), "Map(Ilios)"),
        (game_mode(Gamemode::Push), "Game Mode(Push)"),
        (team(Team::Two), "Team(Team 2)"),
        (Team::Two.into(), "Team 2"),
        (custom_color(0, 0, 0, 0), "Custom Color(0, 0, 0, 0)"),
        (icon_string(Icon::Warning), "Icon String(Warning)"),
        (
            hero_icon_string(hero_of(ep())),
            "Hero Icon String(Hero Of(Event Player))",
        ),
        (
            input_binding_string(Button::SecondaryFire),
            "Input Binding String(Button(Secondary Fire))",
        ),
        (
            ability_icon_string(hero_of(ep()), Button::SecondaryFire),
            "Ability Icon String(Hero Of(Event Player), Button(Secondary Fire))",
        ),
        (
            custom_string("{0}%", [payload_progress_percentage()]),
            "Custom String(\"{0}%\", Payload Progress Percentage)",
        ),
        (global_variable(&global("v")), "Global.v"),
        (
            player_variable(ep(), &player_var("rot")),
            "(Event Player).rot",
        ),
    ]);
}
