//! Sanity checks for the generated constant enums.

use ow_workshop_data::{Beam, Color, Effect, Extension};

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
