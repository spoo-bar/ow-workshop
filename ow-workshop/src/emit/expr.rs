//! Expression printing.

use core::fmt::{self, Write};

use ow_workshop_data::{Keyword, ValueId};

use super::constant::constant_name;
use super::string::{write_escaped, write_string};
use super::{Cx, number::write_number};
use crate::ir::{CmpOp, ConstRef, Expr, ExprKind, StrExpr, StrToken, VarId};

/// Writes an expression.
pub(super) fn write_expr(out: &mut impl Write, cx: Cx<'_>, expr: &Expr) -> fmt::Result {
    use ExprKind as K;
    match expr.kind() {
        K::Num(n) => write_number(out, *n),
        K::Bool(b) => out.write_str(truth(cx, *b)),
        K::Null => out.write_str(ValueId::Null.name(cx.lang)),
        K::Const(c) => write_const(out, cx, *c),
        K::Str(StrExpr::Custom { tokens, args }) => write_custom_string(out, cx, tokens, args),
        K::Str(StrExpr::Localized(text)) => {
            out.write_str(ValueId::LocalizedString.name(cx.lang))?;
            out.write_char('(')?;
            write_string(out, text.text(cx.lang))?;
            out.write_char(')')
        }
        K::Vector([x, y, z]) => write_call(out, cx, ValueId::Vect.name(cx.lang), &[x, y, z]),
        K::Array(items) if items.is_empty() => out.write_str(ValueId::EmptyArray.name(cx.lang)),
        K::Array(items) => {
            let items: Vec<&Expr> = items.iter().collect();
            write_call(out, cx, ValueId::Array.name(cx.lang), &items)
        }
        K::Value {
            id: ValueId::LocalizedString,
            args,
        } if matches!(
            args.first().map(Expr::kind),
            Some(K::Str(StrExpr::Localized(_)))
        ) =>
        {
            write_localized(out, cx, args)
        }
        K::Value {
            id: ValueId::CreateWorkshopSettingHero,
            args,
        } => write_hero_setting(out, cx, args),
        K::Value { id, args } => {
            let args: Vec<&Expr> = args.iter().collect();
            write_name_and_args(out, cx, id.name(cx.lang), &args)
        }
        K::GlobalVar(var) => write_global(out, cx, *var),
        K::PlayerVar { player, var } => write_player_var(out, cx, player, *var),
        K::Index { array, index } => write_call(
            out,
            cx,
            ValueId::ValueInArray.name(cx.lang),
            &[array, index],
        ),
        K::ArrayElem => out.write_str(ValueId::CurrentArrayElement.name(cx.lang)),
        K::ArrayIndex => out.write_str(ValueId::CurrentArrayIndex.name(cx.lang)),
        K::Compare { op, lhs, rhs } => {
            out.write_str(ValueId::Compare.name(cx.lang))?;
            out.write_char('(')?;
            write_expr(out, cx, lhs)?;
            write!(out, ", {}, ", operator(*op))?;
            write_expr(out, cx, rhs)?;
            out.write_char(')')
        }
        K::And(a, b) => write_call(out, cx, ValueId::And.name(cx.lang), &[a, b]),
        K::Or(a, b) => write_call(out, cx, ValueId::Or.name(cx.lang), &[a, b]),
        K::Not(a) => write_call(out, cx, ValueId::Not.name(cx.lang), &[a]),
        K::IfThenElse { cond, then, else_ } => write_call(
            out,
            cx,
            ValueId::IfThenElse.name(cx.lang),
            &[cond, then, else_],
        ),
        K::Add(a, b) => write_call(out, cx, ValueId::Add.name(cx.lang), &[a, b]),
        K::Sub(a, b) => write_call(out, cx, ValueId::Subtract.name(cx.lang), &[a, b]),
        K::Mul(a, b) => write_call(out, cx, ValueId::Multiply.name(cx.lang), &[a, b]),
        K::Div(a, b) => write_call(out, cx, ValueId::Divide.name(cx.lang), &[a, b]),
        K::Mod(a, b) => write_call(out, cx, ValueId::Modulo.name(cx.lang), &[a, b]),
        K::Pow(a, b) => write_call(out, cx, ValueId::RaiseToPower.name(cx.lang), &[a, b]),
        // The workshop has no negate.
        K::Neg(a) => {
            out.write_str(ValueId::Multiply.name(cx.lang))?;
            out.write_str("(-1, ")?;
            write_expr(out, cx, a)?;
            out.write_char(')')
        }
        // Removed by lowering and label resolution.
        K::Dict { .. }
        | K::Translated(_)
        | K::Resolve { .. }
        | K::Decompress(_)
        | K::StrModify { .. }
        | K::LocalizedCall { .. }
        | K::HolyGrail
        | K::RuleCondition
        | K::LabelDistance(_) => Err(fmt::Error),
    }
}

/// The localized `True` or `False`.
pub(super) fn truth(cx: Cx<'_>, value: bool) -> &'static str {
    if value { ValueId::True } else { ValueId::False }.name(cx.lang)
}

/// The symbol of a comparison.
pub(super) fn operator(op: CmpOp) -> &'static str {
    match op {
        CmpOp::Eq => "==",
        CmpOp::Ne => "!=",
        CmpOp::Lt => "<",
        CmpOp::Le => "<=",
        CmpOp::Gt => ">",
        CmpOp::Ge => ">=",
    }
}

