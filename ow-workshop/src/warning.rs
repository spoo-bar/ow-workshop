//! Warnings a rule can suppress.

/// A kind of warning, identified by a stable code.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Warning {
    /// `break_()` outside a loop or switch.
    BreakOutsideLoop,
    /// `continue_()` outside a loop.
    ContinueOutsideLoop,
    /// A subroutine reads event values the calling rule's event does not provide.
    MismatchedSubroutineEvent,
    /// `Start Rule(.., Restart Rule)` on a subroutine that waits, which eventually crashes the server.
    StartRuleCrash,
    /// A chased variable is also a `For` loop variable, so the loop never runs.
    ChasedVarInFor,
    /// A rule condition reads a variable that is chased, which breaks re-triggering in Overwatch 2.
    Ow2RuleConditionChase,
    /// `Chase … At Rate` towards `9999`, which a long game can exceed.
    Chase9999,
    /// `Closest Player To` also finds dead and unspawned players.
    ClosestPlayer,
    /// `Farthest Player From` also finds dead and unspawned players.
    FarthestPlayer,
    /// `Player Closest To Reticle` also finds dead and unspawned players.
    PlayerClosestToReticle,
    /// A text color that is hard to read on the default HUD.
    DarkColor,
    /// `Wait(9999)` ends before a long game does.
    Wait9999,
    /// A duration of `9999` ends before a long game does.
    Duration9999,
    /// `Wait Until` on a condition that is not a boolean.
    WaitUntil,
    /// `Apply Impulse` with the legacy `Cancel Contrary Motion`.
    CancelContraryMotion,
    /// An operation on operand types the game coerces in surprising ways, e.g. a number ÷ a vector.
    TypeCheck,
    /// `Value In Array` on a number, which is always 0.
    ArrayIndexNumber,
    /// An array in a string, of which only the first element shows.
    ArrayInString,
    /// `.fullwidth()` text with characters that have no fullwidth form.
    NotTotalFullwidth,
    /// A text tag (`<fg…>`, `<tx…>`) while tags are disabled.
    Tags,
    /// A `<tx…>` tag whose texture id is not 14 hex digits after `C`.
    MalformattedTx,
    /// The extensions cost more points than the settings leave.
    ExtensionPoints,
}

impl Warning {
    /// The kebab-case code used in diagnostics.
    #[must_use]
    pub fn code(self) -> &'static str {
        match self {
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
            Self::Tags => "tags",
            Self::MalformattedTx => "malformatted-tx",
            Self::ExtensionPoints => "extension-points",
        }
    }
}
