//! Validations and warnings for specific functions and constants.

use crate::ir::ErrorKind;
use ow_workshop_data::signature::{Signature, is_subtype};
use ow_workshop_data::{ActionId, Beam, DynamicEffect, Effect, Extension, Gamemode, Map, ValueId};

use super::Ctx;
use super::functions::{arg_index, is_const};
use super::visit::walk_block;
use crate::ir::{Diagnostics, Expr, ExprKind, Location, Program, Rule, Stmt, StmtKind};
use crate::warning::Warning;

/// Colors that are hard to read on a HUD.
const DARK_COLORS: [&str; 3] = ["RED", "VIOLET", "PURPLE"];
/// The number a long game can exceed when used as a duration.
const NINE_THOUSAND: f64 = 9999.0;

/// The extension a constant needs, or the reason it cannot be used in Overwatch 2.
fn constant_problem(enum_name: &str, key: &str) -> Option<Result<Extension, &'static str>> {
    match enum_name {
        "Effect" => Effect::ALL
            .iter()
            .find(|e| e.key() == key)?
            .extension()
            .map(Ok),
        "Beam" => Beam::ALL
            .iter()
            .find(|e| e.key() == key)?
            .extension()
            .map(Ok),
        "DynamicEffect" => DynamicEffect::ALL
            .iter()
            .find(|e| e.key() == key)?
            .extension()
            .map(Ok),
        "Map" => Map::ALL
            .iter()
            .find(|m| m.key() == key)?
            .only_in_ow1()
            .then_some(Err("map")),
        "Gamemode" => Gamemode::ALL
            .iter()
            .find(|g| g.key() == key)?
            .only_in_ow1()
            .then_some(Err("gamemode")),
        _ => None,
    }
}

/// Whether `expr` can never be a boolean, as far as the untyped IR shows.
fn is_never_bool(expr: &Expr) -> bool {
    match expr.kind() {
        ExprKind::Num(_)
        | ExprKind::Str(_)
        | ExprKind::Vector(_)
        | ExprKind::Array(_)
        | ExprKind::Add(..)
        | ExprKind::Sub(..)
        | ExprKind::Mul(..)
        | ExprKind::Div(..)
        | ExprKind::Mod(..)
        | ExprKind::Pow(..)
        | ExprKind::Neg(_) => true,
        ExprKind::Value { id, .. } => {
            let sig: &Signature = id.signature();
            // Numbers are accepted where a boolean is expected, which is exactly the trap.
            is_subtype(&sig.ret, &ow_workshop_data::Ty::Float)
        }
        _ => false,
    }
}

struct Checker<'a, 'b> {
    ctx: &'a mut Ctx<'b>,
    program_has_extension: &'a dyn Fn(Extension) -> bool,
    rule: &'a Rule,
    errors: Diagnostics,
}

impl Checker<'_, '_> {
    fn warn(&mut self, warning: Warning, message: String, loc: Option<Location>) {
        self.ctx.warn(&self.rule.suppressed, warning, message, loc);
    }

    fn expr(&mut self, expr: &Expr, stmt_loc: Option<Location>) {
        let loc = expr.loc().or(stmt_loc);
        match expr.kind() {
            ExprKind::Const(c) => match constant_problem(c.enum_name, c.key) {
                Some(Ok(extension)) if !(self.program_has_extension)(extension) => {
                    self.errors.error(ErrorKind::ExtensionNotEnabled,
                        format!(
                            "`{}` needs the `{extension:?}` extension, which is not enabled in the settings",
                            c.key
                        ),
                        loc,
                    );
                }
                Some(Err(what)) => self.errors.error(
                    ErrorKind::Ow1Only,
                    format!("the {what} `{}` only exists in Overwatch 1", c.key),
                    loc,
                ),
                _ => {}
            },
            ExprKind::Value { id, .. } => {
                let found = match id {
                    ValueId::GetClosestPlayer => Some((
                        Warning::ClosestPlayer,
                        "`Closest Player To` also finds dead and unspawned players at the origin; use the \"real\" variant from the standard library",
                    )),
                    ValueId::GetFarthestPlayer => Some((
                        Warning::FarthestPlayer,
                        "`Farthest Player From` also finds dead and unspawned players at the origin; use the \"real\" variant from the standard library",
                    )),
                    ValueId::GetPlayerClosestToReticle => Some((
                        Warning::PlayerClosestToReticle,
                        "`Player Closest To Reticle` also finds dead and unspawned players at the origin; use the \"real\" variant from the standard library",
                    )),
                    _ => None,
                };
                if let Some((warning, message)) = found {
                    self.warn(warning, message.to_owned(), loc);
                }
            }
            _ => {}
        }
        for child in expr.kind().children() {
            self.expr(child, stmt_loc);
        }
    }

