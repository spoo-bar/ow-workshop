//! A small deathmatch mod: `cargo run -p ow-workshop --example deathmatch`.
#![cfg_attr(target_arch = "wasm32", no_main)]

use ow_workshop::prelude::*;
use ow_workshop::{Gamemode, Map};

#[derive(GlobalVars)]
struct Globals {
    round: Global<Num>,
}

#[derive(PlayerVars)]
struct Players {
    score: PlayerVar<Num>,
}

struct Deathmatch;

impl Workshop for Deathmatch {
    const NAME: &'static str = "Deathmatch";
    const DESCRIPTION: &'static str = "Score on every kill";

    type Globals = Globals;
    type Players = Players;

    fn build(&self, ws: &mut Builder, g: &Globals, p: &Players) {
        ws.rule("Init", OngoingGlobal, |r, _| {
            r.actions(|b| b.set(g.round, 1));
        });

        ws.rule("Score on kill", PlayerDied, |r, ev| {
            r.condition(ev.attacker().ne(ev.victim()));
            r.actions(|b| {
                b.add_to(ev.attacker().var(p.score), 1);
                b.big_message_with(
                    "Score!",
                    BigMessageArgs {
                        visible_to: ev.attacker().into(),
                    },
                );
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

ow_workshop::main!(Deathmatch);
