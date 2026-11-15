//! The harness on a hand-built mod, the owner samples, and optional Marblr files.

use ow_workshop::{Hero, Lobby, Mode, Profile, Rule, Workshop, script, utils};
use ow_workshop_golden::{Options, assert_same_script, assert_same_script_with, reference_file};

struct Small;

impl Workshop for Small {
    const NAME: &'static str = "Small";
    const DESCRIPTION: &'static str = "Excerpt";
    type Globals = ();
    type Players = ();

    fn lobby() -> Lobby {
        Lobby::new().set("Max Team 1 Players", 1)
    }
    fn modes() -> Vec<Mode> {
        vec![Mode::new("Clash").disabled()]
    }
    fn heroes() -> Vec<Hero> {
        Vec::new()
    }
    fn rules(&self, (): &(), (): &()) -> Vec<Box<dyn Rule>> {
        vec![Box::new(utils::DisableDamageAndHealing)]
    }
}

/// The same script as `Small`, spelled the way a reference file might.
const REFERENCE: &str = r#"settings
{
	lobby
	{
		Max Team 1 Players: 1
	}
	main
	{
		Description: "Excerpt"
		Mode Name: "Small"
	}
	modes
	{
		disabled Clash
		{
		}
	}
}
rule ("Disable damage and healing") {
    event {
        Ongoing - Each Player;
        All;
        All;
    }
    conditions {
        Has Spawned(Event Player) == True;
    }
    actions {
        "comment from the compiler"
        Set Damage Dealt(Event Player, 0.00);
        Set Damage Received(Event Player, 0);
        Set Healing Dealt(Event Player, 0);
        Set Healing Received(Event Player, 0);
    }
}
"#;

#[test]
fn hand_built_mod_equals_the_excerpt() {
    assert_same_script(&script(&Small, Profile::Release), REFERENCE);
}

#[test]
fn a_changed_action_fails_with_a_clear_diff() {
    let changed = REFERENCE.replace(
        "Set Healing Dealt(Event Player, 0)",
        "Set Healing Dealt(Event Player, 1)",
    );
    let result = std::panic::catch_unwind(|| {
        assert_same_script(&script(&Small, Profile::Release), &changed);
    });
    let message = *result.unwrap_err().downcast::<String>().unwrap();
    assert!(message.contains("- "), "{message}");
    assert!(
        message.contains("-         Set Healing Dealt(Event Player, 1);"),
        "{message}"
    );
    assert!(
        message.contains("+         Set Healing Dealt(Event Player, 0);"),
        "{message}"
    );
    // Unchanged lines are context, not marked as differences.
    assert!(
        message.contains("  Set Damage Received(Event Player, 0);"),
        "{message}"
    );
}

#[test]
fn main_differences_can_be_ignored() {
    let renamed = REFERENCE.replace("\"Small\"", "\"Other\"");
    let generated = script(&Small, Profile::Release);
    assert_same_script_with(&generated, &renamed, &Options::default().ignore_main());
}

fn sample(name: &str) -> String {
    let path = format!("{}/../examples/samples/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(path).unwrap()
}

#[test]
fn owner_samples_are_stable_under_normalization() {
    use ow_workshop_golden::normalize;
    for name in ["overchess.owmod", "settings_only.owmod"] {
        let once = normalize(&sample(name));
        assert_eq!(normalize(&once), once, "{name}");
    }
    assert_same_script(
        &sample("overchess.owmod"),
        &sample("overchess.owmod").replace('\n', "\r\n"),
    );
}

/// Marblr files carry no licence, so they are read from `OW_REFERENCE_DIR`.
#[test]
fn marblr_references_are_stable_under_normalization() {
    use ow_workshop_golden::normalize;
    for file in [
        "Fall Damage/FallDamage.txt",
        "Limited Ammo/LimitedAmmo.txt",
        "Limited Ammo/UTIL_AmmoPackEditor.txt",
        "Payload Skips Checkpoint/PayloadSkipsCheckpoint.txt",
        "Spawn Bots 2.0/SpawnBots2.0.txt",
        "Spawn Points/SpawnPoints.txt",
    ] {
        let Some(text) = reference_file(file) else {
            return;
        };
        let once = normalize(&text);
        assert_eq!(normalize(&once), once, "{file}");
    }
}
