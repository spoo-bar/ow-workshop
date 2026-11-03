//! The languages workshop code can be written in.

use std::fmt;
use std::str::FromStr;

/// A language of the game client; workshop code must match it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Lang {
    /// German (`de-DE`)
    DeDe,
    /// English (`en-US`)
    #[default]
    EnUs,
    /// Spanish, Spain (`es-ES`)
    EsEs,
    /// Spanish, Mexico (`es-MX`)
    EsMx,
    /// French (`fr-FR`)
    FrFr,
    /// Italian (`it-IT`)
    ItIt,
    /// Japanese (`ja-JP`)
    JaJp,
    /// Korean (`ko-KR`)
    KoKr,
    /// Polish (`pl-PL`)
    PlPl,
    /// Portuguese, Brazil (`pt-BR`)
    PtBr,
    /// Russian (`ru-RU`)
    RuRu,
    /// Thai (`th-TH`)
    ThTh,
    /// Turkish (`tr-TR`)
    TrTr,
    /// Chinese, simplified (`zh-CN`)
    ZhCn,
    /// Chinese, traditional (`zh-TW`)
    ZhTw,
}

impl Lang {
    /// Every language, in discriminant order.
    pub const ALL: [Self; 15] = [
        Self::DeDe,
        Self::EnUs,
        Self::EsEs,
        Self::EsMx,
        Self::FrFr,
        Self::ItIt,
        Self::JaJp,
        Self::KoKr,
        Self::PlPl,
        Self::PtBr,
        Self::RuRu,
        Self::ThTh,
        Self::TrTr,
        Self::ZhCn,
        Self::ZhTw,
    ];

    /// The language code, e.g. `"en-US"`.
    pub fn code(self) -> &'static str {
        match self {
            Self::DeDe => "de-DE",
            Self::EnUs => "en-US",
            Self::EsEs => "es-ES",
            Self::EsMx => "es-MX",
            Self::FrFr => "fr-FR",
            Self::ItIt => "it-IT",
            Self::JaJp => "ja-JP",
            Self::KoKr => "ko-KR",
            Self::PlPl => "pl-PL",
            Self::PtBr => "pt-BR",
            Self::RuRu => "ru-RU",
            Self::ThTh => "th-TH",
            Self::TrTr => "tr-TR",
            Self::ZhCn => "zh-CN",
            Self::ZhTw => "zh-TW",
        }
    }
}

/// An unknown language code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseLangError(String);

impl fmt::Display for ParseLangError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown language code `{}`", self.0)
    }
}

impl std::error::Error for ParseLangError {}

impl FromStr for Lang {
    type Err = ParseLangError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|lang| lang.code() == code)
            .ok_or_else(|| ParseLangError(code.to_owned()))
    }
}

/// Picks `lang` from translations joined by U+001F in `Lang` order; empty or missing means English.
pub(crate) fn localized(packed: &'static str, lang: Lang) -> &'static str {
    let slot = |lang: Lang| {
        packed
            .split('\u{1f}')
            .nth(lang as usize)
            .filter(|s| !s.is_empty())
    };
    slot(lang).or_else(|| slot(Lang::EnUs)).unwrap_or_default()
}

/// All translations of one name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Names(pub(crate) &'static str);

impl Names {
    /// The name in `lang`, or in English if it has no translation.
    pub fn get(self, lang: Lang) -> &'static str {
        localized(self.0, lang)
    }
}
