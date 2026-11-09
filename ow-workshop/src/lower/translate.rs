//! Lowers translated strings (`t!`, `tr(..)`) to arrays of every translation and a pick of the
//! viewer's language.
//!
//! The compiler stores all translations of a string in one array and indexes it by the position
//! of the viewer's language, found client-side by stringifying a constant (see `translations`).

use crate::ir::ErrorKind;
use std::fmt::Write as _;

use ow_workshop_data::ValueId;

use super::Ctx;
use super::client_args::client_side_args;
use super::visit::{blocks_mut, map_children, map_place};
use crate::catalog::text::{number_placeholders, strip_whitespace};
use crate::catalog::{Catalog, Entry};
use crate::ir::{
    Block, Diagnostics, Expr, ExprKind, Location, Program, Rule, Stmt, StmtKind, StrExpr, StrToken,
    Translated, VarId,
};
use crate::translations::{
    Detection, HELPER, LANGUAGE_INDEX, PlayerVarMode, SEPARATOR, TranslationLang, Translations,
    choose,
};

/// Longest custom string the game allows, in characters.
const SHORT_CHARS: usize = 128;

/// Most placeholders a custom string has.
const SHORT_ARGS: usize = 3;

/// Most arguments a translated string takes.
const MAX_ARGS: usize = 16;

/// Longest language part in replace mode, in UTF-8 bytes.
const PART_BYTES: usize = 511;

/// Shown for a stored translated string that is displayed without `tr`.
const TL_ERR: &str = "ＴＬＥｒｒ";

/// Where an expression is evaluated.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Place {
    /// On the server, where the viewer's language is not known.
    Server,
    /// In a reevaluated argument of a display action.
    Client,
    /// As the value of a `set`.
    Store,
}

/// The pass.
pub(crate) fn lower(program: &mut Program, _: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    let mut rules = std::mem::take(&mut program.rules);
    let Some(mut lowerer) = Lowerer::new(program) else {
        // Without translations a translated string is an error; with them, setup failed already.
        let mut errors = Diagnostics::default();
        for rule in &mut rules {
            report_unconfigured(rule, &mut errors);
        }
        program.rules = rules;
        return if errors.has_errors() {
            Err(errors)
        } else {
            Ok(())
        };
    };
    for rule in &mut rules {
        lowerer.rule(rule);
    }
    program.rules = rules;
    lowerer.finish(program)
}

fn report_unconfigured(rule: &Rule, errors: &mut Diagnostics) {
    let mut found = false;
    let mut check = |e: &Expr| {
        found |= has_translated(e);
    };
    for cond in &rule.conditions {
        check(&cond.expr);
    }
    super::visit::walk_block(&rule.body, &mut |stmt: &Stmt| {
        let mut stmt = stmt.clone();
        super::visit::map_stmt_roots(&mut stmt, &mut |e| {
            check(e);
            e.clone()
        });
    });
    if found {
        errors.error(ErrorKind::TranslationsNotConfigured,
            format!(
                "rule `{}` uses translated strings, but `Workshop::translations` returns no languages",
                rule.name
            ),
            rule.location,
        );
    }
}

fn has_translated(expr: &Expr) -> bool {
    matches!(
        expr.kind(),
        ExprKind::Translated(_) | ExprKind::Resolve { .. }
    ) || expr.kind().children().into_iter().any(has_translated)
}

struct Lowerer {
    langs: Vec<TranslationLang>,
    detection: Detection,
    helper: VarId,
    /// The player variable with the language index, in player-variable mode.
    language_index: Option<VarId>,
    tl_err: bool,
    source_lines: bool,
    old: Catalog,
    /// Entries used by this compile, in encounter order.
    used: Vec<Entry>,
    errors: Diagnostics,
    /// Name of the rule being lowered.
    rule: String,
}

impl Lowerer {
    fn new(program: &Program) -> Option<Self> {
        let t: &Translations = program
            .translations
            .as_ref()
            .filter(|t| !t.langs.is_empty())?;
        let find = |vars: &crate::ir::Vars, name: &str| {
            vars.entries
                .iter()
                .position(|d| d.name == name)
                .and_then(|i| u32::try_from(i).ok())
                .map(VarId)
        };
        Some(Self {
            langs: t.langs.clone(),
            detection: choose(&t.langs).ok()?,
            helper: find(&program.globals, HELPER)?,
            language_index: t
                .player_var
                .and_then(|_| find(&program.players, LANGUAGE_INDEX)),
            tl_err: t.player_var != Some(PlayerVarMode::NoTlErr),
            source_lines: t.source_lines,
            old: t.catalog.clone(),
            used: Vec::new(),
            errors: Diagnostics::default(),
            rule: String::new(),
        })
    }

