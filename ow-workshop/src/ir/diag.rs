//! Diagnostics: errors and warnings returned as data. The core never prints them.

use super::{ErrorKind, Location};
use crate::warning::Warning;

/// How serious a diagnostic is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    /// Compilation can continue.
    Warning,
    /// Compilation fails.
    Error,
}

/// What a diagnostic is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    /// Code that compiles but likely misbehaves in game.
    Warning(Warning),
    /// A problem that stops the compilation.
    Error(ErrorKind),
}

impl Kind {
    /// The stable kebab-case code, e.g. `chased-var-in-for`.
    #[must_use]
    pub fn code(self) -> &'static str {
        match self {
            Self::Warning(warning) => warning.code(),
            Self::Error(error) => error.code(),
        }
    }

    /// The severity this kind is reported with.
    #[must_use]
    pub fn severity(self) -> Severity {
        match self {
            Self::Warning(_) => Severity::Warning,
            Self::Error(_) => Severity::Error,
        }
    }
}

impl From<Warning> for Kind {
    fn from(warning: Warning) -> Self {
        Self::Warning(warning)
    }
}

impl From<ErrorKind> for Kind {
    fn from(error: ErrorKind) -> Self {
        Self::Error(error)
    }
}

/// One problem found in the user's mod.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    /// Warning or error.
    pub severity: Severity,
    /// Which warning or error this is.
    pub kind: Kind,
    /// Human-readable description.
    pub message: String,
    /// Where in the user's source, if known.
    pub location: Option<Location>,
}

impl Diagnostic {
    /// A diagnostic of `kind`, with the severity that kind implies.
    pub fn new(kind: impl Into<Kind>, message: String, location: Option<Location>) -> Self {
        let kind = kind.into();
        Self {
            severity: kind.severity(),
            kind,
            message,
            location,
        }
    }

    /// The stable kebab-case code of the kind.
    #[must_use]
    pub fn code(&self) -> &'static str {
        self.kind.code()
    }
}

/// A list of diagnostics.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Diagnostics(pub Vec<Diagnostic>);

impl Diagnostics {
    /// Records an error.
    pub fn error(&mut self, kind: ErrorKind, message: String, location: Option<Location>) {
        self.0.push(Diagnostic::new(kind, message, location));
    }

    /// Records a warning. Passes go through `Ctx::warn` so suppression applies.
    pub(crate) fn warning(
        &mut self,
        warning: Warning,
        message: String,
        location: Option<Location>,
    ) {
        self.0.push(Diagnostic::new(warning, message, location));
    }

    /// Drops repeated diagnostics (same kind, message and location), then orders the rest by
    /// location (file, line, column) and, within a location, by the order they were raised.
    /// Diagnostics without a location come last.
    pub(crate) fn finish(&mut self) {
        let mut seen = std::collections::HashSet::new();
        self.0
            .retain(|d| seen.insert((d.kind, d.message.clone(), d.location)));
        self.0.sort_by_key(|d| {
            (
                d.location.is_none(),
                d.location.map(|l| (l.file, l.line, l.column)),
            )
        });
    }

    /// Whether any diagnostic is an error.
    pub fn has_errors(&self) -> bool {
        self.0.iter().any(|d| d.severity == Severity::Error)
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Warning => "warning",
            Self::Error => "error",
        })
    }
}

/// `file:line:col: warning[code]: message`, without the location when it is unknown.
impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(loc) = &self.location {
            write!(f, "{loc}: ")?;
        }
        write!(f, "{}[{}]: {}", self.severity, self.code(), self.message)
    }
}

/// One diagnostic per line.
impl std::fmt::Display for Diagnostics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, diagnostic) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str("\n")?;
            }
            write!(f, "{diagnostic}")?;
        }
        Ok(())
    }
}

impl std::error::Error for Diagnostics {}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn at(file: &'static str, line: u32, column: u32) -> Location {
        Location { file, line, column }
    }

    #[test]
    fn a_warning_prints_like_a_compiler() {
        let diagnostic = Diagnostic::new(
            Warning::ChasedVarInFor,
            "the loop variable is chased".to_owned(),
            Some(at("src/mode.rs", 12, 5)),
        );
        assert_eq!(
            diagnostic.to_string(),
            "src/mode.rs:12:5: warning[chased-var-in-for]: the loop variable is chased"
        );
    }

    #[test]
    fn an_error_prints_like_a_compiler() {
        let diagnostic = Diagnostic::new(
            ErrorKind::DuplicateName,
            "variable `x` is declared twice".to_owned(),
            Some(at("src/mode.rs", 7, 1)),
        );
        assert_eq!(
            diagnostic.to_string(),
            "src/mode.rs:7:1: error[duplicate-name]: variable `x` is declared twice"
        );
    }

    #[test]
    fn severity_follows_the_kind() {
        assert_eq!(Kind::from(Warning::DarkColor).severity(), Severity::Warning);
        assert_eq!(Kind::from(ErrorKind::Ow1Only).severity(), Severity::Error);
    }

    #[test]
    fn codes_are_unique_kebab_case() {
        let codes: Vec<_> = Warning::ALL
            .iter()
            .map(|w| w.code())
            .chain(ErrorKind::ALL.iter().map(|e| e.code()))
            .collect();
        let unique: HashSet<_> = codes.iter().collect();
        assert_eq!(unique.len(), codes.len(), "a code is used twice");
        for code in codes {
            assert!(
                !code.is_empty()
                    && code.split('-').all(|part| !part.is_empty()
                        && part
                            .chars()
                            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())),
                "`{code}` is not kebab-case"
            );
        }
    }

    #[test]
    fn finish_dedups_and_orders_by_location() {
        let warn =
            |w: Warning, m: &str, loc: Option<Location>| Diagnostic::new(w, m.to_owned(), loc);
        let mut diags = Diagnostics(vec![
            warn(Warning::DarkColor, "no location", None),
            warn(Warning::DarkColor, "b", Some(at("b.rs", 1, 1))),
            warn(Warning::DarkColor, "a2", Some(at("a.rs", 9, 1))),
            warn(Warning::DarkColor, "a1", Some(at("a.rs", 2, 7))),
            warn(Warning::DarkColor, "a1", Some(at("a.rs", 2, 7))),
            // Same place, different message: kept, in raise order.
            warn(Warning::DarkColor, "a1 again", Some(at("a.rs", 2, 7))),
        ]);
        diags.finish();
        let messages: Vec<_> = diags.0.iter().map(|d| d.message.as_str()).collect();
        assert_eq!(messages, ["a1", "a1 again", "a2", "b", "no location"]);
    }
}
