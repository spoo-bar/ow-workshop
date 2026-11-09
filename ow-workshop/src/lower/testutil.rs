//! Helpers for hand-building programs in pass tests.

use ow_workshop_data::{ActionId, Lang};

use super::{Ctx, Pass};
use crate::ir::{
    Block, Diagnostics, Event, Expr, ExprKind, LabelId, Program, Rule, SkipTarget, Stmt, StmtKind,
};
use crate::options::{CompileOptions, OptLevel};

/// Gives every statement, condition and expression of `program` the same source location, so
/// suppression scopes (which are found by location) apply to the warnings it raises.
pub(crate) fn locate(program: &mut Program) {
    let loc = Some(crate::ir::Location {
        file: "mod.rs",
        line: 1,
        column: 1,
    });
    for rule in &mut program.rules {
        rule.location = loc;
        super::visit::fold_block(&mut rule.body, &mut |stmt| vec![Stmt { loc, ..stmt }]);
        let mut relocate = |e: Expr| Expr::new(e.kind().clone(), loc);
        super::visit::rewrite_block_exprs(&mut rule.body, &mut relocate);
        for cond in &mut rule.conditions {
            cond.expr = super::visit::rewrite_expr(&cond.expr, &mut relocate);
        }
    }
}

pub(crate) fn num(n: f64) -> Expr {
    Expr::new(ExprKind::Num(n), None)
}

pub(crate) fn boolean(b: bool) -> Expr {
    Expr::new(ExprKind::Bool(b), None)
}

pub(crate) fn expr(kind: ExprKind) -> Expr {
    Expr::new(kind, None)
}

pub(crate) fn block(kinds: Vec<StmtKind>) -> Block {
    Block {
        stmts: kinds
            .into_iter()
            .map(|kind| Stmt {
                kind,
                loc: None,
                comment: None,
                suppress: crate::ir::WarningSet::default(),
            })
            .collect(),
        ..Block::default()
    }
}

/// A single-argument-free action such as `Abort` stand-ins: `Wait(1)`.
pub(crate) fn action() -> StmtKind {
    StmtKind::Action {
        id: ActionId::Wait,
        args: vec![num(1.0)],
    }
}

pub(crate) fn skip_to(l: u32) -> StmtKind {
    StmtKind::Skip(SkipTarget::Label(LabelId(l)))
}

pub(crate) fn place(l: u32) -> StmtKind {
    StmtKind::Label(LabelId(l))
}

pub(crate) fn if_(cond: Expr, then: Vec<StmtKind>) -> StmtKind {
    StmtKind::If {
        cond,
        then: block(then),
        elifs: vec![],
        else_: None,
    }
}

pub(crate) fn program(body: Vec<StmtKind>) -> Program {
    Program {
        rules: vec![Rule::new("r", Event::OngoingGlobal, block(body))],
        ..Program::default()
    }
}

pub(crate) fn options(optimize: OptLevel) -> CompileOptions {
    CompileOptions {
        optimize,
        ..CompileOptions::default()
    }
}

/// Runs one pass; returns the warnings it reported, or its error.
pub(crate) fn run(
    pass: Pass,
    program: &mut Program,
    options: &CompileOptions,
) -> Result<Diagnostics, Diagnostics> {
    let mut diags = Diagnostics::default();
    let mut ctx = Ctx::new(options, Lang::EnUs, &mut diags, program);
    pass(program, &mut ctx)?;
    Ok(diags)
}

pub(crate) fn konst(enum_name: &'static str, key: &'static str) -> Expr {
    expr(ExprKind::Const(crate::ir::ConstRef { enum_name, key }))
}

pub(crate) fn call(id: ow_workshop_data::ValueId, args: Vec<Expr>) -> Expr {
    expr(ExprKind::Value { id, args })
}

pub(crate) fn null() -> Expr {
    expr(ExprKind::Null)
}

/// An action call with the arguments in `given` (by name) and `Null` for the rest.
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn act(id: ActionId, given: Vec<(&str, Expr)>) -> StmtKind {
    let args = id
        .signature()
        .args
        .iter()
        .map(|a| {
            given
                .iter()
                .find(|(name, _)| *name == a.name)
                .map_or_else(null, |(_, e)| e.clone())
        })
        .collect();
    StmtKind::Action { id, args }
}

/// The kinds of a rule's top-level statements.
pub(crate) fn kinds(program: &Program) -> Vec<&StmtKind> {
    program.rules[0]
        .body
        .stmts
        .iter()
        .map(|s| &s.kind)
        .collect()
}

pub(crate) fn codes(d: &Diagnostics) -> Vec<&'static str> {
    d.0.iter().map(crate::ir::Diagnostic::code).collect()
}

/// A compact, stable rendering of an expression for snapshot-style assertions.
pub(crate) fn show(e: &Expr) -> String {
    let list = |items: &[Expr]| items.iter().map(show).collect::<Vec<_>>().join(", ");
    match e.kind() {
        ExprKind::Num(n) => format!("{n}"),
        ExprKind::Bool(b) => format!("{b}"),
        ExprKind::Null => "Null".to_owned(),
        ExprKind::GlobalVar(v) => format!("g{}", v.0),
        ExprKind::PlayerVar { player, var } => format!("{}.p{}", show(player), var.0),
        ExprKind::Array(items) => format!("[{}]", list(items)),
        ExprKind::Value { id, args } => format!("{id:?}({})", list(args)),
        ExprKind::Index { array, index } => format!("{}[{}]", show(array), show(index)),
        ExprKind::Add(a, b) => format!("({} + {})", show(a), show(b)),
        ExprKind::Sub(a, b) => format!("({} - {})", show(a), show(b)),
        ExprKind::Mul(a, b) => format!("({} * {})", show(a), show(b)),
        ExprKind::Compare { op, lhs, rhs } => format!("({} {op:?} {})", show(lhs), show(rhs)),
        ExprKind::ArrayIndex => "#".to_owned(),
        ExprKind::ArrayElem => "@".to_owned(),
        ExprKind::Const(c) => format!("{}.{}", c.enum_name, c.key),
        ExprKind::Str(crate::ir::StrExpr::Custom { tokens, args }) => {
            let text: String = tokens
                .iter()
                .map(|t| match t {
                    crate::ir::StrToken::Arg(n) => format!("{{{n}}}"),
                    crate::ir::StrToken::Text(t) if t.chars().all(|c| c == '\u{2003}') => {
                        format!("<{}em>", t.chars().count())
                    }
                    crate::ir::StrToken::Text(t) => t.clone(),
                })
                .collect();
            format!("\"{text}\"({})", list(args))
        }
        ExprKind::And(a, b) => format!("({} && {})", show(a), show(b)),
        ExprKind::Not(a) => format!("!{}", show(a)),
        ExprKind::RuleCondition => "rule_condition".to_owned(),
        other => format!("{other:?}"),
    }
}
