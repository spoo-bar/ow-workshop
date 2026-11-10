//! The compile passes: generated rules → checks → lower → optimize → labels → flatten → count → emit.

use crate::ir::{Diagnostic, ErrorKind, Kind};
use crate::ir::{Diagnostics, Program};
use crate::lower::debug_panels::{self, PanelPlan};
use crate::lower::{self, Ctx};
use crate::options::DebugOptions;
use crate::output::CompileOutput;
use crate::suppress::Suppression;
use crate::translations::Translations;
use crate::warning::Warning;
use crate::workshop_debug::Scope;
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

/// Compiles with the debug panels of a debug build cut down until the mod fits the element
/// limit: the player panel goes first, then the global fields from the last declared. A mod that
/// does not fit even without panels fails as it would in a release build.
fn compile(
    program: Program,
    diagnostics: Diagnostics,
    translations: Option<Translations>,
    lang: Lang,
) -> Result<CompileOutput, Diagnostics> {
    let Some(debug) = program.options.debug.resolve() else {
        return compile_once(program, diagnostics, translations, lang, PanelPlan::NONE);
    };
    let (globals, players) = debug_panels::sizes(&program, debug);
    // Plans from the most to the least shown; fewer panels never need more elements.
    let mut plans = vec![PanelPlan::default()];
    if players > 0 {
        plans.push(PanelPlan {
            players: false,
            globals,
        });
    }
    plans.extend((0..globals).rev().map(|globals| PanelPlan {
        players: false,
        globals,
    }));
    let attempt = |plan: PanelPlan| {
        compile_once(
            program.clone(),
            diagnostics.clone(),
            translations.clone(),
            lang,
            plan,
        )
    };
    let too_many = |found: &Diagnostics| {
        found
            .0
            .iter()
            .any(|d| d.kind == Kind::Error(ErrorKind::TooManyElements))
    };
    let first = attempt(plans[0]);
    if !matches!(&first, Err(found) if too_many(found)) || plans.len() == 1 {
        return first;
    }
    // The last plan shows nothing; if even that does not fit, the mod is the problem.
    let last = plans.len() - 1;
    let mut best = attempt(plans[last])?;
    // Invariant: `plans[good]` fits and `plans[bad]` does not.
    let (mut bad, mut good) = (0, last);
    while good - bad > 1 {
        let middle = bad + (good - bad) / 2;
        match attempt(plans[middle]) {
            Ok(output) => {
                (good, best) = (middle, output);
            }
            Err(found) if too_many(&found) => bad = middle,
            Err(found) => return Err(found),
        }
    }
    if !program.options.suppress.contains(Warning::DebugBudget) {
        let message = dropped_message(&program, debug, plans[good]);
        best.warnings
            .push(Diagnostic::new(Warning::DebugBudget, message, None));
    }
    Ok(best)
}

/// What `plan` left out of the panels of a debug build, for the `DebugBudget` warning.
fn dropped_message(program: &Program, debug: DebugOptions, plan: PanelPlan) -> String {
    let label = |scope| {
        program
            .debug_fields
            .iter()
            .filter(move |f| f.scope == scope)
    };
    let mut dropped = Vec::new();
    if !plan.players && debug.player_hud && label(Scope::Player).next().is_some() {
        let names: Vec<_> = label(Scope::Player).map(|f| f.label).collect();
        dropped.push(format!("the player panel ({})", names.join(", ")));
    }
    if debug.hud {
        let names: Vec<_> = label(Scope::Global)
            .skip(plan.globals)
            .map(|f| f.label)
            .collect();
        if !names.is_empty() {
            dropped.push(format!("global fields {}", names.join(", ")));
        }
    }
    format!(
        "the debug panels would push the mod over the {} element limit, so they leave out {}",
        crate::count::ELEMENT_LIMIT,
        dropped.join(" and ")
    )
}

fn compile_once(
    mut program: Program,
    mut diagnostics: Diagnostics,
    translations: Option<Translations>,
    lang: Lang,
    plan: PanelPlan,
) -> Result<CompileOutput, Diagnostics> {
    program.debug_panels = plan;
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
