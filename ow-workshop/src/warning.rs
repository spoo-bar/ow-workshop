//! Warnings a rule can suppress.

/// A kind of warning, identified by a stable code.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Warning {
    /// `break_()` outside a loop or switch.
    BreakOutsideLoop,
    /// `continue_()` outside a loop.
    ContinueOutsideLoop,
}

impl Warning {
    /// The kebab-case code used in diagnostics.
    #[must_use]
    pub fn code(self) -> &'static str {
        match self {
            Self::BreakOutsideLoop => "break-outside-loop",
            Self::ContinueOutsideLoop => "continue-outside-loop",
        }
    }
}
