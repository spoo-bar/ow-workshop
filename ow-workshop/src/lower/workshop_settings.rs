//! Workshop settings: checks their strings, rewrites the characters the workshop cannot paste, and
//! keeps their names apart.

use crate::ir::ErrorKind;
use std::collections::BTreeSet;

use ow_workshop_data::ValueId;

use crate::ir::{Block, Diagnostics, Expr, ExprKind, Location, Program, StrExpr, StrToken};
use crate::lower::Ctx;
use crate::lower::visit::{blocks_mut, map_stmt_roots, rewrite_expr};
use crate::warning::Warning;

/// The most workshop settings a mod can have.
pub(crate) const MAX_SETTINGS: usize = 128;
/// The longest category, name or option, in characters.
const MAX_CHARS: usize = 128;
/// Appended to a string of only whitespace, which the workshop cannot paste.
const FILLER: char = '\u{3000}';

/// Replaced in every string: they break the paste.
const EVERYWHERE: [(char, char); 3] = [(':', '\u{E003A}'), ('{', '\u{E007B}'), ('}', '\u{E007D}')];
/// Also replaced in names: the game strips them when a setting is copied.
const IN_NAMES: [(char, char); 8] = [
    ('"', '\u{E0022}'),
    ('(', '\u{E0028}'),
    (')', '\u{E0029}'),
    (',', '\u{E002C}'),
    ('/', '\u{E002F}'),
    (';', '\u{E003B}'),
    ('\\', '\u{E005C}'),
    ('|', '\u{E007C}'),
];

/// What a string of a workshop setting is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Part {
    Category,
    Name,
    Option,
}

impl Part {
    fn label(self) -> &'static str {
        match self {
            Self::Category => "category",
            Self::Name => "name",
            Self::Option => "combo option",
        }
    }
}

/// Rewrites `text` so the workshop can paste it.
fn sanitize(text: &str, part: Part) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        let replaced = EVERYWHERE
            .iter()
            .chain(if part == Part::Name {
                &IN_NAMES[..]
            } else {
                &[]
            })
            .find(|(from, _)| *from == c);
        out.push(replaced.map_or(c, |&(_, to)| to));
    }
    if text.chars().all(char::is_whitespace) {
        out.push(FILLER);
    }
    out
}

/// The suffixes that tell equal names apart, in the order they are tried.
fn suffixes() -> impl Iterator<Item = String> {
    (0x01u8..=0x08).chain(0x0E..=0x1D).flat_map(|code| {
        let c = char::from(code);
        [format!("{c}"), format!("\u{1E}{c}"), format!("\u{1F}{c}")]
    })
}

/// The pass.
pub(crate) fn lower(program: &mut Program, ctx: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    let mut errors = Diagnostics::default();
    let mut used = Used::default();
    for rule in &mut program.rules {
        let mut env = Env {
            ctx: &mut *ctx,
            errors: &mut errors,
            used: &mut used,
        };
        let mut node = |e: Expr| lower_node(e, &mut env);
        for cond in &mut rule.conditions {
            cond.expr = rewrite_expr(&cond.expr, &mut node);
        }
        lower_block(&mut rule.body, &mut node);
    }
    if errors.has_errors() {
        return Err(errors);
    }
    Ok(())
}

/// The settings seen so far.
#[derive(Default)]
struct Used {
    count: usize,
    names: BTreeSet<String>,
}

struct Env<'a, 'b> {
    ctx: &'a mut Ctx<'b>,
    errors: &'a mut Diagnostics,
    used: &'a mut Used,
}

impl Env<'_, '_> {
    fn error(&mut self, code: ErrorKind, message: String, loc: Option<Location>) {
        self.errors.error(code, message, loc);
    }

    /// The text of a plain literal string, or an error.
    fn literal(&mut self, expr: &Expr, part: Part, loc: Option<Location>) -> Option<String> {
        if let ExprKind::Str(StrExpr::Custom { tokens, args }) = expr.kind()
            && args.is_empty()
        {
            let mut text = String::new();
            for token in tokens {
                if let StrToken::Text(t) = token {
                    text.push_str(t);
                }
            }
            return Some(text);
        }
        self.error(
            ErrorKind::WorkshopSettingNotLiteral,
            format!(
                "the {} of a workshop setting must be a plain string, not a placeholder or an \
                 expression",
                part.label()
            ),
            loc,
        );
        None
    }

    /// The sanitized text of a string argument; too-long strings are an error.
    fn string(&mut self, expr: &Expr, part: Part, loc: Option<Location>) -> Option<String> {
        let text = self.literal(expr, part, loc)?;
        let clean = sanitize(&text, part);
        self.check_length(&text, &clean, part, loc);
        Some(clean)
    }

