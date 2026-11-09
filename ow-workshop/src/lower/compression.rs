//! Expands `Decompress` nodes into the workshop expression that decodes the packed string.
//!
//! Each item of the string is a run of alphabet symbols; a symbol's position in the alphabet is
//! a base-100 digit, so `Index Of String Char(alphabet, Char In String(item, k))` reads digit
//! `k`.

use ow_workshop_data::ValueId;

use super::Ctx;
use super::visit::{blocks_mut, map_stmt_roots, rewrite_expr};
use crate::compress::alphabet_text;
use crate::ir::{
    Decompress, Diagnostics, Expr, ExprKind, Location, Program, StrExpr, StrToken, VarDecl, VarId,
};

/// The internal global that holds the alphabet when the option asks for it.
pub(crate) const ALPHABET_VAR: &str = "__compressionAlphabet__";

fn node(kind: ExprKind, loc: Option<Location>) -> Expr {
    Expr::new(kind, loc)
}

fn num(n: f64, loc: Option<Location>) -> Expr {
    node(ExprKind::Num(n), loc)
}

fn value(id: ValueId, args: Vec<Expr>, loc: Option<Location>) -> Expr {
    node(ExprKind::Value { id, args }, loc)
}

fn alphabet(loc: Option<Location>) -> Expr {
    node(
        ExprKind::Str(StrExpr::Custom {
            tokens: vec![StrToken::Text(alphabet_text())],
            args: Vec::new(),
        }),
        loc,
    )
}

fn has_decompress(expr: &Expr) -> bool {
    matches!(expr.kind(), ExprKind::Decompress(_))
        || expr.kind().children().into_iter().any(has_decompress)
}

fn program_decompresses(program: &Program) -> bool {
    let mut found = false;
    let mut check = |e: &Expr| found |= has_decompress(e);
    for rule in &program.rules {
        rule.conditions.iter().for_each(|c| check(&c.expr));
        super::visit::walk_block(&rule.body, &mut |stmt| {
            stmt.exprs().into_iter().for_each(&mut check);
        });
    }
    for decl in program
        .globals
        .entries
        .iter()
        .chain(&program.players.entries)
    {
        decl.init.iter().for_each(&mut check);
    }
    found
}

/// Declares `__compressionAlphabet__` when the option is on and something is compressed.
///
/// Runs before the init rules are generated, so the variable gets its initial value.
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn declare_variables(
    program: &mut Program,
    ctx: &mut Ctx<'_>,
) -> Result<(), Diagnostics> {
    let declared = alphabet_var(program).is_some();
    if !ctx.options.compression_alphabet_variable || declared || !program_decompresses(program) {
        return Ok(());
    }
    program.globals.declare(VarDecl {
        name: ALPHABET_VAR.to_owned(),
        index: None,
        init: Some(alphabet(None)),
        internal: true,
        flags: crate::ir::VarFlags::default(),
        location: None,
    });
    Ok(())
}

fn alphabet_var(program: &Program) -> Option<VarId> {
    let at = program
        .globals
        .entries
        .iter()
        .position(|d| d.name == ALPHABET_VAR)?;
    Some(VarId(u32::try_from(at).ok()?))
}

/// The pass.
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn lower(program: &mut Program, _: &mut Ctx<'_>) -> Result<(), Diagnostics> {
    let var = alphabet_var(program);
    let mut expand = |e: Expr| match e.kind() {
        ExprKind::Decompress(d) => decode(d, e.loc(), var),
        _ => e,
    };
    for rule in &mut program.rules {
        for cond in &mut rule.conditions {
            cond.expr = rewrite_expr(&cond.expr, &mut expand);
        }
        lower_block(&mut rule.body, &mut expand);
    }
    Ok(())
}

fn lower_block(block: &mut crate::ir::Block, node: &mut impl FnMut(Expr) -> Expr) {
    for stmt in &mut block.stmts {
        map_stmt_roots(stmt, &mut |e| rewrite_expr(e, node));
        for child in blocks_mut(stmt) {
            lower_block(child, node);
        }
    }
}

/// `Mapped Array(items, decode(e))`.
///
/// Without the variable, each item is paired with the alphabet text (`Append To Array(e,
/// alphabet)`) so the text appears once, and the decoder reads it back with `Last Of(e)`.
pub(crate) fn decode(d: &Decompress, loc: Option<Location>, var: Option<VarId>) -> Expr {
    let at = |n: f64| num(n, loc);
    let elem = || node(ExprKind::ArrayElem, loc);
    let null = || node(ExprKind::Null, loc);
    let split = value(
        ValueId::Split,
        vec![d.packed.clone(), value(ValueId::FirstOf, vec![null()], loc)],
        loc,
    );
    let (items, symbols) = match var {
        Some(var) => (split, node(ExprKind::GlobalVar(var), loc)),
        None => (
            value(
                ValueId::MappedArray,
                vec![
                    split,
                    value(ValueId::Concat, vec![elem(), alphabet(loc)], loc),
                ],
                loc,
            ),
            value(ValueId::Last, vec![elem()], loc),
        ),
    };

    // Symbols per component (each pair of digits is one symbol).
    let symbols_per_component = (d.max_place - d.min_place + 1).max(0) / 2;
    let component = |part: i32| {
        let mut sum: Option<Expr> = None;
        for k in 0..symbols_per_component {
            let digit_pair = value(
                ValueId::StrIndex,
                vec![
                    symbols.clone(),
                    value(
                        ValueId::CharAt,
                        vec![elem(), at(f64::from(k + part * symbols_per_component))],
                        loc,
                    ),
                ],
                loc,
            );
            // 100^(k + min_place / 2) = 10^(2k + min_place), also for an odd min_place.
            let weight = node(
                ExprKind::Pow(at(100.0), at(f64::from(k) + f64::from(d.min_place) / 2.0)),
                loc,
            );
            let term = node(ExprKind::Mul(weight, digit_pair), loc);
            sum = Some(match sum {
                Some(sum) => node(ExprKind::Add(sum, term), loc),
                None => term,
            });
        }
        sum.unwrap_or_else(|| at(0.0))
    };

    let body = if d.vectors {
        // The text holds x, z, y.
        let vector = node(
            ExprKind::Vector([component(0), component(2), component(1)]),
            loc,
        );
        let offsets = node(ExprKind::Vector([at(1.0), at(1.0), at(1.0)]), loc);
        node(
            ExprKind::Sub(vector, node(ExprKind::Mul(offsets, at(d.offset)), loc)),
            loc,
        )
    } else {
        node(ExprKind::Sub(component(0), at(d.offset)), loc)
    };
    value(ValueId::MappedArray, vec![items, body], loc)
}
