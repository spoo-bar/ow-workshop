//! Compile options.

use ow_workshop_data::Button;

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

/// What a debug build adds to the output; see [`DebugMode`].
///
/// Construct it with `DebugOptions::default()` and set the fields you want to change.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[allow(clippy::struct_excessive_bools)]
pub struct DebugOptions {
    /// Show a panel with the global variables on screen.
    pub hud: bool,
    /// Show a panel with each player's variables to that player.
    pub player_hud: bool,
    /// Return a transpile-time log on [`CompileOutput::debug_log`](crate::CompileOutput).
    pub log: bool,
    /// Keep the calls that exist only for debugging (`debug!`, `print`); a release build
    /// strips them.
    pub keep_debug_calls: bool,
    /// When set, the panels show only while this button is held.
    pub toggle: Option<Button>,
}

impl Default for DebugOptions {
    fn default() -> Self {
        Self {
            hud: true,
            player_hud: true,
            log: true,
            keep_debug_calls: true,
            toggle: None,
        }
    }
}

/// Whether a build adds debug output.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum DebugMode {
    /// Debug output when the crate is built without optimizations (`cargo run`), none with
    /// `cargo run --release`. See [`DebugMode::resolve`].
    #[default]
    Auto,
    /// Always add debug output.
    On(DebugOptions),
    /// Never add debug output: the output is exactly the release output.
    Off,
}

impl DebugMode {
    /// The options in effect, or `None` when debug is off.
    ///
    /// `Auto` follows `cfg!(debug_assertions)` as `ow-workshop` itself was compiled. Dependencies
    /// are built with the profile of the binary that uses them, so that is the profile of the
    /// user's `cargo run`: on in a dev build, off with `--release`. A profile that changes
    /// `debug-assertions` follows that; force a mode with [`DebugMode::On`] or
    /// [`DebugMode::Off`] if it should not.
    #[must_use]
    pub fn resolve(self) -> Option<DebugOptions> {
        self.resolve_with(cfg!(debug_assertions))
    }

    /// [`resolve`](Self::resolve) for a given value of `debug_assertions`.
    pub(crate) fn resolve_with(self, debug_assertions: bool) -> Option<DebugOptions> {
        match self {
            Self::Auto => debug_assertions.then(DebugOptions::default),
            Self::On(options) => Some(options),
            Self::Off => None,
        }
    }
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
    /// Keep the compression alphabet in the internal global `__compressionAlphabet__`, set once
    /// by the global init rule, instead of repeating it in every decoding expression.
    pub compression_alphabet_variable: bool,
    /// Name of the rule that initializes global variables.
    pub global_init_rule_name: String,
    /// Name of the rule that initializes player variables.
    pub player_init_rule_name: String,
    /// Warnings to suppress everywhere.
    pub suppress: WarningSet,
    /// Enable `<fg…>` / `<tx…>` text tags (always on when a `Texture` is used).
    pub tags: bool,
    /// Builds the final rule name from its name and group prefix.
    pub rule_name_template: fn(&RuleMeta) -> String,
    /// Debug output; follows the build profile by default.
    pub debug: DebugMode,
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
            compression_alphabet_variable: false,
            global_init_rule_name: "Initialize global variables".to_owned(),
            player_init_rule_name: "Initialize player variables".to_owned(),
            suppress: WarningSet::default(),
            tags: false,
            rule_name_template: default_rule_name,
            debug: DebugMode::default(),
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
            && self.compression_alphabet_variable == other.compression_alphabet_variable
            && self.global_init_rule_name == other.global_init_rule_name
            && self.player_init_rule_name == other.player_init_rule_name
            && self.suppress == other.suppress
            && self.tags == other.tags
            && self.debug == other.debug
            && std::ptr::fn_addr_eq(self.rule_name_template, other.rule_name_template)
    }
}

impl Eq for CompileOptions {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_follows_debug_assertions() {
        assert_eq!(
            DebugMode::Auto.resolve_with(true),
            Some(DebugOptions::default())
        );
        assert_eq!(DebugMode::Auto.resolve_with(false), None);
    }

    #[test]
    fn auto_resolves_like_the_build_profile() {
        assert_eq!(DebugMode::Auto.resolve().is_some(), cfg!(debug_assertions));
    }

    #[test]
    fn forced_modes_ignore_the_profile() {
        let custom = DebugOptions {
            log: false,
            toggle: Some(Button::Interact),
            ..DebugOptions::default()
        };
        for assertions in [true, false] {
            assert_eq!(DebugMode::Off.resolve_with(assertions), None);
            assert_eq!(DebugMode::On(custom).resolve_with(assertions), Some(custom));
        }
    }

    #[test]
    fn defaults() {
        assert_eq!(DebugMode::default(), DebugMode::Auto);
        assert_eq!(CompileOptions::default().debug, DebugMode::Auto);
        let options = DebugOptions::default();
        assert!(options.hud && options.player_hud && options.log && options.keep_debug_calls);
        assert_eq!(options.toggle, None);
    }
}
