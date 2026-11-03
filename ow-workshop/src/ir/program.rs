//! The whole program.

use super::{Diagnostics, Rule, RuleError, Severity, Subs, Vars};

/// Custom game settings; filled in by the settings epic.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Settings;

/// Compile options; filled in by the options epic.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct CompileOptions;

/// Everything the passes transform.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Program {
    /// Rules, in output order.
    pub(crate) rules: Vec<Rule>,
    /// Global variables.
    pub(crate) globals: Vars,
    /// Player variables.
    pub(crate) players: Vars,
    /// Subroutines.
    pub(crate) subroutines: Subs,
    /// Custom game settings.
    pub(crate) settings: Settings,
    /// Compile options.
    pub(crate) options: CompileOptions,
}

impl Program {
    /// Rejects malformed rules.
    pub(crate) fn validate(&self) -> Result<(), Diagnostics> {
        let mut diags = Diagnostics::default();
        for rule in &self.rules {
            let Err(err) = rule.validate() else { continue };
            let (code, message) = match err {
                RuleError::FilterNotAllowed => (
                    "filter-not-allowed",
                    format!(
                        "rule `{}`: only player events take a team/player filter",
                        rule.name
                    ),
                ),
                RuleError::SubroutineHasConditions => (
                    "subroutine-conditions",
                    format!("rule `{}`: subroutines cannot have conditions", rule.name),
                ),
                RuleError::SlotOutOfRange(slot) => (
                    "slot-out-of-range",
                    format!("rule `{}`: slot {slot} is not in 0..=11", rule.name),
                ),
            };
            diags.push(Severity::Error, code, message, rule.location);
        }
        if diags.has_errors() {
            Err(diags)
        } else {
            Ok(())
        }
    }
}
