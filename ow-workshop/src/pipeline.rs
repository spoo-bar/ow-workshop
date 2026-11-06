//! The compile passes: generated rules → checks → lower → optimize → labels → flatten → count → emit.

use crate::ir::{Diagnostics, Program, Severity};
use crate::lower::{self, Ctx};
use crate::output::CompileOutput;
use crate::translations::Translations;
use ow_workshop_data::Lang;

/// Runs every pass over a recorded program.
pub(crate) fn run(
    program: Program,
    mut diagnostics: Diagnostics,
    _translations: Option<Translations>,
    lang: Lang,
) -> Result<CompileOutput, Diagnostics> {
    if let Err(found) = program.validate() {
        diagnostics.0.extend(found.0);
    }
    crate::check::check_labels(&program.rules, &mut diagnostics);
    crate::check::check_nested_lambdas(&program.rules, &mut diagnostics);
    crate::check::check_multi_dim_indices(&program.rules, &mut diagnostics);
    if diagnostics.has_errors() {
        return Err(diagnostics);
    }
    let mut program = program;
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
            Ok(allocated) => diagnostics.0.extend(allocated.warnings.0),
            Err(found) => {
                diagnostics.0.extend(found.0);
                return Err(diagnostics);
            }
        }
    }
    let element_count = count(&program);
    let Ok(code) = crate::emit::emit(&program, lang) else {
        diagnostics.push(
            Severity::Error,
            "emit-failed",
            "the program still contains a node that lowering should have removed".to_owned(),
            None,
        );
        return Err(diagnostics);
    };
    Ok(CompileOutput {
        code,
        warnings: diagnostics.0,
        element_count,
        catalog: None,
    })
}

// Replaced by the counting pass.

fn count(_program: &Program) -> usize {
    0
}
