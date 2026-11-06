//! Swaps of literals for zero-argument values the mod's author vouches for (size mode only).

use ow_workshop_data::ValueId as V;

use super::Env;
use super::build::{is_num, value};
use crate::ir::{Diagnostics, Expr, ExprKind, Program, Rule, StrExpr, StrToken, VarDecl, VarId};
use crate::lower::Ctx;
use crate::lower::visit::walk_block;
use crate::options::EmptyString;

/// The internal global that holds the empty string.
pub(super) const EMPTY_STRING_VAR: &str = "__emptyString__";

/// Whether `e` is `Custom String("")`.
pub(super) fn is_empty_string(e: &Expr) -> bool {
    matches!(
        e.kind(),
        ExprKind::Str(StrExpr::Custom { tokens, args })
            if args.is_empty()
                && tokens.iter().all(|t| matches!(t, StrToken::Text(s) if s.is_empty()))
    )
}

fn is_team_one(e: &Expr) -> bool {
    matches!(e.kind(), ExprKind::Const(c) if c.enum_name == "Team" && c.key == "1")
}

/// The value standing in for `0`, if one is configured.
pub(super) fn zero(e: &Expr, env: &Env<'_, '_>) -> Option<Expr> {
    let options = &env.ctx.options.replacements;
    let id = if options.zero_by_capture_percentage {
        V::GetCapturePercentage
    } else if options.zero_by_payload_progress {
        V::GetPayloadProgressPercentage
    } else if options.zero_by_is_match_complete {
        V::IsMatchComplete
    } else {
        return None;
    };
    is_num(e, 0.0).then(|| value(id, Vec::new(), e.loc()))
}

/// The value standing in for `1`, if one is configured.
pub(super) fn one(e: &Expr, env: &Env<'_, '_>) -> Option<Expr> {
    let options = &env.ctx.options.replacements;
    (options.one_by_match_round && is_num(e, 1.0))
        .then(|| value(V::GetMatchRound, Vec::new(), e.loc()))
}

/// The value standing in for `Team 1`, if one is configured.
pub(super) fn team_one(e: &Expr, env: &Env<'_, '_>) -> Option<Expr> {
    let options = &env.ctx.options.replacements;
    (options.team1_by_control_scoring_team && is_team_one(e))
        .then(|| value(V::GetControlScoringTeam, Vec::new(), e.loc()))
}

/// The configured stand-in for `""`, if any.
pub(super) fn empty_string(e: &Expr, env: &Env<'_, '_>) -> Option<Expr> {
    match env.ctx.options.replacements.empty_string? {
        EmptyString::EmptyArray => Some(Expr::new(ExprKind::Array(Vec::new()), e.loc())),
        EmptyString::Variable => env
            .empty_string
            .map(|var| Expr::new(ExprKind::GlobalVar(var), e.loc())),
    }
}

/// The literals replaced in a place that no signature flag covers, in table order.
pub(super) fn literal(e: &Expr, env: &Env<'_, '_>) -> Option<Expr> {
    zero(e, env)
        .or_else(|| one(e, env))
        .or_else(|| team_one(e, env))
}

fn mentions_empty_string(rule: &Rule) -> bool {
    fn deep(e: &Expr) -> bool {
        is_empty_string(e) || e.kind().children().into_iter().any(deep)
    }
    let mut found = rule.conditions.iter().any(|c| deep(&c.expr));
    walk_block(&rule.body, &mut |stmt| {
        found |= stmt.exprs().into_iter().any(deep);
    });
    found
}

/// Declares `__emptyString__` when the program has an empty string to replace by it.
///
/// Runs before the init rules are generated, so the variable gets its initial value.
pub(crate) fn declare_variables(
    program: &mut Program,
    ctx: &mut Ctx<'_>,
) -> Result<(), Diagnostics> {
    let wanted = ctx.options.replacements.empty_string == Some(EmptyString::Variable)
        && ctx.options.optimize != crate::options::OptLevel::Off;
    let declared = program
        .globals
        .entries
        .iter()
        .any(|d| d.name == EMPTY_STRING_VAR);
    if !wanted || declared || !program.rules.iter().any(mentions_empty_string) {
        return Ok(());
    }
    let nothing = || Expr::new(ExprKind::Array(Vec::new()), None);
    program.globals.declare(VarDecl {
        name: EMPTY_STRING_VAR.to_owned(),
        index: None,
        init: Some(value(
            V::CharAt,
            vec![nothing(), Expr::new(ExprKind::Null, None)],
            None,
        )),
        internal: true,
        flags: crate::ir::VarFlags::default(),
        location: None,
    });
    Ok(())
}

/// The id of the empty-string variable, once declared.
pub(super) fn empty_string_var(program: &Program) -> Option<VarId> {
    let at = program
        .globals
        .entries
        .iter()
        .position(|d| d.name == EMPTY_STRING_VAR)?;
    Some(VarId(u32::try_from(at).ok()?))
}

#[cfg(test)]
mod tests;