    fn action(&mut self, id: ActionId, args: &[Expr], loc: Option<Location>) {
        let sig = id.signature();
        let arg = |name: &str| arg_index(sig, name).and_then(|i| args.get(i));
        let is_9999 =
            |e: Option<&Expr>| e.is_some_and(|e| e.kind() == &ExprKind::Num(NINE_THOUSAND));
        match id {
            ActionId::Wait if is_9999(arg("time")) => self.warn(
                Warning::Wait9999,
                "Wait(9999) ends before a long game does (up to 16200 s); use infinity or 99999"
                    .to_owned(),
                loc,
            ),
            ActionId::WaitUntil => {
                if is_9999(arg("timeout")) {
                    self.warn(
                        Warning::Wait9999,
                        "Wait Until(…, 9999) times out before a long game ends (up to 16200 s); use infinity or 99999".to_owned(),
                        loc,
                    );
                }
                if arg("continueCondition").is_some_and(is_never_bool) {
                    self.warn(
                        Warning::WaitUntil,
                        "Wait Until treats non-zero numbers as false; pass a boolean condition"
                            .to_owned(),
                        loc,
                    );
                }
            }
            ActionId::SetStatusEffect
            | ActionId::StartDamageOverTime
            | ActionId::StartHealingOverTime
                if is_9999(arg("duration")) =>
            {
                self.warn(
                    Warning::Duration9999,
                    "A duration of 9999 ends before a long game does (up to 16200 s); use infinity or 99999".to_owned(),
                    loc,
                );
            }
            ActionId::ApplyImpulse
                if arg("motion")
                    .is_some_and(|m| is_const(m, "Impulse", "CANCEL_CONTRARY_MOTION")) =>
            {
                self.warn(
                    Warning::CancelContraryMotion,
                    "Cancel Contrary Motion is legacy; use Cancel Contrary Motion XYZ".to_owned(),
                    loc,
                );
            }
            ActionId::HudText
            | ActionId::CreateInWorldText
            | ActionId::ProgressBarHud
            | ActionId::CreateProgressBarInWorldText => {
                for name in ["color", "headerColor", "subheaderColor", "textColor"] {
                    let Some(color) = arg(name) else { continue };
                    if DARK_COLORS.iter().any(|k| is_const(color, "Color", k)) {
                        self.warn(
                            Warning::DarkColor,
                            "This text color is hard to read; use the Light variant of it"
                                .to_owned(),
                            color.loc().or(loc),
                        );
                    }
                }
            }
            _ => {}
        }
    }

    fn stmt(&mut self, stmt: &Stmt) {
        if let StmtKind::Action { id, args } = &stmt.kind {
            self.action(*id, args, stmt.loc);
        }
        for e in stmt.exprs() {
            self.expr(e, stmt.loc);
        }
    }
}

/// Reports unusable constants (errors) and risky calls (warnings).
pub(crate) fn check(program: &Program, ctx: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    let has = |extension| program.settings.has_extension(extension);
    let mut errors = Diagnostics::default();
    for rule in &program.rules {
        let mut checker = Checker {
            ctx,
            program_has_extension: &has,
            rule,
            errors: Diagnostics::default(),
        };
        for cond in &rule.conditions {
            checker.expr(&cond.expr, rule.location);
        }
        walk_block(&rule.body, &mut |stmt: &Stmt| checker.stmt(stmt));
        errors.0.append(&mut checker.errors.0);
    }
    if errors.has_errors() {
        return Err(errors);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
