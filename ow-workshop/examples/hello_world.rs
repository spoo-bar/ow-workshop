//! The smallest useful mod: `cargo run -p ow-workshop --example hello_world`.
#![cfg_attr(target_arch = "wasm32", no_main)]

use ow_workshop::prelude::*;
use ow_workshop::{Gamemode, Map};

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

    fn settings(&self) -> Settings {
        let maps = MapSelection::Enabled(vec![Map::WorkshopChamber.into()]);
        Settings::default()
            .with_main(Main {
                mode_name: "Hello World".to_owned(),
                description: "The smallest useful mod".to_owned(),
            })
            .with_mode(
                ModeKey::Mode(Gamemode::TeamDeathmatch),
                ModeSettings::default().maps(maps),
            )
    }
}

ow_workshop::main!(HelloWorld);
