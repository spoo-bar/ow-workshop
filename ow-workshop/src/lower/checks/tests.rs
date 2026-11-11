use ow_workshop_data::{ActionId, Beam, Effect, Extension, Gamemode, Map, ValueId};

use super::*;
use crate::lower::testutil::*;
use crate::options::OptLevel;
use crate::settings::Settings;

fn check_body(body: Vec<StmtKind>) -> Result<Diagnostics, Diagnostics> {
    check_with(body, Settings::default())
}

fn check_with(body: Vec<StmtKind>, settings: Settings) -> Result<Diagnostics, Diagnostics> {
    let mut p = program(body);
    p.settings = settings;
    let options = options(OptLevel::Size);
    let mut diags = Diagnostics::default();
    let mut ctx = Ctx::new(&options, ow_workshop_data::Lang::EnUs, &mut diags, &p);
    check(&p, &mut ctx)?;
    Ok(diags)
}

fn warned(body: Vec<StmtKind>) -> Vec<&'static str> {
    codes(&check_body(body).expect("no errors"))
}

fn teleport(e: Expr) -> StmtKind {
    act(ActionId::Teleport, vec![("player", e)])
}

#[test]
fn extension_constants_need_the_extension() {
    let effect = Effect::ALL
        .iter()
        .find(|e| e.extension().is_some())
        .copied()
        .expect("an extension effect");
    let ext = effect.extension().expect("extension");
    let body = vec![teleport(konst("Effect", effect.key()))];
    let err = check_body(body.clone()).expect_err("not enabled");
    assert_eq!(codes(&err), ["extension-not-enabled"]);
    assert!(err.0[0].message.contains(effect.key()));
    // Enabled in the settings: fine.
    check_with(body, Settings::default().with_extension(ext)).expect("enabled");
}

#[test]
fn beams_need_their_extension_too() {
    let beam = Beam::ALL
        .iter()
        .find(|b| b.extension().is_some())
        .copied()
        .expect("an extension beam");
    let err = check_body(vec![teleport(konst("Beam", beam.key()))]).expect_err("not enabled");
    assert_eq!(codes(&err), ["extension-not-enabled"]);
    // A different extension does not help.
    let other = Extension::ALL
        .iter()
        .find(|e| Some(**e) != beam.extension())
        .copied()
        .expect("another extension");
    assert!(
        check_with(
            vec![teleport(konst("Beam", beam.key()))],
            Settings::default().with_extension(other)
        )
        .is_err()
    );
}

#[test]
fn constants_without_an_extension_are_fine() {
    check_body(vec![teleport(konst("Effect", "no-such-effect"))]).expect("unknown keys pass");
}

#[test]
fn overwatch_1_maps_and_gamemodes_are_errors() {
    let map = Map::ALL
        .iter()
        .find(|m| m.only_in_ow1())
        .copied()
        .expect("an OW1 map");
    let err = check_body(vec![teleport(konst("Map", map.key()))]).expect_err("ow1 map");
    assert_eq!(codes(&err), ["ow1-only"]);
    let mode = Gamemode::ALL
        .iter()
        .find(|g| g.only_in_ow1())
        .copied()
        .expect("an OW1 mode");
    let err = check_body(vec![teleport(konst("Gamemode", mode.key()))]).expect_err("ow1 mode");
    assert_eq!(codes(&err), ["ow1-only"]);
    let fine = Map::ALL
        .iter()
        .find(|m| !m.only_in_ow1())
        .copied()
        .expect("an OW2 map");
    check_body(vec![teleport(konst("Map", fine.key()))]).expect("ow2 map");
}

#[test]
fn closest_and_farthest_players() {
    for (id, code) in [
        (ValueId::GetClosestPlayer, "closest-player"),
        (ValueId::GetFarthestPlayer, "farthest-player"),
        (
            ValueId::GetPlayerClosestToReticle,
            "player-closest-to-reticle",
        ),
    ] {
        assert_eq!(warned(vec![teleport(call(id, vec![]))]), [code]);
    }
}

