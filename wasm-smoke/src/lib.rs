//! Exports `compile(lang) -> String` for the `hello_world` mod. CI builds it for
//! `wasm32-unknown-unknown`, runs `wasm-opt -Oz` and reports the size.

use ow_workshop::prelude::*;
use ow_workshop::{Gamemode, Map};
use wasm_bindgen::prelude::wasm_bindgen;

/// The `hello_world` example's mod.
struct HelloWorld;

impl Workshop for HelloWorld {
    fn mode_name(&self) -> &'static str {
        "My Mode"
    }

    fn description(&self) -> &'static str {
        "Made with ow-workshop"
    }

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
        Settings::default().with_mode(
            ModeKey::Mode(Gamemode::TeamDeathmatch),
            ModeSettings::default().maps(maps),
        )
    }
}

/// Compiles the mod for the language tag `lang` (for example `en-US`); errors come back as text.
///
/// # Errors
/// An unknown language, or a mod that does not compile.
#[wasm_bindgen]
pub fn compile(lang: &str) -> Result<String, String> {
    let lang: Lang = lang.parse().map_err(|e| format!("{e}"))?;
    HelloWorld
        .compile(lang)
        .map(|output| output.code)
        .map_err(|diagnostics| diagnostics.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles_hello_world() {
        let code = compile("en-US").expect("compiles");
        assert!(code.contains("Hello, Workshop!"), "{code}");
        assert!(compile("xx-XX").is_err());
    }
}