    fn check_length(&mut self, before: &str, after: &str, part: Part, loc: Option<Location>) {
        for text in [before, after] {
            let chars = text.chars().count();
            if chars > MAX_CHARS {
                self.error(
                    ErrorKind::WorkshopSettingTooLong,
                    format!(
                        "the {} of a workshop setting is {chars} characters long; the most is \
                         {MAX_CHARS}",
                        part.label()
                    ),
                    loc,
                );
                return;
            }
        }
    }

    /// `name` made different from every name used so far.
    fn unique(&mut self, name: String, loc: Option<Location>) -> String {
        let chosen = std::iter::once(String::new())
            .chain(suffixes())
            .map(|suffix| format!("{name}{suffix}"))
            .find(|candidate| !self.used.names.contains(candidate));
        let Some(chosen) = chosen else {
            self.error(
                ErrorKind::WorkshopSettingNameExhausted,
                format!(
                    "more than 67 workshop settings are named the same; rename some of `{name}`"
                ),
                loc,
            );
            return name;
        };
        if chosen != name {
            // The suffix grows the name past what was checked.
            self.check_length(&name, &chosen, Part::Name, loc);
        }
        self.used.names.insert(chosen.clone());
        chosen
    }

    /// Counts a setting, and reports the first one over the limit.
    fn count(&mut self, loc: Option<Location>) {
        self.used.count += 1;
        if self.used.count == MAX_SETTINGS + 1 {
            self.error(
                ErrorKind::TooManyWorkshopSettings,
                format!("a mod can have at most {MAX_SETTINGS} workshop settings"),
                loc,
            );
        }
    }
}

fn lower_block(block: &mut Block, node: &mut impl FnMut(Expr) -> Expr) {
    for stmt in &mut block.stmts {
        map_stmt_roots(stmt, &mut |e| rewrite_expr(e, node));
        for child in blocks_mut(stmt) {
            lower_block(child, node);
        }
    }
}

/// Whether `id` is one of the `Workshop Setting …` values.
pub(crate) fn is_workshop_setting(id: ValueId) -> bool {
    matches!(
        id,
        ValueId::CreateWorkshopSettingBool
            | ValueId::CreateWorkshopSettingEnum
            | ValueId::CreateWorkshopSettingFloat
            | ValueId::CreateWorkshopSettingHero
            | ValueId::CreateWorkshopSettingInt
    )
}

fn lower_node(expr: Expr, env: &mut Env<'_, '_>) -> Expr {
    let ExprKind::Value { id, args } = expr.kind() else {
        return expr;
    };
    if !is_workshop_setting(*id) || args.len() < 3 {
        return expr;
    }
    let loc = expr.loc();
    env.count(loc);
    let mut args = args.clone();
    if let Some(category) = env.string(&args[0], Part::Category, loc) {
        args[0] = literal(&category, args[0].loc());
    }
    if let Some(name) = env.string(&args[1], Part::Name, loc) {
        let name = env.unique(name, loc);
        args[1] = literal(&name, args[1].loc());
    }
    if *id == ValueId::CreateWorkshopSettingEnum {
        combo(&mut args, loc, env);
    }
    Expr::new(ExprKind::Value { id: *id, args }, loc)
}

/// Rewrites the options of a combo, and warns about a default other than the first option.
fn combo(args: &mut [Expr], loc: Option<Location>, env: &mut Env<'_, '_>) {
    if let ExprKind::Num(default) = args[2].kind()
        && *default != 0.0
    {
        env.ctx.warn(
            Warning::WorkshopSettingEnumDefault,
            "a combo whose default is not its first option cannot select that option in the \
             lobby settings"
                .to_owned(),
            loc,
        );
    }
    let ExprKind::Array(options) = args[3].kind() else {
        env.error(
            ErrorKind::WorkshopSettingNotLiteral,
            "the options of a workshop setting must be a list of plain strings".to_owned(),
            loc,
        );
        return;
    };
    let options: Vec<Expr> = options
        .iter()
        .map(|option| {
            env.string(option, Part::Option, loc)
                .map_or_else(|| option.clone(), |text| literal(&text, option.loc()))
        })
        .collect();
    args[3] = Expr::new(ExprKind::Array(options), args[3].loc());
}

fn literal(text: &str, loc: Option<Location>) -> Expr {
    Expr::new(
        ExprKind::Str(StrExpr::Custom {
            tokens: vec![StrToken::Text(text.to_owned())],
            args: Vec::new(),
        }),
        loc,
    )
}

#[cfg(test)]
mod tests;
