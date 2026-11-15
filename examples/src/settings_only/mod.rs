//! A mod with settings only and no rules.

use ow_workshop::{Hero, Lobby, Mode, Rule, Workshop};

#[derive(Default)]
pub struct SettingsOnly;

impl Workshop for SettingsOnly {
    const NAME: &'static str = "Settings Only";
    const DESCRIPTION: &'static str = "Lobby, mode and hero settings without rules";

    type Globals = ();
    type Players = ();

    fn lobby() -> Lobby {
        Lobby::new()
            .set("Data Center Preference", "Netherlands")
            .set("Map Rotation", "After A Game")
            .set("Match Voice Chat", "Enabled")
            .set("Pause Game On Player Disconnect", "Yes")
            .set("Return To Lobby", "After A Game")
            .set("Team Balancing", "After A Game")
    }

    fn modes() -> Vec<Mode> {
        vec![
            Mode::new("Capture the Flag")
                .disabled()
                .set("Damage Interrupts Flag Interaction", "Enabled")
                .set("Enable Perks", "On")
                .set("Flag Dropped Lock Time", "5.0")
                .set("Flag Return Time", "4.0")
                .set("Flag Score Respawn Time", "15.0")
                .set("Perk Elimination Catchup Level Amount", "33%")
                .set("Perk Generation", "175%")
                .set("Team Needs Flag At Base To Score", "Yes"),
            Mode::new("Clash")
                .set("Capture Speed Modifier", "45%")
                .set("Perk Elimination Catchup Level Amount", "46%")
                .set("Perk Generation", "249%"),
            Mode::new("Control")
                .set("Perk Elimination Catchup Level Amount", "47%")
                .set("Perk Generation", "248%"),
            Mode::new("Push")
                .set("Perk Elimination Catchup Level Amount", "47%")
                .set("Perk Generation", "242%")
                .set("TS-1 Walk Speed Modifier", "101%"),
            Mode::new("General")
                .set("Health Pack Respawn Time Scalar", "49%")
                .set("Hero Limit", "2 Per Team")
                .set("Limit Roles", "2 Of Each Role Per Team")
                .set("Spawn Health Packs", "Enabled")
                .set("Team Overlay", "Enabled"),
        ]
    }

    fn heroes() -> Vec<Hero> {
        vec![
            Hero::general()
                .set("Ability Cooldown Time", "101%")
                .set("Ammunition Clip Size Scalar", "106%")
                .set("Ultimate Duration", "252%")
                .set("Ultimate Generation", "215%")
                .set("Ultimate Generation - Combat", "216%")
                .set("Ultimate Generation - Passive", "208%"),
        ]
    }

    fn rules(&self, (): &(), (): &()) -> Vec<Box<dyn Rule>> {
        Vec::new()
    }
}
