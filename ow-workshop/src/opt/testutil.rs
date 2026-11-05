//! Helpers for testing expression rewrites.

use ow_workshop_data::{Lang, ValueId};

use super::{Env, Scope, optimize_expr};
use crate::ir::{CmpOp, Diagnostics, Expr, ExprKind, Program, VarId, WarningSet};
use crate::lower::Ctx;
use crate::lower::testutil::num;
use crate::options::{CompileOptions, OptLevel};

pub(crate) fn v3(x: f64, y: f64, z: f64) -> Expr {
    Expr::new(ExprKind::Vector([num(x), num(y), num(z)]), None)
}

/// The global variable `n`, shown as `g<n>`.
pub(crate) fn g(n: u32) -> Expr {
    Expr::new(ExprKind::GlobalVar(VarId(n)), None)
}

pub(crate) fn val(id: ValueId, args: Vec<Expr>) -> Expr {
    Expr::new(ExprKind::Value { id, args }, None)
}

fn k(kind: ExprKind) -> Expr {
    Expr::new(kind, None)
}

pub(crate) fn add(a: Expr, b: Expr) -> Expr {
    k(ExprKind::Add(a, b))
}
pub(crate) fn sub(a: Expr, b: Expr) -> Expr {
    k(ExprKind::Sub(a, b))
}
pub(crate) fn mul(a: Expr, b: Expr) -> Expr {
    k(ExprKind::Mul(a, b))
}
pub(crate) fn div(a: Expr, b: Expr) -> Expr {
    k(ExprKind::Div(a, b))
}
pub(crate) fn modulo(a: Expr, b: Expr) -> Expr {
    k(ExprKind::Mod(a, b))
}
pub(crate) fn pow(a: Expr, b: Expr) -> Expr {
    k(ExprKind::Pow(a, b))
}
pub(crate) fn neg(a: Expr) -> Expr {
    k(ExprKind::Neg(a))
}
pub(crate) fn not(a: Expr) -> Expr {
    k(ExprKind::Not(a))
}
pub(crate) fn and(a: Expr, b: Expr) -> Expr {
    k(ExprKind::And(a, b))
}
pub(crate) fn or(a: Expr, b: Expr) -> Expr {
    k(ExprKind::Or(a, b))
}
pub(crate) fn cmp(op: CmpOp, lhs: Expr, rhs: Expr) -> Expr {
    k(ExprKind::Compare { op, lhs, rhs })
}
pub(crate) fn tru() -> Expr {
    k(ExprKind::Bool(true))
}
pub(crate) fn fals() -> Expr {
    k(ExprKind::Bool(false))
}
pub(crate) fn arr(items: Vec<Expr>) -> Expr {
    k(ExprKind::Array(items))
}
pub(crate) fn index(array: Expr, index: Expr) -> Expr {
    k(ExprKind::Index { array, index })
}
pub(crate) fn ite(cond: Expr, then: Expr, else_: Expr) -> Expr {
    k(ExprKind::IfThenElse { cond, then, else_ })
}
pub(crate) fn cstr(text: &str) -> Expr {
    k(ExprKind::Str(crate::ir::StrExpr::Custom {
        tokens: vec![crate::ir::StrToken::Text(text.to_owned())],
        args: vec![],
    }))
}
pub(crate) fn rounding(key: &'static str) -> Expr {
    crate::lower::testutil::konst("Rounding", key)
}

