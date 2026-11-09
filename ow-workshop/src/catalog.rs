//! The translation catalog: every translatable string with its translations.
//!
//! The core does no I/O: [`Catalog::merge_po`] reads `.po` text and [`Catalog::to_po`] returns it.

mod po;
pub(crate) mod text;

use std::collections::BTreeMap;

use crate::translations::TranslationLang;

/// A translatable string, keyed by `(msgid, msgctxt)`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct Entry {
    /// The source text, without outer whitespace and with numbered placeholders.
    pub msgid: String,
    /// Disambiguating context.
    pub msgctxt: Option<String>,
    /// Non-empty translations by language.
    pub translations: BTreeMap<TranslationLang, String>,
    /// Translator comments (`# …`).
    pub comments: Vec<String>,
    /// Where the string is used in this compile, as `file <module>, line <n>, rule "<name>"`.
    pub occurrences: Vec<String>,
    /// The `hasMultipleOccurrencesNotification` flag is set.
    pub flag_multi: bool,
}

impl Entry {
    /// An entry with no translations or occurrences.
    pub fn new(msgid: impl Into<String>, msgctxt: Option<String>) -> Self {
        Self {
            msgid: msgid.into(),
            msgctxt,
            ..Self::default()
        }
    }

    fn is_key(&self, msgid: &str, msgctxt: Option<&str>) -> bool {
        self.msgid == msgid && self.msgctxt.as_deref() == msgctxt
    }
}

/// All translatable strings of a mod.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct Catalog {
    /// The entries, in output order.
    pub entries: Vec<Entry>,
}

impl Catalog {
    /// The entry for `(msgid, msgctxt)`.
    pub fn get(&self, msgid: &str, msgctxt: Option<&str>) -> Option<&Entry> {
        self.entries.iter().find(|e| e.is_key(msgid, msgctxt))
    }

    /// The entry for `(msgid, msgctxt)`, created if it does not exist.
    pub fn entry_mut(&mut self, msgid: &str, msgctxt: Option<&str>) -> &mut Entry {
        let at = self
            .entries
            .iter()
            .position(|e| e.is_key(msgid, msgctxt))
            .unwrap_or_else(|| {
                self.entries
                    .push(Entry::new(msgid, msgctxt.map(str::to_owned)));
                self.entries.len() - 1
            });
        &mut self.entries[at]
    }

    /// The translation of `(msgid, msgctxt)` into `lang`, if it has one.
    pub fn translation(
        &self,
        lang: TranslationLang,
        msgid: &str,
        msgctxt: Option<&str>,
    ) -> Option<&str> {
        let text = self.get(msgid, msgctxt)?.translations.get(&lang)?;
        (!text.is_empty()).then_some(text.as_str())
    }
}

#[cfg(test)]
mod tests;
