use std::error::Error;
use std::fmt::{self, Display};

use crate::SettingsError;

/// Why a mod's script cannot be built.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum BuildError {
    /// The settings are inconsistent or name something unknown.
    Settings(SettingsError),
    /// A rule or subroutine calls a subroutine the mod does not declare.
    UndeclaredSubroutine(UndeclaredSubroutine),
}

impl Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Settings(error) => error.fmt(f),
            Self::UndeclaredSubroutine(error) => error.fmt(f),
        }
    }
}

impl Error for BuildError {}

impl From<SettingsError> for BuildError {
    fn from(error: SettingsError) -> Self {
        Self::Settings(error)
    }
}

impl From<UndeclaredSubroutine> for BuildError {
    fn from(error: UndeclaredSubroutine) -> Self {
        Self::UndeclaredSubroutine(error)
    }
}

/// A call to a subroutine missing from [`Workshop::subroutines`](crate::Workshop::subroutines).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UndeclaredSubroutine {
    /// The subroutine called.
    pub subroutine: String,
    /// The title of the rule that calls it.
    pub rule: String,
}

impl Display for UndeclaredSubroutine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rule \"{}\" calls subroutine `{}`, which the mod does not declare in `subroutines()`",
            self.rule, self.subroutine
        )
    }
}

impl Error for UndeclaredSubroutine {}
