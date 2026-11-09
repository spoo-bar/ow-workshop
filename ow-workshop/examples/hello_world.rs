//! The smallest useful mod: `cargo run -p ow-workshop --example hello_world`.
#![cfg_attr(target_arch = "wasm32", no_main)]

use ow_workshop::prelude::*;

pub struct HelloWorld;

impl Workshop for HelloWorld {
    type Globals = ();
    type Players = ();

    fn build(&self, ws: &mut Builder, (): &(), (): &()) {
        ws.rule("Header", OngoingGlobal, |r, _| {
            r.actions(|b| {
                b.disable_inspector();
                b.hud_header("Hello, Workshop!");
            });
        });

        ws.rule("Welcome", OngoingEachPlayer, |r, ev| {
            r.condition(ev.event_player().has_spawned());
            r.actions(|b| {
                b.small_message_with(
                    ws_fmt!("Welcome, {}!", ev.event_player()),
                    SmallMessageArgs {
                        visible_to: ev.event_player().into(),
                    },
                );
            });
        });
    }
}

ow_workshop::main!(HelloWorld);
