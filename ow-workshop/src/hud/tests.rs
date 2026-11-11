use ow_workshop_data::ActionId;

use super::*;
use crate::ir::StmtKind;
use crate::opt::testutil::sh;

/// The arguments of the only `Create HUD Text` recorded, rendered.
fn hud_args(record: impl FnOnce(&mut Block)) -> Vec<String> {
    let mut block = Block::new();
    record(&mut block);
    let stmts = block.into_ir().stmts;
    assert_eq!(stmts.len(), 1);
    // `print` is a debug-only call: its HUD text is inside one `DebugOnly`.
    let stmts = match &stmts[0].kind {
        StmtKind::DebugOnly(inner) => inner.stmts.clone(),
        _ => stmts,
    };
    assert_eq!(stmts.len(), 1);
    let StmtKind::Action { id, args } = &stmts[0].kind else {
        panic!("not an action: {:?}", stmts[0].kind);
    };
    assert_eq!(*id, ActionId::HudText);
    args.iter().map(sh).collect()
}

const ALL: &str = "GetPlayers(Team.ALL)";
const LEFT: &str = "HudPosition.LEFT";
const REEVAL: &str = "HudReeval.VISIBILITY_SORT_ORDER_STRING_AND_COLOR";
const SPEC: &str = "SpecVisibility.DEFAULT";
const HI: &str = "\"Hi\"()";
const WHITE: &str = "Color.WHITE";

fn args(list: [&str; 11]) -> Vec<String> {
    list.map(str::to_owned).to_vec()
}

#[test]
fn header_fills_the_header_and_its_color() {
    assert_eq!(
        hud_args(|b| b.hud_header("Hi")),
        args([
            ALL, HI, "Null", "Null", LEFT, "0", WHITE, "Null", "Null", REEVAL, SPEC
        ])
    );
}

#[test]
fn subheader_fills_the_subheader_and_its_color() {
    assert_eq!(
        hud_args(|b| b.hud_subheader("Hi")),
        args([
            ALL, "Null", HI, "Null", LEFT, "0", "Null", WHITE, "Null", REEVAL, SPEC
        ])
    );
}

#[test]
fn subtext_fills_the_text_and_its_color() {
    assert_eq!(
        hud_args(|b| b.hud_subtext("Hi")),
        args([
            ALL, "Null", "Null", HI, LEFT, "0", "Null", "Null", WHITE, REEVAL, SPEC
        ])
    );
}

#[test]
fn optional_arguments_replace_the_defaults() {
    let opts = HudArgs {
        visible_to: Expr::from(crate::generated::api::host_player()),
        location: HudPosition::Right.into(),
        sort_order: 5.into(),
        color: Color::Red.into(),
        reevaluation: HudReeval::None.into(),
        spec_visibility: SpecVisibility::Never.into(),
    };
    assert_eq!(
        hud_args(|b| b.hud_subtext_with("Hi", opts)),
        args([
            "HostPlayer()",
            "Null",
            "Null",
            HI,
            "HudPosition.RIGHT",
            "5",
            "Null",
            "Null",
            "Color.RED",
            "HudReeval.NONE",
            "SpecVisibility.NEVER",
        ])
    );
}

#[test]
fn print_is_a_wide_orange_header_at_the_top() {
    let spaces = format!("\"{}\"()", "\u{2003}".repeat(170));
    assert_eq!(
        hud_args(|b| b.print("Hi")),
        args([
            ALL,
            HI,
            &spaces,
            "Null",
            LEFT,
            "-9999",
            "Color.ORANGE",
            "Null",
            "Null",
            "HudReeval.VISIBILITY_AND_STRING",
            SPEC,
        ])
    );
}
