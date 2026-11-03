//! A small deathmatch mod: `cargo run -p ow-workshop --example deathmatch`.

use ow_workshop::prelude::*;

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
}

ow_workshop::main!(Deathmatch);
