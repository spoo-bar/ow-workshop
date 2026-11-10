//! Rewrites of strings, teams, colors and direction constants.

// Channels are named r, g, b, a.
#![allow(clippy::many_single_char_names)]

use ow_workshop_data::{Color, Lang, LocalizedString, ValueId as V};

use super::Env;
use super::build::{call_args, num, num_of, vec3};
use super::helpers::is_literal;
use crate::ir::{ConstRef, Expr, ExprKind, StrExpr, StrToken};

/// Texts of localized phrases that read the same in every language, so a custom string does the
/// same job with one element fewer.
const LANGUAGE_NEUTRAL: [&str; 30] = [
    "",
    "*",
    "----------",
    "#{0}",
    "-> {0}",
    "<-> {0}",
    "<- {0}",
    "{0} ->",
    "{0} <->",
    "{0} <-",
    "{0} -> {1}",
    "{0} - {1}",
    "{0} != {1}",
    "{0} * {1}",
    "{0} / {1}",
    "{0} + {1}",
    "{0} <-> {1}",
    "{0} <- {1}",
    "{0} <= {1}",
    "{0} < {1}",
    "{0} == {1}",
    "{0} = {1}",
    "{0} >= {1}",
    "{0} > {1}",
    "{0} {1}",
    "{0} : {1} : {2}",
    "{0} {1} {2}",
    "({0})",
    "¡{0}!",
    "¿{0}?",
];

fn custom(text: &str, args: Vec<Expr>, like: &Expr) -> Expr {
    let mut tokens = Vec::new();
    let mut rest = text;
    while !rest.is_empty() {
        let placeholder = rest.strip_prefix('{').and_then(|r| {
            let digit = r.chars().next()?.to_digit(10)?;
            let r = r.get(1..)?.strip_prefix('}')?;
            Some((usize::try_from(digit).ok()?, r))
        });
        if let Some((n, after)) = placeholder {
            tokens.push(StrToken::Arg(n));
            rest = after;
            continue;
        }
        let end = rest[1..].find('{').map_or(rest.len(), |p| p + 1);
        tokens.push(StrToken::Text(rest[..end].to_owned()));
        rest = &rest[end..];
    }
    Expr::new(ExprKind::Str(StrExpr::Custom { tokens, args }), like.loc())
}

/// The text of a custom string without placeholders, if folding on it is sound.
fn plain(e: &Expr, strict: bool) -> Option<String> {
    let ExprKind::Str(StrExpr::Custom { tokens, .. }) = e.kind() else {
        return None;
    };
    if !is_literal(e, strict) {
        return None;
    }
    tokens
        .iter()
        .map(|t| match t {
            StrToken::Text(text) => Some(text.as_str()),
            StrToken::Arg(_) => None,
        })
        .collect()
}

fn index(e: &Expr) -> Option<usize> {
    let n = num_of(e)?.round().max(0.0);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    Some(n as usize)
}

fn string_fold(e: &Expr, id: V, args: &[Expr], strict: bool) -> Option<Expr> {
    let text = plain(args.first()?, strict)?;
    #[allow(clippy::cast_precision_loss)] // string lengths are tiny
    match (id, args) {
        (V::StrContains, [_, sub]) => {
            let found = text.contains(&plain(sub, strict)?);
            Some(Expr::new(ExprKind::Bool(found), e.loc()))
        }
        (V::StrLen, [_]) => Some(num(text.chars().count() as f64, e.loc())),
        (V::CharAt, [_, at]) => {
            let c: String = text.chars().skip(index(at)?).take(1).collect();
            Some(custom(&c, Vec::new(), e))
        }
        (V::Substring, [_, start, len]) => {
            let part: String = text.chars().skip(index(start)?).take(index(len)?).collect();
            Some(custom(&part, Vec::new(), e))
        }
        (V::Replace, [_, pattern, with]) => {
            let pattern = plain(pattern, strict)?;
            if pattern.is_empty() {
                return None;
            }
            Some(custom(
                &text.replace(&pattern, &plain(with, strict)?),
                Vec::new(),
                e,
            ))
        }
        _ => None,
    }
}

/// `String("x")` whose text is the same in every language becomes a custom string.
fn neutral(e: &Expr, phrase: LocalizedString, args: &[Expr]) -> Option<Expr> {
    let text = phrase.text(Lang::EnUs);
    if !LANGUAGE_NEUTRAL.contains(&text) || Lang::ALL.iter().any(|l| phrase.text(*l) != text) {
        return None;
    }
    let used = (0..=2)
        .rev()
        .find(|n| text.contains(&format!("{{{n}}}")))
        .map_or(0, |n| n + 1);
    if args.len() < used {
        return None;
    }
    Some(custom(text, args[..used].to_vec(), e))
}

