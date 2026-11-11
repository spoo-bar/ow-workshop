//! Debug-only statements: removed in a release build, unwrapped in a debug build.
//!
//! This is the first pass, so what it removes never reaches variable usage, the optimizer or
//! label resolution: jumps over a removed statement have the distance they would have had if
//! it had never been written.

use super::Ctx;
use super::visit::fold_block;
use crate::ir::{Diagnostics, Program, StmtKind};

/// Whether debug-only statements stay in this compilation.
fn keeps(ctx: &Ctx<'_>) -> bool {
    ctx.options
        .debug
        .resolve()
        .is_some_and(|debug| debug.keep_debug_calls)
}

/// Replaces every `DebugOnly` by its statements, or removes it.
pub(crate) fn strip(program: &mut Program, ctx: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    let keep = keeps(ctx);
    for rule in &mut program.rules {
        fold_block(&mut rule.body, &mut |stmt| match stmt.kind {
            StmtKind::DebugOnly(block) if keep => block.stmts,
            StmtKind::DebugOnly(_) => Vec::new(),
            _ => vec![stmt],
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lower::testutil::{action, block, if_, kinds, options, program, run};
    use crate::options::{CompileOptions, DebugMode, DebugOptions, OptLevel};

    fn compiled(debug: DebugMode) -> Program {
        let nested = StmtKind::DebugOnly(block(vec![action(), StmtKind::DebugOnly(block(vec![]))]));
        let mut program = program(vec![
            action(),
            StmtKind::DebugOnly(block(vec![action(), action()])),
            if_(
                crate::lower::testutil::boolean(true),
                vec![StmtKind::DebugOnly(block(vec![action()])), action()],
            ),
            nested,
        ]);
        let options = CompileOptions {
            debug,
            ..options(OptLevel::Off)
        };
        run(strip, &mut program, &options).expect("runs");
        program
    }

    fn count(program: &Program) -> usize {
        let mut seen = 0;
        crate::lower::visit::walk_block(&program.rules[0].body, &mut |s| {
            seen += usize::from(matches!(s.kind, StmtKind::Action { .. }));
        });
        seen
    }

    fn has_debug_only(program: &Program) -> bool {
        let mut found = false;
        crate::lower::visit::walk_block(&program.rules[0].body, &mut |s| {
            found |= matches!(s.kind, StmtKind::DebugOnly(_));
        });
        found
    }

    #[test]
    fn off_removes_them_at_every_depth() {
        let program = compiled(DebugMode::Off);
        assert!(!has_debug_only(&program));
        // One action at the top, one in the `If`.
        assert_eq!(count(&program), 2);
        assert_eq!(kinds(&program).len(), 2);
    }

    #[test]
    fn on_unwraps_them() {
        let program = compiled(DebugMode::On(DebugOptions::default()));
        assert!(!has_debug_only(&program));
        assert_eq!(count(&program), 1 + 2 + 2 + 1);
    }

    #[test]
    fn on_without_keep_debug_calls_removes_them() {
        let options = DebugOptions {
            keep_debug_calls: false,
            ..DebugOptions::default()
        };
        let program = compiled(DebugMode::On(options));
        assert!(!has_debug_only(&program));
        assert_eq!(count(&program), 2);
    }
}
