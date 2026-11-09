//! Reading and writing gettext `.po` text.

use std::fmt::Write as _;

use super::{Catalog, Entry};
use crate::ir::{Diagnostics, Severity};
use crate::translations::TranslationLang;

/// The flag that marks an entry whose occurrences may need different translations.
const MULTI_FLAG: &str = "hasMultipleOccurrencesNotification";

/// The comment added next to that flag.
const MULTI_TODO: &str = "TODO: check if all occurrences have the same translation";

/// An entry being read.
#[derive(Default)]
struct Raw {
    comments: Vec<String>,
    flag_multi: bool,
    msgctxt: Option<String>,
    msgid: Option<String>,
    msgstr: Option<String>,
    line: usize,
}

/// Which string a continuation line extends.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Field {
    Ctxt,
    Id,
    Str,
}

impl Catalog {
    /// Reads the translations into `lang` from `.po` text and merges them into the catalog.
    ///
    /// An empty `msgstr` means untranslated. Fails when the `Language` header names another
    /// language or a translation has a different number of lines than its `msgid`.
    pub fn merge_po(&mut self, lang: TranslationLang, po: &str) -> Result<(), Diagnostics> {
        let mut errors = Diagnostics::default();
        let mut raws = Vec::new();
        parse(po, &mut raws, &mut errors);
        for raw in raws {
            self.merge_raw(lang, raw, &mut errors);
        }
        if errors.has_errors() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn merge_raw(&mut self, lang: TranslationLang, raw: Raw, errors: &mut Diagnostics) {
        let (Some(msgid), Some(msgstr)) = (raw.msgid.as_deref(), raw.msgstr.as_deref()) else {
            return;
        };
        if msgid.is_empty() && raw.msgctxt.is_none() {
            check_header(lang, msgstr, raw.line, errors);
            return;
        }
        if !msgstr.is_empty() && msgstr.matches('\n').count() != msgid.matches('\n').count() {
            error(
                errors,
                "po-newlines",
                raw.line,
                format!("the translation of {msgid:?} has a different number of lines"),
            );
            return;
        }
        let entry = self.entry_mut(msgid, raw.msgctxt.as_deref());
        if !msgstr.is_empty() {
            entry.translations.insert(lang, msgstr.to_owned());
        }
        for comment in raw.comments {
            if !entry.comments.contains(&comment) {
                entry.comments.push(comment);
            }
        }
        entry.flag_multi |= raw.flag_multi;
    }

    /// The `.po` text for `lang`.
    ///
    /// Entries without occurrences are unused: they are dropped unless `keep_unused`, and
    /// kept ones are marked. Used entries come first, in the order they are stored.
    pub fn to_po(&self, lang: TranslationLang, keep_unused: bool) -> String {
        let mut out = format!(
            "msgid \"\"\nmsgstr \"\"\n\"Content-Type: text/plain; charset=UTF-8\\n\"\n\"Language: {}\\n\"\n\"X-Generator: ow-workshop\\n\"\n",
            lang.code()
        );
        let used = self.entries.iter().filter(|e| !e.occurrences.is_empty());
        let unused = self
            .entries
            .iter()
            .filter(|e| e.occurrences.is_empty() && keep_unused);
        for entry in used.chain(unused) {
            write_entry(&mut out, entry, lang);
        }
        out
    }
}

fn error(errors: &mut Diagnostics, code: &'static str, line: usize, message: String) {
    let message = if line > 0 {
        format!("line {line}: {message}")
    } else {
        message
    };
    errors.push(Severity::Error, code, message, None);
}

/// Checks the `Language` line of the header.
fn check_header(lang: TranslationLang, header: &str, line: usize, errors: &mut Diagnostics) {
    let found = header
        .lines()
        .find_map(|l| l.strip_prefix("Language:"))
        .map(str::trim);
    if let Some(found) = found
        && found.parse::<TranslationLang>() != Ok(lang)
    {
        error(
            errors,
            "po-language",
            line,
            format!("the file is for language `{found}`, not `{}`", lang.code()),
        );
    }
}

fn parse(po: &str, raws: &mut Vec<Raw>, errors: &mut Diagnostics) {
    let mut current = Raw::default();
    let mut field = None;
    for (n, line) in po.lines().enumerate().map(|(i, l)| (i + 1, l.trim())) {
        if line.is_empty() || line.starts_with("#~") || line.starts_with("#|") {
            continue;
        }
        if let Some(rest) = line.strip_prefix('#') {
            if current.msgstr.is_some() {
                raws.push(std::mem::take(&mut current));
            }
            field = None;
            comment(&mut current, rest);
            continue;
        }
        let keyword = |name: &str| {
            line.strip_prefix(name)
                .filter(|rest| rest.starts_with([' ', '"']))
        };
        let next = if let Some(rest) = keyword("msgctxt") {
            (Field::Ctxt, rest)
        } else if let Some(rest) = keyword("msgid") {
            (Field::Id, rest)
        } else if let Some(rest) = keyword("msgstr") {
            (Field::Str, rest)
        } else if line.starts_with('"') {
            let Some(f) = field else {
                error(
                    errors,
                    "po-syntax",
                    n,
                    "a string without a keyword".to_owned(),
                );
                return;
            };
            (f, line)
        } else {
            error(errors, "po-syntax", n, format!("unexpected `{line}`"));
            return;
        };
        let (f, rest) = next;
        let continues = field == Some(f) && rest.as_ptr() == line.as_ptr();
        if !continues && (f == Field::Ctxt || f == Field::Id) && current.msgstr.is_some() {
            raws.push(std::mem::take(&mut current));
        }
        if !continues && current.line == 0 {
            current.line = n;
        }
        let Some(text) = unquote(rest.trim()) else {
            error(errors, "po-syntax", n, format!("bad string `{rest}`"));
            return;
        };
        let slot = match f {
            Field::Ctxt => &mut current.msgctxt,
            Field::Id => &mut current.msgid,
            Field::Str => &mut current.msgstr,
        };
        slot.get_or_insert_with(String::new).push_str(&text);
        field = Some(f);
    }
    if current.msgstr.is_some() || current.msgid.is_some() {
        raws.push(current);
    }
}

fn comment(raw: &mut Raw, rest: &str) {
    match rest.chars().next() {
        Some(',') => {
            raw.flag_multi |= rest[1..].split(',').any(|f| f.trim() == MULTI_FLAG);
        }
        // Extracted comments and references are rebuilt on every compile.
        Some('.' | ':') => {}
        Some(_) => raw
            .comments
            .push(rest.strip_prefix(' ').unwrap_or(rest).to_owned()),
        None => raw.comments.push(String::new()),
    }
}

/// The text between the quotes of `"…"`, with escapes resolved.
fn unquote(quoted: &str) -> Option<String> {
    let inner = quoted.strip_prefix('"')?.strip_suffix('"')?;
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        out.push(match chars.next()? {
            'n' => '\n',
            't' => '\t',
            'r' => '\r',
            '"' => '"',
            '\\' => '\\',
            _ => return None,
        });
    }
    Some(out)
}

