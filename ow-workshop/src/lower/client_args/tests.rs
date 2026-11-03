use ow_workshop_data::ActionId;

use super::*;
use crate::lower::testutil::*;

fn names(action: ActionId, given: &[(&str, Expr)]) -> Vec<&'static str> {
    let crate::ir::StmtKind::Action { args, .. } = act(action, given.to_vec()) else {
        unreachable!()
    };
    let sig = action.signature();
    client_side_args(action, &args)
        .iter()
        .map(|i| sig.args[i].name)
        .collect()
}

fn hud(reeval: &'static str) -> Vec<&'static str> {
    names(
        ActionId::HudText,
        &[("reevaluation", konst("HudReeval", reeval))],
    )
}

#[test]
fn hud_text_with_all_reevaluated() {
    assert_eq!(
        hud("VISIBILITY_SORT_ORDER_STRING_AND_COLOR"),
        [
            "visibleTo",
            "header",
            "subheader",
            "text",
            "sortOrder",
            "headerColor",
            "subheaderColor",
            "textColor"
        ]
    );
}

#[test]
fn hud_text_with_some_reevaluated() {
    assert_eq!(hud("STRING"), ["header", "subheader", "text"]);
    assert_eq!(hud("VISIBILITY_AND_SORT_ORDER"), ["visibleTo", "sortOrder"]);
}

#[test]
fn hud_text_with_nothing_reevaluated() {
    assert_eq!(hud("NONE"), [] as [&str; 0]);
}

#[test]
fn big_and_small_messages_and_the_inspector_log_are_always_client_side() {
    for action in [
        ActionId::BigMessage,
        ActionId::SmallMessage,
        ActionId::LogToInspector,
    ] {
        assert_eq!(names(action, &[]), ["text"]);
    }
}

#[test]
fn names_the_action_lacks_are_ignored() {
    // Create Effect has no text, sort order or header.
    assert_eq!(
        names(
            ActionId::CreateEffect,
            &[(
                "reevaluation",
                konst("EffectReeval", "VISIBILITY_POSITION_RADIUS_AND_COLOR")
            )]
        ),
        ["visibleTo", "color", "position", "radius"]
    );
}

#[test]
fn values_cover_the_progress_bar_value_and_text() {
    assert_eq!(
        names(
            ActionId::ProgressBarHud,
            &[(
                "reevaluation",
                konst("ProgressHudReeval", "VALUES_AND_COLOR")
            )]
        ),
        ["value", "text", "progressBarColor", "textColor"]
    );
}

#[test]
fn direction_turn_rate_magnitude_size_and_friendliness() {
    assert_eq!(
        names(
            ActionId::StartFacing,
            &[(
                "reevaluation",
                konst("FacingReeval", "DIRECTION_AND_TURN_RATE")
            )]
        ),
        ["direction", "turnRate"]
    );
    assert_eq!(
        names(
            ActionId::StartThrottleInDirection,
            &[(
                "reevaluation",
                konst("ThrottleReeval", "DIRECTION_AND_MAGNITUDE")
            )]
        ),
        ["direction", "magnitude"]
    );
    assert_eq!(
        names(
            ActionId::CreateProjectileEffect,
            &[(
                "reevaluation",
                konst(
                    "ProjectileEffectReeval",
                    "VISIBILITY_FRIENDLINESS_POSITION_DIRECTION_AND_SIZE"
                )
            )]
        ),
        [
            "visibleTo",
            "friendlyTo",
            "position",
            "direction",
            "oversize"
        ]
    );
}

#[test]
fn other_actions_and_unknown_reevaluations_have_no_client_side_arguments() {
    assert_eq!(names(ActionId::Teleport, &[]), [] as [&str; 0]);
    // Not a constant: nothing can be assumed.
    assert_eq!(
        names(ActionId::HudText, &[("reevaluation", num(1.0))]),
        [] as [&str; 0]
    );
    // A constant of the wrong enum is not a reevaluation.
    assert_eq!(
        names(
            ActionId::HudText,
            &[("reevaluation", konst("Color", "RED"))]
        ),
        [] as [&str; 0]
    );
}

#[test]
fn tokens_split_two_word_names() {
    assert_eq!(
        tokens("SORT_ORDER_STRING_AND_COLOR"),
        ["SORT_ORDER", "STRING", "COLOR"]
    );
    assert_eq!(
        tokens("DIRECTION_AND_TURN_RATE"),
        ["DIRECTION", "TURN_RATE"]
    );
    assert_eq!(tokens("NONE"), ["NONE"]);
}

#[test]
fn the_bit_set_holds_positions() {
    let set: SmallBitSet = [3, 1, 3].into_iter().collect();
    assert!(set.contains(1) && set.contains(3) && !set.contains(2));
    assert_eq!(set.iter().collect::<Vec<_>>(), [1, 3]);
    assert!(SmallBitSet::default().is_empty());
}