    fn error(&mut self, code: ErrorKind, message: String, loc: Option<Location>) {
        self.errors.error(code, message, loc);
    }

    /// Stores the updated catalog back in the program.
    fn finish(mut self, program: &mut Program) -> Result<(), Diagnostics> {
        if let Some(t) = program.translations.as_mut() {
            let mut catalog = Catalog::default();
            catalog.entries.append(&mut self.used);
            for mut entry in std::mem::take(&mut self.old.entries) {
                if catalog
                    .get(&entry.msgid, entry.msgctxt.as_deref())
                    .is_none()
                {
                    entry.occurrences.clear();
                    catalog.entries.push(entry);
                }
            }
            t.catalog = catalog;
        }
        if self.errors.has_errors() {
            Err(self.errors)
        } else {
            Ok(())
        }
    }

    fn rule(&mut self, rule: &mut Rule) {
        self.rule.clone_from(&rule.name);
        for cond in &mut rule.conditions {
            cond.expr = self.expr(&cond.expr, Place::Server);
        }
        self.block(&mut rule.body);
    }

    fn block(&mut self, block: &mut Block) {
        for stmt in &mut block.stmts {
            self.stmt(stmt);
            for child in blocks_mut(stmt) {
                self.block(child);
            }
        }
    }

    fn stmt(&mut self, stmt: &mut Stmt) {
        match &mut stmt.kind {
            StmtKind::Action { id, args } => {
                let client = client_side_args(*id, args);
                for (i, arg) in args.iter_mut().enumerate() {
                    let place = if client.contains(i) {
                        Place::Client
                    } else {
                        Place::Server
                    };
                    *arg = self.expr(arg, place);
                }
            }
            StmtKind::Set { target, value } => {
                map_place(target, &mut |e| self.expr(e, Place::Server));
                *value = self.expr(value, Place::Store);
            }
            _ => super::visit::map_stmt_roots(stmt, &mut |e| self.expr(e, Place::Server)),
        }
    }

    fn expr(&mut self, expr: &Expr, place: Place) -> Expr {
        // Only the value itself may be stored; what it contains is evaluated on the server.
        let inner = if place == Place::Store {
            Place::Server
        } else {
            place
        };
        match expr.kind() {
            ExprKind::Translated(t) => {
                let args: Vec<Expr> = t.args.iter().map(|a| self.expr(a, inner)).collect();
                self.translated(expr, t, &args, place)
            }
            ExprKind::Resolve {
                inner: value,
                spectator,
            } => {
                let value = self.expr(value, inner);
                if place == Place::Client {
                    let array = value;
                    let index = self.language_pick(*spectator);
                    Expr::new(ExprKind::Index { array, index }, expr.loc())
                } else {
                    self.error(
                        ErrorKind::TranslationPlacement,
                        "`tr(..)` must be used in a reevaluated argument of a display action"
                            .to_owned(),
                        expr.loc(),
                    );
                    value
                }
            }
            kind => Expr::new(map_children(kind, &mut |c| self.expr(c, inner)), expr.loc()),
        }
    }

    /// The index of the viewer's language in an array of translations.
    fn language_pick(&self, spectator: bool) -> Expr {
        let ex = |kind| Expr::new(kind, None);
        let call = |id, args| ex(ExprKind::Value { id, args });
        if let (Some(var), false) = (self.language_index, spectator) {
            return ex(ExprKind::PlayerVar {
                player: call(ValueId::LocalPlayer, vec![]),
                var,
            });
        }
        let names = ex(ExprKind::GlobalVar(self.helper));
        let split = call(
            ValueId::Split,
            vec![self.detection.expr(), ex(ExprKind::Array(vec![]))],
        );
        let found = call(ValueId::Abs, vec![call(ValueId::Index, vec![names, split])]);
        // Without the leading error entry the languages start at 0.
        if self.tl_err {
            found
        } else {
            ex(ExprKind::Sub(found, ex(ExprKind::Num(1.0))))
        }
    }

