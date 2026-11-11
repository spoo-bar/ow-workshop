//! Warnings a rule can suppress.

/// A kind of warning, identified by a stable code.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, thiserror::Error)]
#[non_exhaustive]
pub enum Warning {
    /// The mod uses more elements than the game allows.
    #[error("the mod uses more elements than the game allows")]
    ElementLimit,
    /// A variable name the game refuses to paste.
    #[error("a variable name the game refuses to paste")]
    CensoredVarName,
    /// `break_()` outside a loop or switch.
    #[error("`break_()` outside a loop or switch")]
    BreakOutsideLoop,
    /// `continue_()` outside a loop.
    #[error("`continue_()` outside a loop")]
    ContinueOutsideLoop,
    /// A subroutine reads event values the calling rule's event does not provide.
    #[error("A subroutine reads event values the calling rule's event does not provide")]
    MismatchedSubroutineEvent,
    /// `Start Rule(.., Restart Rule)` on a subroutine that waits, which eventually crashes the server.
    #[error(
        "`Start Rule(.., Restart Rule)` on a subroutine that waits, which eventually crashes the server"
    )]
    StartRuleCrash,
    /// A chased variable is also a `For` loop variable, so the loop never runs.
    #[error("A chased variable is also a `For` loop variable, so the loop never runs")]
    ChasedVarInFor,
    /// A rule condition reads a variable that is chased, which breaks re-triggering in Overwatch 2.
    #[error(
        "A rule condition reads a variable that is chased, which breaks re-triggering in Overwatch 2"
    )]
    Ow2RuleConditionChase,
    /// `Chase … At Rate` towards `9999`, which a long game can exceed.
    #[error("`Chase … At Rate` towards `9999`, which a long game can exceed")]
    Chase9999,
    /// `Closest Player To` also finds dead and unspawned players.
    #[error("`Closest Player To` also finds dead and unspawned players")]
    ClosestPlayer,
    /// `Farthest Player From` also finds dead and unspawned players.
    #[error("`Farthest Player From` also finds dead and unspawned players")]
    FarthestPlayer,
    /// `Player Closest To Reticle` also finds dead and unspawned players.
    #[error("`Player Closest To Reticle` also finds dead and unspawned players")]
    PlayerClosestToReticle,
    /// A text color that is hard to read on the default HUD.
    #[error("A text color that is hard to read on the default HUD")]
    DarkColor,
    /// `Wait(9999)` ends before a long game does.
    #[error("`Wait(9999)` ends before a long game does")]
    Wait9999,
    /// A duration of `9999` ends before a long game does.
    #[error("A duration of `9999` ends before a long game does")]
    Duration9999,
    /// `Wait Until` on a condition that is not a boolean.
    #[error("`Wait Until` on a condition that is not a boolean")]
    WaitUntil,
    /// `Apply Impulse` with the legacy `Cancel Contrary Motion`.
    #[error("`Apply Impulse` with the legacy `Cancel Contrary Motion`")]
    CancelContraryMotion,
    /// An operation on operand types the game coerces in surprising ways, e.g. a number ÷ a vector.
    #[error(
        "An operation on operand types the game coerces in surprising ways, e.g. a number ÷ a vector"
    )]
    TypeCheck,
    /// `Value In Array` on a number, which is always 0.
    #[error("`Value In Array` on a number, which is always 0")]
    ArrayIndexNumber,
    /// An array in a string, of which only the first element shows.
    #[error("An array in a string, of which only the first element shows")]
    ArrayInString,
    /// `.fullwidth()` text with characters that have no fullwidth form.
    #[error("`.fullwidth()` text with characters that have no fullwidth form")]
    NotTotalFullwidth,
    /// A string that contains `{digits}`, which the game reads as a placeholder.
    #[error("a string that contains `{{digits}}`, which the game reads as a placeholder")]
    PlaceholderLookalike,
    /// A text tag (`<fg…>`, `<tx…>`) while tags are disabled.
    #[error("A text tag (`<fg…>`, `<tx…>`) while tags are disabled")]
    Tags,
    /// A `<tx…>` tag whose texture id is not 14 hex digits after `C`.
    #[error("A `<tx…>` tag whose texture id is not 14 hex digits after `C`")]
    MalformattedTx,
    /// The extensions cost more points than the settings leave.
    #[error("The extensions cost more points than the settings leave")]
    ExtensionPoints,
    /// A combo workshop setting whose default is not its first option, which then cannot be selected.
    #[error(
        "A combo workshop setting whose default is not its first option, which then cannot be selected"
    )]
    WorkshopSettingEnumDefault,
    /// Debug panels left out, or shortened, so that the mod stays under the element limit.
    #[error("Debug panels left out, or shortened, so that the mod stays under the element limit")]
    DebugBudget,
}

impl Warning {
    /// The kebab-case code used in diagnostics.
    #[must_use]
    pub fn code(self) -> &'static str {
        match self {
            Self::ElementLimit => "element-limit",
            Self::CensoredVarName => "censored-var-name",
            Self::BreakOutsideLoop => "break-outside-loop",
            Self::ContinueOutsideLoop => "continue-outside-loop",
            Self::MismatchedSubroutineEvent => "mismatched-subroutine-event",
            Self::StartRuleCrash => "start-rule-crash",
            Self::ChasedVarInFor => "chased-var-in-for",
            Self::Ow2RuleConditionChase => "ow2-rule-condition-chase",
            Self::Chase9999 => "chase-9999",
            Self::ClosestPlayer => "closest-player",
            Self::FarthestPlayer => "farthest-player",
            Self::PlayerClosestToReticle => "player-closest-to-reticle",
            Self::DarkColor => "dark-color",
            Self::Wait9999 => "wait-9999",
            Self::Duration9999 => "duration-9999",
            Self::WaitUntil => "wait-until",
            Self::CancelContraryMotion => "cancel-contrary-motion",
            Self::TypeCheck => "type-check",
            Self::ArrayIndexNumber => "array-index-number",
            Self::ArrayInString => "array-in-string",
            Self::NotTotalFullwidth => "not-total-fullwidth",
            Self::PlaceholderLookalike => "placeholder-lookalike",
            Self::Tags => "tags",
            Self::MalformattedTx => "malformatted-tx",
            Self::ExtensionPoints => "extension-points",
            Self::WorkshopSettingEnumDefault => "workshop-setting-enum-default",
            Self::DebugBudget => "debug-budget",
        }
    }

    /// Every kind, for tests and documentation.
    pub const ALL: &'static [Self] = &[
        Self::ElementLimit,
        Self::CensoredVarName,
        Self::BreakOutsideLoop,
        Self::ContinueOutsideLoop,
        Self::MismatchedSubroutineEvent,
        Self::StartRuleCrash,
        Self::ChasedVarInFor,
        Self::Ow2RuleConditionChase,
        Self::Chase9999,
        Self::ClosestPlayer,
        Self::FarthestPlayer,
        Self::PlayerClosestToReticle,
        Self::DarkColor,
        Self::Wait9999,
        Self::Duration9999,
        Self::WaitUntil,
        Self::CancelContraryMotion,
        Self::TypeCheck,
        Self::ArrayIndexNumber,
        Self::ArrayInString,
        Self::NotTotalFullwidth,
        Self::PlaceholderLookalike,
        Self::Tags,
        Self::MalformattedTx,
        Self::ExtensionPoints,
        Self::WorkshopSettingEnumDefault,
        Self::DebugBudget,
    ];
}