/// `text` as one quoted line, or as several (one per line of text) when it has newlines.
fn quote(keyword: &str, text: &str) -> String {
    let escape = |s: &str| {
        s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\t', "\\t")
            .replace('\r', "\\r")
    };
    if !text.contains('\n') {
        return format!("{keyword} \"{}\"\n", escape(text));
    }
    let mut out = format!("{keyword} \"\"\n");
    for line in text.split_inclusive('\n') {
        let body = escape(line.strip_suffix('\n').unwrap_or(line));
        let newline = if line.ends_with('\n') { "\\n" } else { "" };
        let _ = writeln!(out, "\"{body}{newline}\"");
    }
    out
}

fn write_entry(out: &mut String, entry: &Entry, lang: TranslationLang) {
    out.push('\n');
    let translated = entry.translations.get(&lang).is_some_and(|t| !t.is_empty());
    let many = entry.occurrences.len() > 1;
    let add_todo = many && entry.msgctxt.is_none() && !translated && !entry.flag_multi;
    let mut comments = entry.comments.clone();
    if add_todo && !comments.iter().any(|c| c == MULTI_TODO) {
        comments.push(MULTI_TODO.to_owned());
    }
    for comment in &comments {
        out.push_str(&format!("# {comment}\n").replace("# \n", "#\n"));
    }
    if many {
        let _ = writeln!(out, "#. {} occurrences:", entry.occurrences.len());
    }
    for occurrence in &entry.occurrences {
        let dash = if many { " - " } else { "" };
        let _ = writeln!(out, "#. {dash}{occurrence}");
    }
    if entry.occurrences.is_empty() {
        out.push_str("#. Unused translation\n");
    }
    if entry.flag_multi || add_todo {
        let _ = writeln!(out, "#, {MULTI_FLAG}");
    }
    if let Some(ctxt) = &entry.msgctxt {
        out.push_str(&quote("msgctxt", ctxt));
    }
    out.push_str(&quote("msgid", &entry.msgid));
    let text = entry.translations.get(&lang).map_or("", String::as_str);
    out.push_str(&quote("msgstr", text));
}
