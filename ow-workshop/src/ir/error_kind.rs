//! The errors that stop a compilation.

/// A kind of error, identified by a stable code. Errors cannot be suppressed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, thiserror::Error)]
#[non_exhaustive]
pub enum ErrorKind {
    /// A big-letters string matches no glyph.
    #[error("a big-letters string matches no glyph")]
    BigLettersNoMatch,
    /// A dictionary is used without an index.
    #[error("a dictionary is used without an index")]
    DictNotIndexed,
    /// Two entries claim the same slot index.
    #[error("two entries claim the same slot index")]
    DuplicateIndex,
    /// A label is declared twice.
    #[error("a label is declared twice")]
    DuplicateLabel,
    /// A name is declared twice.
    #[error("a name is declared twice")]
    DuplicateName,
    /// A node survived lowering and cannot be emitted.
    #[error("a node survived lowering and cannot be emitted")]
    EmitFailed,
    /// An extension is used but not enabled.
    #[error("an extension is used but not enabled")]
    ExtensionNotEnabled,
    /// A team or player filter on an event that takes none.
    #[error("a team or player filter on an event that takes none")]
    FilterNotAllowed,
    /// An explicit slot index is out of range.
    #[error("an explicit slot index is out of range")]
    IndexOutOfRange,
    /// A variable or subroutine name is not a valid identifier.
    #[error("a variable or subroutine name is not a valid identifier")]
    InvalidName,
    /// A label is placed before the skip that targets it.
    #[error("a label is placed before the skip that targets it")]
    LabelBeforeSkip,
    /// A skip targets a label in another rule.
    #[error("a skip targets a label in another rule")]
    LabelInOtherRule,
    /// A label is never placed.
    #[error("a label is never placed")]
    LabelNeverPlaced,
    /// A skip targets a label that does not exist.
    #[error("a skip targets a label that does not exist")]
    LabelNotFound,
    /// A label is placed more than once.
    #[error("a label is placed more than once")]
    LabelPlacedTwice,
    /// `do_while` is not followed by its condition.
    #[error("`do_while` is not followed by its condition")]
    MisplacedDoWhile,
    /// A required built-in is missing.
    #[error("a required built-in is missing")]
    MissingHolygrail,
    /// A lambda is nested inside another lambda.
    #[error("a lambda is nested inside another lambda")]
    NestedLambda,
    /// Content that exists only in Overwatch 1.
    #[error("content that exists only in Overwatch 1")]
    Ow1Only,
    /// The translation file is for another language.
    #[error("the translation file is for another language")]
    PoLanguage,
    /// A translation has a different number of lines.
    #[error("a translation has a different number of lines")]
    PoNewlines,
    /// The translation file is malformed.
    #[error("the translation file is malformed")]
    PoSyntax,
    /// A random value indexes a multi-dimensional array.
    #[error("a random value indexes a multi-dimensional array")]
    RandomInMultiDimIndex,
    /// The settings enable no game mode.
    #[error("the settings enable no game mode")]
    SettingsNoModes,
    /// A setting needs a whole number.
    #[error("a setting needs a whole number")]
    SettingsNotInteger,
    /// A setting is outside its range.
    #[error("a setting is outside its range")]
    SettingsOutOfRange,
    /// A setting exists only in Overwatch 1.
    #[error("a setting exists only in Overwatch 1")]
    SettingsOw1Only,
    /// The lobby slots are invalid.
    #[error("the lobby slots are invalid")]
    SettingsSlots,
    /// A setting text is too long.
    #[error("a setting text is too long")]
    SettingsStringTooLong,
    /// A setting names an unknown map.
    #[error("a setting names an unknown map")]
    SettingsUnknownMap,
    /// A rule sets both a slot and a hero.
    #[error("a rule sets both a slot and a hero")]
    SlotAndHero,
    /// A rule's slot is not in 0..=11.
    #[error("a rule's slot is not in 0..=11")]
    SlotOutOfRange,
    /// A string modifier is applied to the wrong kind of text.
    #[error("a string modifier is applied to the wrong kind of text")]
    StringModifierTarget,
    /// A subroutine has conditions.
    #[error("a subroutine has conditions")]
    SubroutineConditions,
    /// A subroutine is defined twice.
    #[error("a subroutine is defined twice")]
    SubroutineDefinedTwice,
    /// More variables or subroutines than the game allows.
    #[error("more variables or subroutines than the game allows")]
    TooMany,
    /// The mod uses more elements than the game allows.
    #[error("the mod uses more elements than the game allows")]
    TooManyElements,
    /// A localized string is given too many arguments.
    #[error("a localized string is given too many arguments")]
    TooManyLocalizedArgs,
    /// More workshop settings than the game allows.
    #[error("more workshop settings than the game allows")]
    TooManyWorkshopSettings,
    /// A translation uses arguments the string does not have.
    #[error("a translation uses arguments the string does not have")]
    TranslationArgs,
    /// The translations do not cover the same languages.
    #[error("the translations do not cover the same languages")]
    TranslationLanguages,
    /// A translation is too long.
    #[error("a translation is too long")]
    TranslationLength,
    /// A translation has a bad placeholder.
    #[error("a translation has a bad placeholder")]
    TranslationPlaceholder,
    /// A translated string is placed where it cannot be.
    #[error("a translated string is placed where it cannot be")]
    TranslationPlacement,
    /// A translation has a bad separator.
    #[error("a translation has a bad separator")]
    TranslationSeparator,
    /// Translated strings are used but no translations are configured.
    #[error("translated strings are used but no translations are configured")]
    TranslationsNotConfigured,
    /// A subroutine is called but never defined.
    #[error("a subroutine is called but never defined")]
    UndefinedSubroutine,
    /// A localized string does not exist.
    #[error("a localized string does not exist")]
    UnknownLocalizedString,
    /// No workshop setting name is left.
    #[error("no workshop setting name is left")]
    WorkshopSettingNameExhausted,
    /// A workshop setting is not a literal.
    #[error("a workshop setting is not a literal")]
    WorkshopSettingNotLiteral,
    /// A workshop setting text is too long.
    #[error("a workshop setting text is too long")]
    WorkshopSettingTooLong,
}

