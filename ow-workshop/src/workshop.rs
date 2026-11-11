//! The `Workshop` trait users implement to define a mod.

use crate::builder::Builder;
use crate::ir::{Diagnostics, Program};
use crate::options::{CompileOptions, DebugMode};
use crate::output::CompileOutput;
use crate::pipeline;
use crate::settings::{Main, Settings};
use crate::translations::Translations;
use crate::vars::{GlobalVars, PlayerVars, VarRegistry};
use crate::workshop_debug::WorkshopDebug as _;
use ow_workshop_data::Lang;

/// A mod: implement this to define rules, variables and settings.
pub trait Workshop {
    /// Global variable handles.
    type Globals: GlobalVars;
    /// Player variable handles.
    type Players: PlayerVars;

    /// Records the rules and subroutines on `ws`, in output order.
    fn build(&self, ws: &mut Builder, globals: &Self::Globals, players: &Self::Players);

    /// The name of the mode, shown in the game's mode list. Required.
    fn mode_name(&self) -> &'static str;

    /// The description of the mode. Required.
    fn description(&self) -> &'static str;

    /// Custom game settings.
    ///
    /// Required: the game rejects code without a `settings` block, so the settings must enable at
    /// least one game mode, and every enabled mode needs a map.
    fn settings(&self) -> Settings;

    /// Compile options.
    fn options(&self) -> CompileOptions {
        CompileOptions::default()
    }

    /// Translations of language-dependent strings.
    fn translations(&self) -> Option<Translations> {
        None
    }
}

/// Compiles any [`Workshop`].
pub trait WorkshopExt: Workshop {
    /// Builds the mod and returns its workshop code in `lang`.
    fn compile(&self, lang: Lang) -> Result<CompileOutput, Diagnostics>;

    /// [`compile`](Self::compile) with the debug mode forced to `debug` instead of the one in
    /// [`Workshop::options`]; `None` keeps it. This is what `main!`'s `--debug` and
    /// `--no-debug` use.
    fn compile_with_debug(
        &self,
        lang: Lang,
        debug: Option<DebugMode>,
    ) -> Result<CompileOutput, Diagnostics>;
}

impl<W: Workshop> WorkshopExt for W {
    fn compile(&self, lang: Lang) -> Result<CompileOutput, Diagnostics> {
        self.compile_with_debug(lang, None)
    }

    fn compile_with_debug(
        &self,
        lang: Lang,
        debug: Option<DebugMode>,
    ) -> Result<CompileOutput, Diagnostics> {
        let mut global_reg = VarRegistry::default();
        let mut player_reg = VarRegistry::default();
        let globals = W::Globals::declare(&mut global_reg);
        let players = W::Players::declare(&mut player_reg);

        let mut settings = self.settings();
        if !settings.unchecked {
            settings.main = Some(Main {
                mode_name: self.mode_name().to_owned(),
                description: self.description().to_owned(),
            });
        }
        let mut options = self.options();
        if let Some(debug) = debug {
            options.debug = debug;
        }
        let mut ws = Builder::with_template(options.rule_name_template).with_tags(options.tags);
        self.build(&mut ws, &globals, &players);

        let mut debug_fields = globals.debug_fields();
        debug_fields.extend(players.debug_fields());
        let program = Program {
            debug_fields,
            globals: global_reg.vars,
            players: player_reg.vars,
            settings,
            options,
            ..Program::default()
        };
        let (program, diagnostics) = ws.finish(program);
        pipeline::run(program, diagnostics, self.translations(), lang)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Empty;

    impl Workshop for Empty {
        fn mode_name(&self) -> &'static str {
            "Test"
        }

        fn description(&self) -> &'static str {
            "Test mode"
        }

        fn settings(&self) -> crate::settings::Settings {
            crate::settings::Settings::unchecked()
        }

        type Globals = ();
        type Players = ();

        fn build(&self, _: &mut Builder, (): &(), (): &()) {}
    }

    #[test]
    fn empty_mod_compiles_to_empty_string() {
        for debug in [DebugMode::Off, DebugMode::Auto] {
            let out = Empty
                .compile_with_debug(Lang::EnUs, Some(debug))
                .expect("compiles");
            assert_eq!(out.code, "", "{debug:?}");
            assert_eq!(out.warnings, vec![]);
            assert_eq!(out.element_count, 0);
        }
    }

    struct Configured;

    impl Workshop for Configured {
        fn mode_name(&self) -> &'static str {
            "Test"
        }

        fn description(&self) -> &'static str {
            "Test mode"
        }

        fn settings(&self) -> crate::settings::Settings {
            crate::settings::Settings::unchecked()
        }

        type Globals = ();
        type Players = ();

        fn build(&self, _: &mut Builder, (): &(), (): &()) {}

        fn options(&self) -> CompileOptions {
            CompileOptions {
                debug_element_count: true,
                ..CompileOptions::default()
            }
        }
    }

    #[test]
    fn options_default_names() {
        let options = CompileOptions::default();
        assert_eq!(options.global_init_rule_name, "Initialize global variables");
        assert_eq!(options.player_init_rule_name, "Initialize player variables");
        assert!(Configured.options().debug_element_count);
    }
}
