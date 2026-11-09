//! Translation settings: the languages of a mod and how the viewer's language is detected.

mod detect;

use ow_workshop_data::Lang;

use crate::catalog::Catalog;

pub(crate) use detect::{Detection, HELPER, LANGUAGE_INDEX, SEPARATOR, choose, declare};

/// A translation target; the plain `Es`, `Zh` and the regional codes pick a game language.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum TranslationLang {
    /// German.
    De,
    /// English.
    En,
    /// Spanish (Mexico).
    Es,
    /// Spanish (Spain).
    EsEs,
    /// Spanish (Mexico).
    EsMx,
    /// French.
    Fr,
    /// Italian.
    It,
    /// Japanese.
    Ja,
    /// Korean.
    Ko,
    /// Polish.
    Pl,
    /// Portuguese (Brazil).
    Pt,
    /// Russian.
    Ru,
    /// Thai.
    Th,
    /// Turkish.
    Tr,
    /// Chinese (simplified).
    Zh,
    /// Chinese (simplified).
    ZhCn,
    /// Chinese (traditional).
    ZhTw,
}

impl TranslationLang {
    /// Every translation language.
    pub const ALL: [Self; 17] = [
        Self::De,
        Self::En,
        Self::Es,
        Self::EsEs,
        Self::EsMx,
        Self::Fr,
        Self::It,
        Self::Ja,
        Self::Ko,
        Self::Pl,
        Self::Pt,
        Self::Ru,
        Self::Th,
        Self::Tr,
        Self::Zh,
        Self::ZhCn,
        Self::ZhTw,
    ];

    /// The lowercase code, e.g. `"es_mx"`.
    pub fn code(self) -> &'static str {
        match self {
            Self::De => "de",
            Self::En => "en",
            Self::Es => "es",
            Self::EsEs => "es_es",
            Self::EsMx => "es_mx",
            Self::Fr => "fr",
            Self::It => "it",
            Self::Ja => "ja",
            Self::Ko => "ko",
            Self::Pl => "pl",
            Self::Pt => "pt",
            Self::Ru => "ru",
            Self::Th => "th",
            Self::Tr => "tr",
            Self::Zh => "zh",
            Self::ZhCn => "zh_cn",
            Self::ZhTw => "zh_tw",
        }
    }

    /// The game language whose names tell this language apart.
    pub fn game_lang(self) -> Lang {
        match self {
            Self::De => Lang::DeDe,
            Self::En => Lang::EnUs,
            Self::Es | Self::EsMx => Lang::EsMx,
            Self::EsEs => Lang::EsEs,
            Self::Fr => Lang::FrFr,
            Self::It => Lang::ItIt,
            Self::Ja => Lang::JaJp,
            Self::Ko => Lang::KoKr,
            Self::Pl => Lang::PlPl,
            Self::Pt => Lang::PtBr,
            Self::Ru => Lang::RuRu,
            Self::Th => Lang::ThTh,
            Self::Tr => Lang::TrTr,
            Self::Zh | Self::ZhCn => Lang::ZhCn,
            Self::ZhTw => Lang::ZhTw,
        }
    }
}

/// An unknown translation language code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseTranslationLangError(String);

impl std::fmt::Display for ParseTranslationLangError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown translation language `{}`", self.0)
    }
}

impl std::error::Error for ParseTranslationLangError {}

impl std::str::FromStr for TranslationLang {
    type Err = ParseTranslationLangError;

    /// Parses a code such as `es_mx`; case and `-` versus `_` do not matter.
    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let normalized = code.trim().replace('-', "_").to_ascii_lowercase();
        Self::ALL
            .into_iter()
            .find(|lang| lang.code() == normalized)
            .ok_or_else(|| ParseTranslationLangError(code.to_owned()))
    }
}

/// How the viewer's language is found when it is stored in a player variable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PlayerVarMode {
    /// Index starts at 1; an unresolved string shows `TLErr`.
    Default,
    /// Index starts at 0; an unresolved string shows the source language.
    NoTlErr,
}

/// Translations of the mod's strings.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct Translations {
    /// The languages; the first is the source language.
    pub langs: Vec<TranslationLang>,
    /// The translations known so far.
    pub catalog: Catalog,
    /// Detect each player's language once and keep it in a player variable.
    pub player_var: Option<PlayerVarMode>,
    /// Keep catalog entries no string uses any more.
    pub keep_unused: bool,
    /// Write source line numbers in the occurrence comments.
    pub source_lines: bool,
}

impl Translations {
    /// Translations into `langs`; the first is the source language.
    pub fn new(langs: impl IntoIterator<Item = TranslationLang>) -> Self {
        Self {
            langs: langs.into_iter().collect(),
            ..Self::default()
        }
    }

    /// Starts from the translations in `catalog`.
    #[must_use]
    pub fn with_catalog(mut self, catalog: Catalog) -> Self {
        self.catalog = catalog;
        self
    }

    /// Detects the language once per player instead of in every string.
    #[must_use]
    pub fn with_player_var(mut self, mode: PlayerVarMode) -> Self {
        self.player_var = Some(mode);
        self
    }

    /// Keeps unused catalog entries when writing them out.
    #[must_use]
    pub fn keep_unused(mut self, keep: bool) -> Self {
        self.keep_unused = keep;
        self
    }

    /// Writes source line numbers in the occurrence comments.
    #[must_use]
    pub fn source_lines(mut self, lines: bool) -> Self {
        self.source_lines = lines;
        self
    }

    /// The source language, if any language is configured.
    pub fn source(&self) -> Option<TranslationLang> {
        self.langs.first().copied()
    }
}

#[cfg(test)]
mod tests;
