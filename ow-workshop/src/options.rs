//! Compile options.

use crate::ir::WarningSet;
use crate::rule_name::{RuleMeta, default_rule_name};

/// How hard the optimizer works.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum OptLevel {
    /// No optimization.
    Off,
    /// Optimize for fewer runtime operations.
    #[default]
    Speed,
    /// Optimize for fewer elements.
    Size,
}

/// Where an empty string is read from instead of `Custom String("")`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum EmptyString {
    /// `Empty Array`.
    EmptyArray,
    /// A global variable, `__emptyString__`, initialized to `Char In String(Empty Array, Null)`.
    Variable,
}

/// Literals the optimizer may swap for a zero-argument value that is guaranteed equal in the
/// mod's game setup, saving an element each time. Only size mode uses these, and only the
/// mod's author knows whether each assumption holds, so all are off by default.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
#[allow(clippy::struct_excessive_bools)]
pub struct Replacements {
    /// `Team 1` → `Control Mode Scoring Team`: not a Control mode, or team 1 always scores.
    pub team1_by_control_scoring_team: bool,
    /// `0` → `Capture Percentage`: no capture point is in progress.
    pub zero_by_capture_percentage: bool,
    /// `0` → `Payload Progress Percentage`: there is no payload.
    pub zero_by_payload_progress: bool,
    /// `0` → `Is Match Complete`: the match never completes while rules run.
    pub zero_by_is_match_complete: bool,
    /// `1` → `Match Round`: single-round games.
    pub one_by_match_round: bool,
    /// `""` → `Empty Array` or a variable that holds the empty string.
    pub empty_string: Option<EmptyString>,
}

/// Options controlling a compilation.
#[derive(Clone, Debug)]
#[non_exhaustive]
#[allow(clippy::struct_excessive_bools)]
pub struct CompileOptions {
    /// Optimizer level.
    pub optimize: OptLevel,
    /// Skip rewrites that change behaviour under workshop type coercion (`A * 0` may be a
    /// vector, `A + 0` may not be a number).
    pub strict: bool,
    /// Whole-program size rewrites that may hurt readability; needs [`OptLevel::Size`].
    pub aggressive_size: bool,
    /// Literals swapped for equal zero-argument values, in size mode.
    pub replacements: Replacements,
    /// Leave out the `variables {}` block.
    pub exclude_variables_block: bool,
    /// Print the element count in a summary comment.
    pub debug_element_count: bool,
    /// Disable the workshop inspector in the emitted settings.
    pub disable_inspector: bool,
    /// Name of the rule that initializes global variables.
    pub global_init_rule_name: String,
    /// Name of the rule that initializes player variables.
    pub player_init_rule_name: String,
    /// Warnings to suppress everywhere.
    pub suppress: WarningSet,
    /// Emit tags around generated rules.
    pub tags: bool,
    /// Builds the final rule name from its name and group prefix.
    pub rule_name_template: fn(&RuleMeta) -> String,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            optimize: OptLevel::default(),
            strict: false,
            aggressive_size: false,
            replacements: Replacements::default(),
            exclude_variables_block: false,
            debug_element_count: false,
            disable_inspector: false,
            global_init_rule_name: "Initialize global variables".to_owned(),
            player_init_rule_name: "Initialize player variables".to_owned(),
            suppress: WarningSet::default(),
            tags: false,
            rule_name_template: default_rule_name,
        }
    }
}

impl PartialEq for CompileOptions {
    fn eq(&self, other: &Self) -> bool {
        self.optimize == other.optimize
            && self.strict == other.strict
            && self.aggressive_size == other.aggressive_size
            && self.replacements == other.replacements
            && self.exclude_variables_block == other.exclude_variables_block
            && self.debug_element_count == other.debug_element_count
            && self.disable_inspector == other.disable_inspector
            && self.global_init_rule_name == other.global_init_rule_name
            && self.player_init_rule_name == other.player_init_rule_name
            && self.suppress == other.suppress
            && self.tags == other.tags
            && std::ptr::fn_addr_eq(self.rule_name_template, other.rule_name_template)
    }
}

impl Eq for CompileOptions {}
