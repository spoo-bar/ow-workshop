//! The `Workshop` trait users implement to define a mod.

use crate::builder::Builder;
use crate::ir::{Diagnostics, Program, Settings};
use crate::options::CompileOptions;
use crate::output::CompileOutput;
use crate::pipeline;
use crate::translations::Translations;
use crate::vars::{GlobalVars, PlayerVars, VarRegistry};
use ow_workshop_data::Lang;

/// A mod: implement this to define rules, variables and settings.
pub trait Workshop {
    /// Global variable handles.
    type Globals: GlobalVars;
    /// Player variable handles.
    type Players: PlayerVars;

    /// Records the rules and subroutines on `ws`, in output order.
    fn build(&self, ws: &mut Builder, globals: &Self::Globals, players: &Self::Players);

    /// Custom game settings.
    fn settings(&self) -> Settings {
        Settings::default()
    }

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
}

impl<W: Workshop> WorkshopExt for W {
    fn compile(&self, lang: Lang) -> Result<CompileOutput, Diagnostics> {
        let mut global_reg = VarRegistry::default();
        let mut player_reg = VarRegistry::default();
        let globals = W::Globals::declare(&mut global_reg);
        let players = W::Players::declare(&mut player_reg);

        let options = self.options();
        let mut ws = Builder::with_template(options.rule_name_template);
        self.build(&mut ws, &globals, &players);

        let program = Program {
            globals: global_reg.vars,
            players: player_reg.vars,
            settings: self.settings(),
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
        type Globals = ();
        type Players = ();

        fn build(&self, _: &mut Builder, (): &(), (): &()) {}
    }

    #[test]
    fn empty_mod_compiles_to_empty_string() {
        let out = Empty.compile(Lang::EnUs).expect("compiles");
        assert_eq!(out.code, "");
        assert_eq!(out.warnings, vec![]);
        assert_eq!(out.element_count, 0);
    }

    struct Configured;

    impl Workshop for Configured {
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
