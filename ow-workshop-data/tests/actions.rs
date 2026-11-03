//! Tests for action signatures.

use ow_workshop_data::signature::DefaultValue;
use ow_workshop_data::{ActionId, Ty, ValueId};

#[test]
fn all_actions_are_present() {
    assert_eq!(ActionId::ALL.len(), 225);
    assert_eq!(ActionId::Teleport.key(), "teleport");
}

#[test]
fn teleport_is_a_member_action() {
    let sig = ActionId::Teleport.signature();
    assert!(sig.flags.member());
    assert_eq!(sig.ret, Ty::Void);
    assert_eq!(sig.args[0].name, "player");
    assert_eq!(sig.args[1].ty, Ty::Position);
    assert!(sig.args[1].replace.zero_by_null());
}

#[test]
fn hud_text_defaults_to_all_players() {
    let sig = ActionId::HudText.signature();
    assert_eq!(sig.args.len(), 11);
    assert_eq!(
        sig.args[0].default,
        Some(DefaultValue::Value(
            ValueId::GetPlayers,
            &[DefaultValue::Const("Team", "ALL")]
        ))
    );
    assert!(sig.args[3].replace.empty_string_by_empty_array());
    assert_eq!(
        sig.args[4].default,
        Some(DefaultValue::Const("HudPosition", "LEFT"))
    );
}

#[test]
fn wait_has_a_default_time_and_behavior() {
    let sig = ActionId::Wait.signature();
    assert_eq!(sig.args[0].ty, Ty::UnsignedFloat);
    assert_eq!(sig.args[0].default, Some(DefaultValue::Num(0.016)));
    assert!(sig.args[0].replace.zero_by_false() && sig.args[0].replace.one_by_true());
    assert_eq!(
        sig.args[1].default,
        Some(DefaultValue::Const("Wait", "IGNORE_CONDITION"))
    );
}