fn direction(id: V) -> Option<[f64; 3]> {
    Some(match id {
        V::VectorLeft => [1.0, 0.0, 0.0],
        V::VectorRight => [-1.0, 0.0, 0.0],
        V::VectorUp => [0.0, 1.0, 0.0],
        V::VectorDown => [0.0, -1.0, 0.0],
        V::VectorForward => [0.0, 0.0, 1.0],
        V::VectorBackward => [0.0, 0.0, -1.0],
        _ => return None,
    })
}

fn opposite_team(e: &Expr, team: &Expr) -> Option<Expr> {
    let ExprKind::Const(c) = team.kind() else {
        return None;
    };
    let key = match (c.enum_name, c.key) {
        ("Team", "1") => "2",
        ("Team", "2") => "1",
        ("Team", "ALL") => "ALL",
        _ => return None,
    };
    Some(Expr::new(
        ExprKind::Const(ConstRef {
            enum_name: "Team",
            key,
        }),
        e.loc(),
    ))
}

/// A custom color whose RGBA is a built-in color becomes that color.
#[allow(clippy::float_cmp)] // channels are whole numbers
fn builtin_color(e: &Expr, args: &[Expr]) -> Option<Expr> {
    let [r, g, b, a] = args else { return None };
    let rgba = [num_of(r)?, num_of(g)?, num_of(b)?, num_of(a)?];
    let found = Color::ALL
        .iter()
        .find(|c| c.rgba().is_some_and(|fixed| fixed.map(f64::from) == rgba))?;
    Some(Expr::new(
        ExprKind::Const(ConstRef {
            enum_name: "Color",
            key: found.key(),
        }),
        e.loc(),
    ))
}

/// Largest magnitude of a number literal folded into a string's text.
const MAX_FOLDED: f64 = 10_000_000.0;

/// Folds arguments that became literals (a constant sum, a language-neutral phrase's text) into
/// the template, as the normalization pass does for arguments that start out literal.
fn fold_args(e: &Expr, tokens: &[StrToken], args: &[Expr]) -> Option<Expr> {
    let folded = |arg: &Expr| -> Option<String> {
        match arg.kind() {
            ExprKind::Str(StrExpr::Custom { tokens, args: a }) if a.is_empty() => tokens
                .iter()
                .map(|t| match t {
                    StrToken::Text(text) => Some(text.as_str()),
                    StrToken::Arg(_) => None,
                })
                .collect(),
            _ => num_of(arg)
                .filter(|n| n.abs() <= MAX_FOLDED)
                .map(crate::lower::strings::to_fixed_2),
        }
    };
    if !tokens
        .iter()
        .any(|t| matches!(t, StrToken::Arg(i) if args.get(*i).and_then(folded).is_some()))
    {
        return None;
    }
    let mut out = Vec::new();
    let mut out_args: Vec<Expr> = Vec::new();
    for token in tokens {
        match token {
            StrToken::Text(text) => crate::fmt::merge_text(&mut out, text),
            // A placeholder without an argument prints nothing.
            StrToken::Arg(i) => {
                if let Some(arg) = args.get(*i) {
                    if let Some(text) = folded(arg) {
                        crate::fmt::merge_text(&mut out, &text);
                    } else {
                        out.push(StrToken::Arg(out_args.len()));
                        out_args.push(arg.clone());
                    }
                }
            }
        }
    }
    Some(Expr::new(
        ExprKind::Str(StrExpr::Custom {
            tokens: out,
            args: out_args,
        }),
        e.loc(),
    ))
}

/// String, team, color and direction rewrites.
pub(super) fn rule(e: &Expr, env: &mut Env<'_, '_>) -> Option<Expr> {
    let strict = env.scope.strict();
    match e.kind() {
        ExprKind::Str(StrExpr::Localized(phrase)) => neutral(e, *phrase, &[]),
        ExprKind::Str(StrExpr::Custom { tokens, args }) => fold_args(e, tokens, args),
        ExprKind::Value { id, args } => match (*id, args.as_slice()) {
            (V::GetOppositeTeam, [team]) => opposite_team(e, team),
            (V::Rgb, _) if !strict => builtin_color(e, args),
            (V::LocalizedString, [phrase, rest @ ..]) => {
                let ExprKind::Str(StrExpr::Localized(phrase)) = phrase.kind() else {
                    return None;
                };
                neutral(e, *phrase, rest)
            }
            (id, []) => direction(id).map(|v| vec3(v, e.loc())),
            (id, _) => string_fold(e, id, args, strict),
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests;