    fn translated(&mut self, expr: &Expr, t: &Translated, args: &[Expr], place: Place) -> Expr {
        let loc = expr.loc();
        let wanted = if t.raw { Place::Store } else { place };
        if place == Place::Server {
            self.error(ErrorKind::TranslationPlacement,
                "translated strings must be displayed in a reevaluated argument or assigned to a variable"
                    .to_owned(),
                loc,
            );
            return expr.clone();
        }
        let Some(parts) = self.parts(t, args.len(), loc) else {
            return expr.clone();
        };
        self.record(t, &parts.key, loc);
        let ex = |kind| Expr::new(kind, loc);
        let replace = parts.replace.is_some();
        let array = {
            let (tokens, call_args) = match &parts.replace {
                None => (parts.tokens.clone(), args.to_vec()),
                Some(text) => (vec![StrToken::Text(text.clone())], Vec::new()),
            };
            let string = ex(ExprKind::Str(StrExpr::Custom {
                tokens,
                args: call_args,
            }));
            ex(ExprKind::Value {
                id: ValueId::Split,
                args: vec![string, ex(ExprKind::GlobalVar(self.helper))],
            })
        };
        match (wanted, replace) {
            (Place::Client, false) => ex(ExprKind::Index {
                array,
                index: self.language_pick(t.spectator),
            }),
            (Place::Client, true) => {
                let picked = ex(ExprKind::Index {
                    array,
                    index: self.language_pick(t.spectator),
                });
                replacements(picked, args)
            }
            (_, false) => array,
            (_, true) => ex(ExprKind::Value {
                id: ValueId::MappedArray,
                args: vec![array, replacements(ex(ExprKind::ArrayElem), args)],
            }),
        }
    }

    /// Records the string in the catalog, with where it is used.
    fn record(&mut self, t: &Translated, key: &str, loc: Option<Location>) {
        let context = t.context.as_deref();
        let at = self
            .used
            .iter()
            .position(|e| e.msgid == key && e.msgctxt.as_deref() == context)
            .unwrap_or_else(|| {
                let mut entry = self
                    .old
                    .get(key, context)
                    .cloned()
                    .unwrap_or_else(|| Entry::new(key, context.map(str::to_owned)));
                entry.occurrences.clear();
                self.used.push(entry);
                self.used.len() - 1
            });
        let place = match loc {
            Some(l) if self.source_lines => format!("file {}, line {}", l.file, l.line),
            Some(l) => format!("file {}", l.file),
            None => "file <unknown>".to_owned(),
        };
        let occurrence = format!("{place}, rule \"{}\"", self.rule);
        self.used[at].occurrences.push(occurrence);
    }
}

/// How a translated string is laid out in the custom string.
struct Parts {
    /// The catalog key.
    key: String,
    /// Tokens for the short form.
    tokens: Vec<StrToken>,
    /// The whole text with markers instead of placeholders, in replace mode.
    replace: Option<String>,
}

impl Lowerer {
    fn parts(&mut self, t: &Translated, nargs: usize, loc: Option<Location>) -> Option<Parts> {
        if nargs > MAX_ARGS {
            self.error(
                ErrorKind::TranslationArgs,
                format!("a translated string takes at most {MAX_ARGS} arguments, not {nargs}"),
                loc,
            );
            return None;
        }
        let stripped = strip_whitespace(&t.key);
        let Ok(key) = number_placeholders(&stripped.key) else {
            self.error(
                ErrorKind::TranslationArgs,
                format!("{:?} has more than {MAX_ARGS} placeholders", t.key),
                loc,
            );
            return None;
        };
        let mut texts = Vec::new();
        for (i, lang) in self.langs.clone().into_iter().enumerate() {
            let found = (i > 0)
                .then(|| self.old.translation(lang, &key, t.context.as_deref()))
                .flatten();
            let text = match found.map(number_placeholders) {
                None => stripped.apply(&key),
                Some(Ok(numbered)) => stripped.apply(&numbered),
                Some(Err(_)) => {
                    self.error(
                        ErrorKind::TranslationArgs,
                        format!(
                            "the {} translation of {key:?} has too many placeholders",
                            lang.code()
                        ),
                        loc,
                    );
                    return None;
                }
            };
            if text.contains(SEPARATOR) {
                self.error(
                    ErrorKind::TranslationSeparator,
                    format!(
                        "the {} text of {key:?} contains U+EC48, which is reserved",
                        lang.code()
                    ),
                    loc,
                );
                return None;
            }
            if let Some(bad) = placeholder_out_of_range(&text, nargs) {
                self.error(ErrorKind::TranslationPlaceholder,
                    format!("the {} text of {key:?} uses {{{bad}}}, but only {nargs} arguments are given", lang.code()),
                    loc,
                );
                return None;
            }
            texts.push(text);
        }
        let prefix = if self.tl_err {
            format!("{TL_ERR}{SEPARATOR}")
        } else {
            String::new()
        };
        let raw = format!("{prefix}{}", texts.join(&SEPARATOR.to_string()));
        if raw.chars().count() <= SHORT_CHARS && nargs <= SHORT_ARGS {
            return Some(Parts {
                key,
                tokens: tokens(&raw),
                replace: None,
            });
        }
        let mut marked = Vec::new();
        for text in &texts {
            let part = mark_placeholders(text);
            if part.len() > PART_BYTES {
                self.error(
                    ErrorKind::TranslationLength,
                    format!("a translation of {key:?} is longer than {PART_BYTES} bytes"),
                    loc,
                );
                return None;
            }
            marked.push(part);
        }
        let replace = format!("{prefix}{}", marked.join(&SEPARATOR.to_string()));
        Some(Parts {
            key,
            tokens: Vec::new(),
            replace: Some(replace),
        })
    }
}

