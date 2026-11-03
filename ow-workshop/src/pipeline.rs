//! The compile passes: lower → optimize → flatten → count → emit.

use crate::ir::{Diagnostics, Program};
use crate::output::CompileOutput;
use crate::translations::Translations;
use ow_workshop_data::Lang;

/// Runs every pass over a recorded program.
pub(crate) fn run(
    program: Program,
    mut diagnostics: Diagnostics,
    translations: Option<Translations>,
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
    let program = lower(program, translations);
    let program = optimize(program);
    let program = flatten(program);
    let element_count = count(&program);
    let code = emit(&program, lang);
    Ok(CompileOutput {
        code,
        warnings: diagnostics.0,
        element_count,
        catalog: None,
    })
}

// The passes below are replaced by the lowering, optimizer and emitter epics.

fn lower(program: Program, _translations: Option<Translations>) -> Program {
    program
}

fn optimize(program: Program) -> Program {
    program
}

fn flatten(program: Program) -> Program {
    program
}

fn count(_program: &Program) -> usize {
    0
}

fn emit(_program: &Program, _lang: Lang) -> String {
    String::new()
}