/// A rendering that is compact and stable for assertions.
pub(crate) fn sh(e: &Expr) -> String {
    let list = |items: &[Expr]| items.iter().map(sh).collect::<Vec<_>>().join(", ");
    match e.kind() {
        ExprKind::Num(n) => format!("{n}"),
        ExprKind::Bool(b) => format!("{b}"),
        ExprKind::Null => "Null".to_owned(),
        ExprKind::Vector(p) => format!("V({})", list(p)),
        ExprKind::Array(items) => format!("[{}]", list(items)),
        ExprKind::GlobalVar(v) => format!("g{}", v.0),
        ExprKind::Const(c) => format!("{}.{}", c.enum_name, c.key),
        ExprKind::Value { id, args } => format!("{id:?}({})", list(args)),
        ExprKind::Index { array, index } => format!("{}[{}]", sh(array), sh(index)),
        ExprKind::Add(a, b) => format!("({} + {})", sh(a), sh(b)),
        ExprKind::Sub(a, b) => format!("({} - {})", sh(a), sh(b)),
        ExprKind::Mul(a, b) => format!("({} * {})", sh(a), sh(b)),
        ExprKind::Div(a, b) => format!("({} / {})", sh(a), sh(b)),
        ExprKind::Mod(a, b) => format!("({} % {})", sh(a), sh(b)),
        ExprKind::Pow(a, b) => format!("({} ** {})", sh(a), sh(b)),
        ExprKind::Neg(a) => format!("-{}", sh(a)),
        ExprKind::Not(a) => format!("!{}", sh(a)),
        ExprKind::And(a, b) => format!("({} && {})", sh(a), sh(b)),
        ExprKind::Or(a, b) => format!("({} || {})", sh(a), sh(b)),
        ExprKind::Compare { op, lhs, rhs } => {
            let op = match op {
                CmpOp::Eq => "==",
                CmpOp::Ne => "!=",
                CmpOp::Lt => "<",
                CmpOp::Le => "<=",
                CmpOp::Gt => ">",
                CmpOp::Ge => ">=",
            };
            format!("({} {op} {})", sh(lhs), sh(rhs))
        }
        ExprKind::IfThenElse { cond, then, else_ } => {
            format!("({} ? {} : {})", sh(cond), sh(then), sh(else_))
        }
        ExprKind::ArrayElem => "@".to_owned(),
        ExprKind::ArrayIndex => "#".to_owned(),
        ExprKind::Str(crate::ir::StrExpr::Custom { tokens, args }) => {
            let text: String = tokens
                .iter()
                .map(|t| match t {
                    crate::ir::StrToken::Text(t) => t.clone(),
                    crate::ir::StrToken::Arg(n) => format!("{{{n}}}"),
                })
                .collect();
            format!("\"{text}\"({})", list(args))
        }
        other => format!("{other:?}"),
    }
}

pub(crate) fn options(level: OptLevel, strict: bool) -> CompileOptions {
    CompileOptions {
        optimize: level,
        strict,
        ..CompileOptions::default()
    }
}

/// Optimizes `e` under `options`; returns it rendered and the warning codes reported.
pub(crate) fn run_with(e: &Expr, options: &CompileOptions) -> (String, Vec<&'static str>) {
    let mut diags = Diagnostics::default();
    let program = Program::default();
    let mut ctx = Ctx::new(options, Lang::EnUs, &mut diags, &program);
    let mut env = Env {
        scope: Scope::new(options),
        ctx: &mut ctx,
        suppressed: WarningSet::default(),
    };
    let out = sh(&optimize_expr(e, &mut env));
    (out, diags.0.iter().map(|d| d.code).collect())
}

/// `e` optimized for speed, rendered.
pub(crate) fn speed(e: &Expr) -> String {
    run_with(e, &options(OptLevel::Speed, false)).0
}

/// `e` optimized for speed in strict mode, rendered.
pub(crate) fn strict(e: &Expr) -> String {
    run_with(e, &options(OptLevel::Speed, true)).0
}

/// `e` optimized for size, rendered.
pub(crate) fn size(e: &Expr) -> String {
    run_with(e, &options(OptLevel::Size, false)).0
}

/// `e` with the optimizer off, rendered.
pub(crate) fn off(e: &Expr) -> String {
    run_with(e, &options(OptLevel::Off, false)).0
}

/// The warning codes reported while optimizing `e` for speed.
pub(crate) fn warnings(e: &Expr) -> Vec<&'static str> {
    run_with(e, &options(OptLevel::Speed, false)).1
}
