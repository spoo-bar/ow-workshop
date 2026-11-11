//! The strings the in-game checklist (`docs/in-game-checklist.md`) pastes into Overwatch:
//! `cargo run -p ow-workshop --example in_game_checklist`.
#![cfg_attr(target_arch = "wasm32", no_main)]

use ow_workshop::prelude::*;
use ow_workshop::{Gamemode, Map};

/// A 300-character text with seven arguments.
fn long_text() -> Expr<Str> {
    ws_fmt!(
        "Round {} of the match, with {} players, {} alive, {} dead, {} slots, {} seconds and {} total. word00 word01 word02 word03 word04 word05 word06 word07 word08 word09 word10 word11 word12 word13 word14 word15 word16 word17 word18 word19 word20 word21 word22 word23 word24 word25 word26 word27 word28 wor",
        number_of_players(),
        match_round(),
        number_of_living_players(),
        number_of_dead_players(),
        number_of_slots(),
        match_time(),
        total_time_elapsed(),
    )
}

pub struct InGameChecklist;

impl Workshop for InGameChecklist {
    type Globals = ();
    type Players = ();

    fn build(&self, ws: &mut Builder, (): &(), (): &()) {
        ws.rule("Strings", OngoingGlobal, |r, _| {
            r.actions(|b| {
                b.hud_header(long_text());
                b.hud_subheader(ws_fmt!("<fgFF0000FF>Red</fg> and {}", Texture::Ultimate));
                b.hud_subtext(ws_fmt!(
                    "{} {}",
                    time_to_string(match_time()),
                    button_to_string(Button::Jump)
                ));
                b.hud_subtext(array_to_string([1, 2, 3], 16));
            });
        });

        ws.rule("Lowercase", OngoingGlobal, |r, _| {
            r.actions(|b| {
                b.cased_progress_bar_text(
                    3,
                    ws_fmt!("lowercase text"),
                    vect(0, 1, 0).erase().cast::<PlayerOrPosition>(),
                )
                .expect("builds");
            });
        });
    }

    fn options(&self) -> CompileOptions {
        let mut options = CompileOptions::default();
        options.tags = true;
        options
    }

    fn settings(&self) -> Settings {
        let maps = MapSelection::Enabled(vec![Map::WorkshopChamber.into()]);
        Settings::default().with_mode(
            ModeKey::Mode(Gamemode::TeamDeathmatch),
            ModeSettings::default().maps(maps),
        )
    }
}

ow_workshop::main!(InGameChecklist);
