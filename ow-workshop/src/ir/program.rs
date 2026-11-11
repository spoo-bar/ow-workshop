//! The whole program.

use super::{Diagnostics, Rule, RuleError, Severity, Subs, Vars};
use crate::CompileOptions;
use crate::ir::ErrorKind;
use crate::settings::Settings;
use crate::translations::Translations;

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
    /// Language-dependent strings, when the mod has translations.
    pub(crate) translations: Option<Translations>,
    /// How to show variables in a debug build: the variable structs' fields, then whatever the
    /// mod registered with `Builder::debug`.
    pub(crate) debug_fields: Vec<crate::workshop_debug::DebugField>,
    /// How much of the debug panels to generate; the pipeline lowers it to fit the element limit.
    pub(crate) debug_panels: crate::lower::debug_panels::PanelPlan,
}

impl Program {
    /// Rejects malformed rules.
    pub(crate) fn validate(&self) -> Result<(), Diagnostics> {
        let mut diags = Diagnostics::default();
        for rule in &self.rules {
            let Err(err) = rule.validate() else { continue };
            let (code, message) = match err {
                RuleError::FilterNotAllowed => (
                    ErrorKind::FilterNotAllowed,
                    format!(
                        "rule `{}`: only player events take a team/player filter",
                        rule.name
                    ),
                ),
                RuleError::SubroutineHasConditions => (
                    ErrorKind::SubroutineConditions,
                    format!("rule `{}`: subroutines cannot have conditions", rule.name),
                ),
                RuleError::SlotOutOfRange(slot) => (
                    ErrorKind::SlotOutOfRange,
                    format!("rule `{}`: slot {slot} is not in 0..=11", rule.name),
                ),
            };
            diags.error(code, message, rule.location);
        }
        if diags.has_errors() {
            Err(diags)
        } else {
            Ok(())
        }
    }
}
