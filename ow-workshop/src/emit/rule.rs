//! Rules: header, event, conditions and actions.

use core::fmt::{self, Write};

use ow_workshop_data::{ActionId, Keyword, Lang};

use super::expr::{operator, truth, write_expr};
use super::stmt::{write_for, write_stmt};
use super::{Cx, indent, keyword, write_censored_string, write_string};
use crate::flatten::{FlatKind, FlatStmt, OptState, flatten_rule};
use crate::ir::{Event, Expr, ExprKind, PlayerFilter, Rule};
use ow_workshop_data::Ty;

/// Writes every rule, each followed by a blank line.
pub(crate) fn write_rules(out: &mut impl Write, cx: Cx<'_>) -> fmt::Result {
    let mut printed = OptState::DEFAULT;
    for rule in &cx.program.rules {
        write_rule(out, cx, rule, &mut printed)?;
    }
    Ok(())
}

/// Writes one rule. `printed` is the optimizer state the output last announced.
fn write_rule(
    out: &mut impl Write,
    cx: Cx<'_>,
    rule: &Rule,
    printed: &mut OptState,
) -> fmt::Result {
    let lines = flatten_rule(rule, cx.options);
    let state = OptState::from_options(cx.options).enter(rule.body.opt);
    write_markers(out, printed, state, 0)?;

    if rule.disabled {
        write!(out, "{} ", word(cx, Keyword::Disabled))?;
    }
    write!(out, "{} (", word(cx, Keyword::Rule))?;
    write_censored_string(out, &rule.name)?;
    writeln!(out, ") {{")?;

    write_event(out, cx, rule)?;
    if !rule.conditions.is_empty() {
        indent(out, 1)?;
        writeln!(out, "{} {{", word(cx, Keyword::Conditions))?;
        for cond in &rule.conditions {
            if let Some(comment) = &cond.comment {
                indent(out, 2)?;
                write_string(out, comment)?;
                out.write_char('\n')?;
            }
            indent(out, 2)?;
            write_condition(out, cx, &cond.expr)?;
            out.write_str(";\n")?;
        }
        indent(out, 1)?;
        out.write_str("}\n")?;
    }
    if !lines.is_empty() {
        indent(out, 1)?;
        writeln!(out, "{} {{", word(cx, Keyword::Actions))?;
        for line in &lines {
            write_line(out, cx, line, printed)?;
        }
        indent(out, 1)?;
        out.write_str("}\n")?;
    }
    out.write_str("}\n\n")
}

fn word(cx: Cx<'_>, keyword: Keyword) -> &'static str {
    keyword::word(keyword, cx.lang)
}

fn write_event(out: &mut impl Write, cx: Cx<'_>, rule: &Rule) -> fmt::Result {
    indent(out, 1)?;
    writeln!(out, "{} {{", word(cx, Keyword::Event))?;
    indent(out, 2)?;
    writeln!(out, "{};", word(cx, keyword::event(rule.event)))?;
    match rule.event {
        Event::OngoingGlobal => {}
        Event::Subroutine(id) => {
            let name = cx
                .program
                .subroutines
                .entries
                .get(id.0 as usize)
                .map_or("", |decl| decl.name.as_str());
            indent(out, 2)?;
            writeln!(out, "{name};")?;
        }
        _ => {
            let PlayerFilter { team, player } = rule.filter.unwrap_or_default();
            indent(out, 2)?;
            writeln!(out, "{};", word(cx, keyword::team(team)))?;
            indent(out, 2)?;
            let name = keyword::player(player).map_or("", |k| word(cx, k));
            writeln!(out, "{name};")?;
        }
    }
    indent(out, 1)?;
    out.write_str("}\n")
}

/// Whether the expression is a boolean as far as the game can tell.
fn is_boolean(expr: &Expr) -> bool {
    match expr.kind() {
        ExprKind::Bool(_)
        | ExprKind::Compare { .. }
        | ExprKind::And(..)
        | ExprKind::Or(..)
        | ExprKind::Not(_) => true,
        ExprKind::Value { id, .. } => id.signature().ret == Ty::Bool,
        _ => false,
    }
}

/// Writes a condition in the `lhs op rhs` form the game requires.
fn write_condition(out: &mut impl Write, cx: Cx<'_>, expr: &Expr) -> fmt::Result {
    match expr.kind() {
        ExprKind::Compare { op, lhs, rhs } => {
            write_expr(out, cx, lhs)?;
            write!(out, " {} ", operator(*op))?;
            write_expr(out, cx, rhs)
        }
        ExprKind::Not(inner) => {
            write_expr(out, cx, inner)?;
            write!(out, " == {}", truth(cx, false))
        }
        _ if is_boolean(expr) => {
            write_expr(out, cx, expr)?;
            write!(out, " == {}", truth(cx, true))
        }
        _ => {
            write_expr(out, cx, expr)?;
            write!(out, " != {}", truth(cx, false))
        }
    }
}

/// Writes one comment line per optimizer setting that differs between `printed` and `state`.
fn write_markers(
    out: &mut impl Write,
    printed: &mut OptState,
    state: OptState,
    depth: usize,
) -> fmt::Result {
    let state_word = |on: bool| if on { "enabled" } else { "disabled" };
    let changes = [
        (
            printed.enabled != state.enabled,
            "Optimizations",
            state.enabled,
        ),
        (printed.size != state.size, "Optimize for size", state.size),
        (
            printed.strict != state.strict,
            "Strict optimizations",
            state.strict,
        ),
    ];
    for (changed, name, on) in changes {
        if changed {
            indent(out, depth)?;
            writeln!(out, "//{name} {}", state_word(on))?;
        }
    }
    *printed = state;
    Ok(())
}

/// Writes one action line, preceded by its comment and any optimizer markers.
fn write_line(
    out: &mut impl Write,
    cx: Cx<'_>,
    line: &FlatStmt<'_>,
    printed: &mut OptState,
) -> fmt::Result {
    let depth = line.depth + 2;
    write_markers(out, printed, line.opt, depth)?;
    if let Some(comment) = line.comment {
        indent(out, depth)?;
        write_string(out, comment)?;
        out.write_char('\n')?;
    }
    indent(out, depth)?;
    let name = |id: ActionId| id.name(cx.lang);
    match &line.kind {
        FlatKind::Label(id) => return writeln!(out, "//label_{}:", id.0),
        FlatKind::Comment(text) => {
            write_string(out, text)?;
            return out.write_char('\n');
        }
        FlatKind::Stmt(stmt) => write_stmt(out, cx, stmt)?,
        FlatKind::If(cond) => write_header(out, cx, name(ActionId::If), cond)?,
        FlatKind::ElseIf(cond) => write_header(out, cx, name(ActionId::Elif), cond)?,
        FlatKind::While(cond) => write_header(out, cx, name(ActionId::While), cond)?,
        FlatKind::Else => out.write_str(name(ActionId::Else))?,
        FlatKind::End => out.write_str(name(ActionId::End))?,
        FlatKind::DisabledAbort => write!(
            out,
            "{} {}",
            word(cx, Keyword::Disabled),
            name(ActionId::Return)
        )?,
        FlatKind::For {
            var,
            start,
            stop,
            step,
        } => write_for(out, cx, var, [start, stop, step])?,
    }
    out.write_str(";\n")
}

fn write_header(out: &mut impl Write, cx: Cx<'_>, name: &str, cond: &Expr) -> fmt::Result {
    write!(out, "{name}(")?;
    write_expr(out, cx, cond)?;
    out.write_char(')')
}

#[cfg(test)]
mod tests;