impl ErrorKind {
    /// The kebab-case code used in diagnostics.
    #[must_use]
    pub fn code(self) -> &'static str {
        match self {
            Self::BigLettersNoMatch => "big-letters-no-match",
            Self::DictNotIndexed => "dict-not-indexed",
            Self::DuplicateIndex => "duplicate-index",
            Self::DuplicateLabel => "duplicate-label",
            Self::DuplicateName => "duplicate-name",
            Self::EmitFailed => "emit-failed",
            Self::ExtensionNotEnabled => "extension-not-enabled",
            Self::FilterNotAllowed => "filter-not-allowed",
            Self::IndexOutOfRange => "index-out-of-range",
            Self::InvalidName => "invalid-name",
            Self::LabelBeforeSkip => "label-before-skip",
            Self::LabelInOtherRule => "label-in-other-rule",
            Self::LabelNeverPlaced => "label-never-placed",
            Self::LabelNotFound => "label-not-found",
            Self::LabelPlacedTwice => "label-placed-twice",
            Self::MisplacedDoWhile => "misplaced-do-while",
            Self::MissingHolygrail => "missing-holygrail",
            Self::NestedLambda => "nested-lambda",
            Self::Ow1Only => "ow1-only",
            Self::PoLanguage => "po-language",
            Self::PoNewlines => "po-newlines",
            Self::PoSyntax => "po-syntax",
            Self::RandomInMultiDimIndex => "random-in-multi-dim-index",
            Self::SettingsNoModes => "settings-no-modes",
            Self::SettingsNotInteger => "settings-not-integer",
            Self::SettingsOutOfRange => "settings-out-of-range",
            Self::SettingsOw1Only => "settings-ow1-only",
            Self::SettingsSlots => "settings-slots",
            Self::SettingsStringTooLong => "settings-string-too-long",
            Self::SettingsUnknownMap => "settings-unknown-map",
            Self::SlotAndHero => "slot-and-hero",
            Self::SlotOutOfRange => "slot-out-of-range",
            Self::StringModifierTarget => "string-modifier-target",
            Self::SubroutineConditions => "subroutine-conditions",
            Self::SubroutineDefinedTwice => "subroutine-defined-twice",
            Self::TooMany => "too-many",
            Self::TooManyElements => "too-many-elements",
            Self::TooManyLocalizedArgs => "too-many-localized-args",
            Self::TooManyWorkshopSettings => "too-many-workshop-settings",
            Self::TranslationArgs => "translation-args",
            Self::TranslationLanguages => "translation-languages",
            Self::TranslationLength => "translation-length",
            Self::TranslationPlaceholder => "translation-placeholder",
            Self::TranslationPlacement => "translation-placement",
            Self::TranslationSeparator => "translation-separator",
            Self::TranslationsNotConfigured => "translations-not-configured",
            Self::UndefinedSubroutine => "undefined-subroutine",
            Self::UnknownLocalizedString => "unknown-localized-string",
            Self::WorkshopSettingNameExhausted => "workshop-setting-name-exhausted",
            Self::WorkshopSettingNotLiteral => "workshop-setting-not-literal",
            Self::WorkshopSettingTooLong => "workshop-setting-too-long",
        }
    }

    /// Every kind, for tests and documentation.
    pub const ALL: &'static [Self] = &[
        Self::BigLettersNoMatch,
        Self::DictNotIndexed,
        Self::DuplicateIndex,
        Self::DuplicateLabel,
        Self::DuplicateName,
        Self::EmitFailed,
        Self::ExtensionNotEnabled,
        Self::FilterNotAllowed,
        Self::IndexOutOfRange,
        Self::InvalidName,
        Self::LabelBeforeSkip,
        Self::LabelInOtherRule,
        Self::LabelNeverPlaced,
        Self::LabelNotFound,
        Self::LabelPlacedTwice,
        Self::MisplacedDoWhile,
        Self::MissingHolygrail,
        Self::NestedLambda,
        Self::Ow1Only,
        Self::PoLanguage,
        Self::PoNewlines,
        Self::PoSyntax,
        Self::RandomInMultiDimIndex,
        Self::SettingsNoModes,
        Self::SettingsNotInteger,
        Self::SettingsOutOfRange,
        Self::SettingsOw1Only,
        Self::SettingsSlots,
        Self::SettingsStringTooLong,
        Self::SettingsUnknownMap,
        Self::SlotAndHero,
        Self::SlotOutOfRange,
        Self::StringModifierTarget,
        Self::SubroutineConditions,
        Self::SubroutineDefinedTwice,
        Self::TooMany,
        Self::TooManyElements,
        Self::TooManyLocalizedArgs,
        Self::TooManyWorkshopSettings,
        Self::TranslationArgs,
        Self::TranslationLanguages,
        Self::TranslationLength,
        Self::TranslationPlaceholder,
        Self::TranslationPlacement,
        Self::TranslationSeparator,
        Self::TranslationsNotConfigured,
        Self::UndefinedSubroutine,
        Self::UnknownLocalizedString,
        Self::WorkshopSettingNameExhausted,
        Self::WorkshopSettingNotLiteral,
        Self::WorkshopSettingTooLong,
    ];
}