/// Writes `Name(a, b)`, or just `Name` without arguments.
pub(super) fn write_name_and_args(
    out: &mut impl Write,
    cx: Cx<'_>,
    name: &str,
    args: &[&Expr],
) -> fmt::Result {
    out.write_str(name)?;
    if args.is_empty() {
        return Ok(());
    }
    out.write_char('(')?;
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            out.write_str(", ")?;
        }
        write_expr(out, cx, arg)?;
    }
    out.write_char(')')
}

fn write_call(out: &mut impl Write, cx: Cx<'_>, name: &str, args: &[&Expr]) -> fmt::Result {
    write_name_and_args(out, cx, name, args)
}

/// Writes a generated enum member: wrapped for heroes, maps, game modes, buttons and colors,
/// bare for everything else.
fn write_const(out: &mut impl Write, cx: Cx<'_>, c: ConstRef) -> fmt::Result {
    let name = constant_name(c.enum_name, c.key, cx.lang).ok_or(fmt::Error)?;
    let wrapper = match c.enum_name {
        "Hero" => Some(ValueId::Hero),
        "Map" => Some(ValueId::Map),
        "Gamemode" => Some(ValueId::Gamemode),
        "Button" => Some(ValueId::Button),
        "Color" => Some(ValueId::Color),
        _ => None,
    };
    match wrapper {
        Some(id) => write!(out, "{}({name})", id.name(cx.lang)),
        None => out.write_str(name),
    }
}

/// `Workshop Setting Hero(..)`: its default is the bare hero name, not `Hero(..)`.
fn write_hero_setting(out: &mut impl Write, cx: Cx<'_>, args: &[Expr]) -> fmt::Result {
    out.write_str(ValueId::CreateWorkshopSettingHero.name(cx.lang))?;
    out.write_char('(')?;
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            out.write_str(", ")?;
        }
        match arg.kind() {
            ExprKind::Const(c) if i == 2 => {
                out.write_str(constant_name(c.enum_name, c.key, cx.lang).ok_or(fmt::Error)?)?;
            }
            _ => write_expr(out, cx, arg)?,
        }
    }
    out.write_char(')')
}

/// `String("phrase", a, b, c)`; trailing `Null` arguments are left out.
fn write_localized(out: &mut impl Write, cx: Cx<'_>, args: &[Expr]) -> fmt::Result {
    let (phrase, rest) = match args.split_first() {
        Some((first, rest)) => match first.kind() {
            ExprKind::Str(StrExpr::Localized(phrase)) => (phrase, rest),
            _ => return Err(fmt::Error),
        },
        None => return Err(fmt::Error),
    };
    out.write_str(ValueId::LocalizedString.name(cx.lang))?;
    out.write_char('(')?;
    write_string(out, phrase.text(cx.lang))?;
    let used = rest
        .iter()
        .rposition(|arg| !matches!(arg.kind(), ExprKind::Null))
        .map_or(0, |last| last + 1);
    for arg in &rest[..used] {
        out.write_str(", ")?;
        write_expr(out, cx, arg)?;
    }
    out.write_char(')')
}

/// `Custom String("text {0}", a, b, c)`; trailing `Null` arguments are left out.
fn write_custom_string(
    out: &mut impl Write,
    cx: Cx<'_>,
    tokens: &[StrToken],
    args: &[Expr],
) -> fmt::Result {
    out.write_str(ValueId::CustomString.name(cx.lang))?;
    out.write_str("(\"")?;
    for token in tokens {
        match token {
            StrToken::Text(text) => write_escaped(out, text)?,
            StrToken::Arg(n) => write!(out, "{{{n}}}")?,
        }
    }
    out.write_char('"')?;
    let used = args
        .iter()
        .rposition(|arg| !matches!(arg.kind(), ExprKind::Null))
        .map_or(0, |last| last + 1);
    for arg in &args[..used] {
        out.write_str(", ")?;
        write_expr(out, cx, arg)?;
    }
    out.write_char(')')
}

fn var_name(cx: Cx<'_>, global: bool, var: VarId) -> Result<&str, fmt::Error> {
    let vars = if global {
        &cx.program.globals
    } else {
        &cx.program.players
    };
    vars.entries
        .get(var.0 as usize)
        .map(|decl| decl.name.as_str())
        .ok_or(fmt::Error)
}

/// `Global.X`
pub(super) fn write_global(out: &mut impl Write, cx: Cx<'_>, var: VarId) -> fmt::Result {
    // The `global:` section header is lowercase; a variable reference is capitalized.
    let mut word = Keyword::Global.name(cx.lang).chars();
    if let Some(first) = word.next() {
        write!(out, "{}", first.to_uppercase())?;
    }
    write!(out, "{}.{}", word.as_str(), var_name(cx, true, var)?)
}

/// `(P).X`
pub(super) fn write_player_var(
    out: &mut impl Write,
    cx: Cx<'_>,
    player: &Expr,
    var: VarId,
) -> fmt::Result {
    out.write_char('(')?;
    write_expr(out, cx, player)?;
    write!(out, ").{}", var_name(cx, false, var)?)
}

/// The raw name of a variable, as an action takes it.
pub(super) fn write_var_name(
    out: &mut impl Write,
    cx: Cx<'_>,
    global: bool,
    var: VarId,
) -> fmt::Result {
    out.write_str(var_name(cx, global, var)?)
}

#[cfg(test)]
mod tests;
