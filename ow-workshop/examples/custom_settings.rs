//! Settings, workshop settings and translations: `cargo run -p ow-workshop --example custom_settings`.
//!
//! Also rewrites `examples/data/custom_settings.fr.po` with the strings the mod uses.
#![cfg_attr(target_arch = "wasm32", no_main)]

use ow_workshop::TranslationLang::{En, Fr};
use ow_workshop::prelude::*;
use ow_workshop::{Catalog, Extension, Gamemode, Map};

const FRENCH: &str = include_str!("data/custom_settings.fr.po");

pub struct CustomSettings;

impl Workshop for CustomSettings {
    type Globals = ();
    type Players = ();

    fn build(&self, ws: &mut Builder, (): &(), (): &()) {
        let hard_mode = workshop_setting::toggle("Gameplay", "Hard mode", false);
        let rounds: Expr<Num> = workshop_setting::int("Gameplay", "Rounds", 5)
            .range(1..=20)
            .into();
        let difficulty: Expr<Num> =
            workshop_setting::combo("Gameplay", "Difficulty", ["Easy", "Normal", "Hard"], 1).into();

        ws.rule("Round banner", OngoingGlobal, |r, _| {
            r.actions(|b| b.hud_header(t!("Round {} of {}", match_round(), rounds)));
        });

        ws.rule("Hard mode", OngoingEachPlayer, |r, ev| {
            r.condition(hard_mode);
            r.condition(difficulty.ge(2));
            r.actions(|b| {
                b.hud_subheader_with(
                    t!("Hard mode is on"),
                    HudArgs {
                        visible_to: ev.event_player().into(),
                        ..HudArgs::default()
                    },
                );
                b.set_max_health(ev.event_player(), 150);
            });
        });
    }

    fn settings(&self) -> Settings {
        let reaper = Values::new().with("ability1Cooldown%", SettingValue::Percent(50.0));
        Settings::default()
            .with_main(Main {
                mode_name: "Custom Settings".to_owned(),
                description: "Shows settings, workshop settings and translations".to_owned(),
            })
            .with_lobby(Lobby::default().team1_slots(4).team2_slots(0))
            .with_mode(
                ModeKey::Mode(Gamemode::TeamDeathmatch),
                ModeSettings::default().maps(MapSelection::Enabled(
                    [Map::WorkshopChamber, Map::WorkshopIsland]
                        .map(MapRef::new)
                        .to_vec(),
                )),
            )
            .with_heroes(
                HeroTeam::AllTeams,
                HeroSettings::default()
                    .selection(HeroSelection::Disabled(vec![Hero::Bastion]))
                    .hero(Hero::Reaper, reaper),
            )
            .with_extension(Extension::BeamEffects)
    }

    fn translations(&self) -> Option<Translations> {
        let mut catalog = Catalog::default();
        catalog.merge_po(Fr, FRENCH).ok()?;
        Some(Translations::new([En, Fr]).with_catalog(catalog))
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let lang = match ow_workshop::lang_from_args(std::env::args().skip(1)) {
        Ok(lang) => lang,
        Err(message) => {
            eprintln!("error: {message}");
            std::process::exit(2);
        }
    };
    match CustomSettings.compile(lang) {
        Ok(output) => {
            print!("{}", output.code);
            if let Some(catalog) = output.catalog {
                let path = concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/examples/data/custom_settings.fr.po"
                );
                if let Err(error) = std::fs::write(path, catalog.to_po(Fr, false)) {
                    eprintln!("error: cannot write {path}: {error}");
                    std::process::exit(1);
                }
            }
        }
        Err(diagnostics) => {
            for diagnostic in &diagnostics.0 {
                eprintln!("{diagnostic}");
            }
            std::process::exit(1);
        }
    }
}
