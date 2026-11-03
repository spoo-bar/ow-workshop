//! Sanity checks for the generated constant enums.

use ow_workshop_data::{Beam, Button, Color, CompareOp, Effect, Extension, ModifyOp, Team, Wait};

#[test]
fn team_members_are_named() {
    assert_eq!(Team::ALL, &[Team::One, Team::Two, Team::All]);
    assert_eq!(Team::One.key(), "1");
}

#[test]
fn variants_are_pascal_case() {
    assert_eq!(Button::Interact.key(), "INTERACT");
    assert_eq!(Wait::IgnoreCondition.key(), "IGNORE_CONDITION");
    assert_eq!(CompareOp::Le.key(), "<=");
    assert_eq!(ModifyOp::AppendToArray.key(), "__appendToArray__");
}

#[test]
fn colors_have_rgba_and_skip_tooling_colors() {
    assert!(Color::Red.rgba().is_some());
    assert!(Color::ALL.iter().all(|c| c.key() != "LIGHT_GRAY"));
}

#[test]
fn effects_know_their_extension() {
    assert_eq!(
        Beam::BrigitteFlailChain.extension(),
        Some(Extension::BeamEffects)
    );
    assert_eq!(Effect::BadAura.extension(), None);
}
