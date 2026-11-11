//! Waves of bots against the players: `cargo run -p ow-workshop --example zombie_survival`.
#![cfg_attr(target_arch = "wasm32", no_main)]

use ow_workshop::prelude::*;
use ow_workshop::{Gamemode, Map};

#[derive(Clone, Copy, WorkshopEnum)]
pub enum GameState {
    Lobby,
    Playing,
    Over,
}

#[derive(GlobalVars)]
pub struct Globals {
    wave: Global<Num>,
    // A debug build shows the state by name instead of as a number.
    #[ow(debug_with = GameState::debug_render)]
    state: Global<Num>,
    spawn_points: Global<Array<Position>>,
    spawn_index: Global<Num>,
}

#[derive(PlayerVars)]
pub struct Players {
    score: PlayerVar<Num>,
    #[ow(init = 3)]
    lives: PlayerVar<Num>,
}

fn enter(b: &mut Block, g: &Globals, state: GameState) {
    b.set(g.state, state);
    b.switch(g.state.get())
        .case(GameState::Playing, |b| {
            b.big_message("Survive!");
            b.break_();
        })
        .case(GameState::Over, |b| {
            b.big_message("Game over");
            b.break_();
        });
}

pub struct ZombieSurvival {
    pub waves: u8,
}

impl Workshop for ZombieSurvival {
    const NAME: &'static str = "Zombie Survival";
    const DESCRIPTION: &'static str = "Survive waves of zombies";

    type Globals = Globals;
    type Players = Players;

    fn build(&self, ws: &mut Builder, g: &Globals, p: &Players) {
        let spawn_wave = ws.declare_subroutine("spawn_wave");

        ws.group("Setup", |ws| {
            ws.rule("Collect spawn points", OngoingGlobal, |r, _| {
                r.actions(|b| {
                    b.disable_inspector();
                    b.set(g.spawn_points, spawn_points(Team::Two));
                    b.set(g.state, GameState::Lobby);
                });
            });

            ws.rule("Start when players are in", OngoingGlobal, |r, _| {
                r.condition(g.state.get().eq(GameState::Lobby));
                r.condition(number_of_players().ge(1));
                r.actions(|b| {
                    b.wait(5);
                    enter(b, g, GameState::Playing);
                });
            });
        });

        ws.group("Waves", |ws| {
            ws.define(spawn_wave, |b, _| {
                b.add_to(g.wave, 1);
                b.for_range(
                    g.spawn_index,
                    range(0, min(len(g.spawn_points.get()), g.wave.get() + 3)),
                    |b| {
                        b.create_dummy(
                            Hero::Reaper,
                            Team::Two,
                            g.spawn_index.get(),
                            g.spawn_points.get().get(g.spawn_index.get()),
                        );
                    },
                );
            });

            let waves = f64::from(self.waves);
            ws.rule("Run the waves", OngoingGlobal, |r, _| {
                r.condition(g.state.get().eq(GameState::Playing));
                r.actions(|b| {
                    b.while_(g.wave.get().lt(waves), |b| {
                        b.call(spawn_wave);
                        b.wait_until(
                            all_players()
                                .filter(|pl, _| pl.team().eq(Team::Two))
                                .len()
                                .eq(0),
                            120,
                        );
                        b.wait(5);
                    });
                    enter(b, g, GameState::Over);
                });
            });
        });

        ws.group("Players", |ws| {
            ws.rule("Score on kill", PlayerDied, |r, ev| {
                r.condition(ev.attacker().ne(ev.victim()));
                r.condition(ev.victim().team().eq(Team::Two));
                r.actions(|b| b.add_to(ev.attacker().var(p.score), 100));
            });

            ws.rule("Lose a life", PlayerDied, |r, ev| {
                r.condition(ev.victim().team().eq(Team::One));
                r.actions(|b| {
                    b.add_to(ev.victim().var(p.lives), -1);
                    b.if_(ev.victim().var(p.lives).get().le(0), |b| {
                        b.disable_respawn(ev.victim());
                    });
                });
            });
        });

        ws.group("HUD", |ws| {
            ws.rule("Wave counter", OngoingGlobal, |r, _| {
                r.actions(|b| {
                    b.hud_header(ws_fmt!("Wave {}", g.wave.get()));
                    b.hud_subheader(ws_fmt!("{} survivors", number_of_living_players()));
                });
            });

            ws.rule("Leaderboard", OngoingGlobal, |r, _| {
                r.actions(|b| {
                    let ranked = all_players()
                        .filter(|pl, _| pl.team().eq(Team::One))
                        .sorted_by(|pl, _| -pl.var(p.score).get());
                    b.hud_subtext(ws_fmt!("Leader: {}", ranked.first()));
                });
            });
        });
    }

    fn settings(&self) -> Settings {
        let maps = MapSelection::Enabled(vec![Map::WorkshopChamber.into()]);
        Settings::default().with_mode(
            ModeKey::Mode(Gamemode::TeamDeathmatch),
            ModeSettings::default().maps(maps),
        )
    }
}

ow_workshop::main!(ZombieSurvival { waves: 5 });
