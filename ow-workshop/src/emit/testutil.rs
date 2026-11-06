//! Helpers for printer tests.

use ow_workshop_data::Lang;

use super::{Cx, expr::write_expr, stmt::write_stmt};
use crate::ir::{Expr, ExprKind, Program, Stmt, StmtKind, SubDecl, VarDecl, VarId};
use crate::options::CompileOptions;

/// A program with globals `A` and `foo`, player variable `score` and subroutines `Sub0`, `bar`.
pub(super) fn program() -> Program {
    let mut program = Program::default();
    program.globals.declare(VarDecl::new("A"));
    program.globals.declare(VarDecl::new("foo"));
    program.players.declare(VarDecl::new("score"));
    program.subroutines.declare(SubDecl::new("Sub0"));
    program.subroutines.declare(SubDecl::new("bar"));
    program
}

pub(super) fn global(n: u32) -> Expr {
    Expr::new(ExprKind::GlobalVar(VarId(n)), None)
}

pub(super) fn player_var(player: Expr, n: u32) -> Expr {
    Expr::new(
        ExprKind::PlayerVar {
            player,
            var: VarId(n),
        },
        None,
    )
}

/// Prints an expression in `lang`.
pub(super) fn expr_in(lang: Lang, expr: &Expr) -> String {
    let program = program();
    let options = CompileOptions::default();
    let cx = Cx {
        lang,
        program: &program,
        options: &options,
    };
    let mut out = String::new();
    write_expr(&mut out, cx, expr).unwrap();
    out
}

/// Prints a statement in `lang`.
pub(super) fn stmt_in(lang: Lang, kind: StmtKind) -> String {
    let program = program();
    let options = CompileOptions::default();
    let cx = Cx {
        lang,
        program: &program,
        options: &options,
    };
    let mut out = String::new();
    let stmt = Stmt {
        kind,
        loc: None,
        comment: None,
    };
    write_stmt(&mut out, cx, &stmt).unwrap();
    out
}

pub(super) fn text(expr: &Expr) -> String {
    expr_in(Lang::EnUs, expr)
}

pub(super) fn stmt(kind: StmtKind) -> String {
    stmt_in(Lang::EnUs, kind)
}