/// The first placeholder `{n}` of `text` with `n >= nargs`.
fn placeholder_out_of_range(text: &str, nargs: usize) -> Option<usize> {
    placeholders(text).into_iter().find(|&n| n >= nargs)
}

/// The indices of the `{n}` placeholders of `text`.
fn placeholders(text: &str) -> Vec<usize> {
    let mut found = Vec::new();
    let mut rest = text;
    while let Some(open) = rest.find('{') {
        let after = &rest[open + 1..];
        match after.find('}') {
            Some(close)
                if !after[..close].is_empty()
                    && after[..close].bytes().all(|b| b.is_ascii_digit()) =>
            {
                if let Ok(n) = after[..close].parse() {
                    found.push(n);
                }
                rest = &after[close + 1..];
            }
            _ => rest = after,
        }
    }
    found
}

/// Splits `raw` into text and `{n}` placeholder tokens.
fn tokens(raw: &str) -> Vec<StrToken> {
    let mut out = Vec::new();
    let mut text = String::new();
    let mut rest = raw;
    while let Some(open) = rest.find('{') {
        text.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let number = after
            .find('}')
            .filter(|&c| c > 0 && after[..c].bytes().all(|b| b.is_ascii_digit()))
            .and_then(|c| after[..c].parse::<usize>().ok().map(|n| (n, c)));
        if let Some((n, close)) = number {
            if !text.is_empty() {
                out.push(StrToken::Text(std::mem::take(&mut text)));
            }
            out.push(StrToken::Arg(n));
            rest = &after[close + 1..];
        } else {
            text.push('{');
            rest = after;
        }
    }
    text.push_str(rest);
    if !text.is_empty() {
        out.push(StrToken::Text(text));
    }
    out
}

/// The number that stands for placeholder `6 + offset` in replace mode.
fn number_marker(offset: usize) -> f64 {
    1_876_650.25 + f64::from(u32::try_from(offset).unwrap_or(0))
}

/// The text a placeholder is replaced with in replace mode.
fn marker_text(index: usize) -> String {
    const VECTORS: [&str; 6] = [
        "(0.00, 1.00, 0.00)",
        "(0.00, -1.00, 0.00)",
        "(1.00, 0.00, 0.00)",
        "(-1.00, 0.00, 0.00)",
        "(0.00, 0.00, 1.00)",
        "(0.00, 0.00, -1.00)",
    ];
    VECTORS.get(index).map_or_else(
        || format!("{}", number_marker(index - VECTORS.len())),
        |s| (*s).to_owned(),
    )
}

fn mark_placeholders(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let number = after
            .find('}')
            .filter(|&c| c > 0 && after[..c].bytes().all(|b| b.is_ascii_digit()))
            .and_then(|c| after[..c].parse::<usize>().ok().map(|n| (n, c)));
        if let Some((n, close)) = number {
            let _ = write!(out, "{}", marker_text(n));
            rest = &after[close + 1..];
        } else {
            out.push('{');
            rest = after;
        }
    }
    out.push_str(rest);
    out
}

/// `String Replace(…, Update Every Frame(marker_i), arg_i)` for each argument, innermost first.
fn replacements(base: Expr, args: &[Expr]) -> Expr {
    let ex = |kind| Expr::new(kind, None);
    args.iter().enumerate().fold(base, |acc, (i, arg)| {
        let vector = [
            ValueId::VectorUp,
            ValueId::VectorDown,
            ValueId::VectorLeft,
            ValueId::VectorRight,
            ValueId::VectorForward,
            ValueId::VectorBackward,
        ];
        let marker = vector.get(i).map_or_else(
            || ex(ExprKind::Num(number_marker(i - vector.len()))),
            |id| {
                ex(ExprKind::Value {
                    id: *id,
                    args: vec![],
                })
            },
        );
        ex(ExprKind::Value {
            id: ValueId::Replace,
            args: vec![
                acc,
                ex(ExprKind::Value {
                    id: ValueId::UpdateEveryFrame,
                    args: vec![marker],
                }),
                arg.clone(),
            ],
        })
    })
}

#[cfg(test)]
mod tests;
