//! The settings-only mod matches the owner sample, ignoring the `main` block.

use examples::settings_only::SettingsOnly;
use ow_workshop::{Profile, script};
use ow_workshop_golden::{Options, assert_same_script_with};

#[test]
fn matches_the_sample() {
    let reference = include_str!("../samples/settings_only.owmod");
    assert_same_script_with(
        &script(&SettingsOnly, Profile::Release),
        reference,
        &Options::default().ignore_main(),
    );
}