#[test]
fn dark_text_colors() {
    for color in ["RED", "VIOLET", "PURPLE"] {
        let hud = act(
            ActionId::HudText,
            vec![("textColor", konst("Color", color))],
        );
        assert_eq!(warned(vec![hud]), ["dark-color"]);
    }
    let world = act(
        ActionId::CreateInWorldText,
        vec![("color", konst("Color", "RED"))],
    );
    assert_eq!(warned(vec![world]), ["dark-color"]);
    let bar = act(
        ActionId::ProgressBarHud,
        vec![("textColor", konst("Color", "PURPLE"))],
    );
    assert_eq!(warned(vec![bar]), ["dark-color"]);
    let fine = act(
        ActionId::HudText,
        vec![("textColor", konst("Color", "GREEN"))],
    );
    assert_eq!(warned(vec![fine]), [] as [&str; 0]);
}

#[test]
fn waits_and_durations_of_9999() {
    let timed = |id, name, value| warned(vec![act(id, vec![(name, num(value))])]);
    assert_eq!(timed(ActionId::Wait, "time", 9999.0), ["wait-9999"]);
    assert_eq!(timed(ActionId::Wait, "time", 5.0), [] as [&str; 0]);
    assert_eq!(
        warned(vec![act(
            ActionId::WaitUntil,
            vec![
                ("continueCondition", boolean(true)),
                ("timeout", num(9999.0))
            ]
        )]),
        ["wait-9999"]
    );
    for id in [
        ActionId::SetStatusEffect,
        ActionId::StartDamageOverTime,
        ActionId::StartHealingOverTime,
    ] {
        assert_eq!(timed(id, "duration", 9999.0), ["duration-9999"]);
    }
}

#[test]
fn wait_until_needs_a_boolean() {
    let wait_until = |cond: Expr| act(ActionId::WaitUntil, vec![("continueCondition", cond)]);
    assert_eq!(warned(vec![wait_until(num(1.0))]), ["wait-until"]);
    assert_eq!(
        warned(vec![wait_until(expr(ExprKind::Add(num(1.0), num(2.0))))]),
        ["wait-until"]
    );
    assert_eq!(
        warned(vec![wait_until(call(ValueId::GetHealth, vec![num(1.0)]))]),
        ["wait-until"]
    );
    for ok in [
        boolean(true),
        call(ValueId::IsAlive, vec![num(1.0)]),
        expr(ExprKind::Not(boolean(false))),
        expr(ExprKind::GlobalVar(crate::ir::VarId(0))),
    ] {
        assert_eq!(warned(vec![wait_until(ok)]), [] as [&str; 0]);
    }
}

#[test]
fn legacy_impulse() {
    let impulse = |key: &'static str| {
        act(
            ActionId::ApplyImpulse,
            vec![("motion", konst("Impulse", key))],
        )
    };
    assert_eq!(
        warned(vec![impulse("CANCEL_CONTRARY_MOTION")]),
        ["cancel-contrary-motion"]
    );
    assert_eq!(
        warned(vec![impulse("CANCEL_CONTRARY_MOTION_XYZ")]),
        [] as [&str; 0]
    );
}

#[test]
fn checks_reach_nested_blocks_conditions_and_arguments() {
    let nested = if_(
        boolean(true),
        vec![teleport(expr(ExprKind::Add(
            num(1.0),
            call(ValueId::GetClosestPlayer, vec![]),
        )))],
    );
    assert_eq!(warned(vec![nested]), ["closest-player"]);
}

#[test]
fn warnings_can_be_suppressed() {
    let mut p = program(vec![act(ActionId::Wait, vec![("time", num(9999.0))])]);
    p.rules[0].suppressed.insert(Warning::Wait9999);
    locate(&mut p);
    let options = options(OptLevel::Size);
    let mut diags = Diagnostics::default();
    let mut ctx = Ctx::new(&options, ow_workshop_data::Lang::EnUs, &mut diags, &p);
    check(&p, &mut ctx).expect("checks");
    assert_eq!(diags.0.len(), 0);
}
