//! The compile passes: generated rules → checks → lower → optimize → labels → flatten → count → emit.

use crate::ir::ErrorKind;
use crate::ir::{Diagnostics, Program};
use crate::lower::{self, Ctx};
use crate::output::CompileOutput;
use crate::suppress::Suppression;
use crate::translations::Translations;
use ow_workshop_data::Lang;

/// The first line of a debug build, so a pasted debug build is recognizable.
pub(crate) const DEBUG_HEADER: &str = "// ow-workshop debug build\n";

/// Runs every pass over a recorded program. Diagnostics come back deduplicated and ordered.
pub(crate) fn run(
    program: Program,
    diagnostics: Diagnostics,
    translations: Option<Translations>,
    lang: Lang,
) -> Result<CompileOutput, Diagnostics> {
    match compile(program, diagnostics, translations, lang) {
        Ok(mut output) => {
            let mut warnings = Diagnostics(std::mem::take(&mut output.warnings));
            warnings.finish();
            output.warnings = warnings.0;
            Ok(output)
        }
        Err(mut found) => {
            found.finish();
            Err(found)
        }
    }
}

fn compile(
    mut program: Program,
    mut diagnostics: Diagnostics,
    translations: Option<Translations>,
    lang: Lang,
) -> Result<CompileOutput, Diagnostics> {
    if let Err(found) = program.validate() {
        diagnostics.0.extend(found.0);
    }
    crate::settings::validate(&mut program.settings, &program.options, &mut diagnostics);
    crate::check::check_labels(&program.rules, &mut diagnostics);
    crate::check::check_nested_lambdas(&program.rules, &mut diagnostics);
    crate::check::check_multi_dim_indices(&program.rules, &mut diagnostics);
    let suppression = Suppression::new(&program.options, &program.rules);
    crate::check::check_placeholder_lookalikes(&program.rules, &suppression, &mut diagnostics);
    if diagnostics.has_errors() {
        return Err(diagnostics);
    }
    program.translations = translations;
    let options = program.options.clone();
    let mut ctx = Ctx::new(&options, lang, &mut diagnostics, &program);
    if let Err(found) = lower::run_passes(lower::PIPELINE, &mut program, &mut ctx) {
        diagnostics.0.extend(found.0);
        return Err(diagnostics);
    }
    for allocation in [
        program.globals.allocate(),
        program.players.allocate(),
        program.subroutines.allocate(),
    ] {
        match allocation {
            Ok(allocated) => diagnostics.0.extend(
                allocated
                    .warnings
                    .0
                    .into_iter()
                    .filter(|d| !suppression.drops(d)),
            ),
            Err(found) => {
                diagnostics.0.extend(found.0);
                return Err(diagnostics);
            }
        }
    }
    let counts = crate::count::count(&program, lang);
    if counts.over_limit() {
        diagnostics.error(
            ErrorKind::TooManyElements,
            format!(
                "the mod uses {} elements; the game allows {}",
                counts.total,
                crate::count::ELEMENT_LIMIT
            ),
            None,
        );
        return Err(diagnostics);
    }
    let element_count = counts.total;
    let debug = program.options.debug_element_count.then_some(&counts);
    let Ok(mut code) = crate::emit::emit(&program, lang, debug) else {
        diagnostics.error(
            ErrorKind::EmitFailed,
            "the program still contains a node that lowering should have removed".to_owned(),
            None,
        );
        return Err(diagnostics);
    };
    if options.debug.resolve().is_some() && !code.is_empty() {
        code.insert_str(0, DEBUG_HEADER);
    }
    Ok(CompileOutput {
        code,
        warnings: diagnostics.0,
        element_count,
        catalog: program.translations.take().map(|t| t.catalog),
    })
}
