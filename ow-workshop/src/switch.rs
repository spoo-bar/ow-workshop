//! `switch`, `break_` and `continue_`.

use std::marker::PhantomData;

use crate::block::Block;
use crate::ir::{self, Location, Stmt, StmtKind};
use crate::ty::{IntoExpr, ValueType};

/// A `switch` statement being built; it is recorded when dropped.
///
/// Cases fall through to the next one unless they call [`Block::break_`], like C.
#[derive(Debug)]
pub struct SwitchBuilder<'a, T> {
    block: &'a mut Block,
    loc: Location,
    value: ir::Expr,
    cases: Vec<(ir::Expr, ir::Block)>,
    default: Option<(usize, ir::Block)>,
    _type: PhantomData<fn() -> T>,
}

impl<T: ValueType> SwitchBuilder<'_, T> {
    /// Adds a case; usually `value` is a literal.
    pub fn case(mut self, value: impl IntoExpr<T>, body: impl FnOnce(&mut Block)) -> Self {
        self.cases
            .push((value.into_expr().into_ir(), Block::record(body)));
        self
    }

    /// Adds the default case; it may appear between cases.
    pub fn default(mut self, body: impl FnOnce(&mut Block)) -> Self {
        self.default = Some((self.cases.len(), Block::record(body)));
        self
    }
}

impl<T> Drop for SwitchBuilder<'_, T> {
    fn drop(&mut self) {
        let (default_at, default) = match self.default.take() {
            Some((at, body)) => (at, Some(body)),
            None => (self.cases.len(), None),
        };
        let kind = StmtKind::Switch {
            value: self.value.clone(),
            cases: std::mem::take(&mut self.cases),
            default,
            default_at,
        };
        self.block.ir.stmts.push(Stmt {
            kind,
            loc: Some(self.loc),
            comment: None,
        });
    }
}

impl Block {
    /// Starts a `switch` on `value`; chain `.case(..)` and `.default(..)`.
    #[track_caller]
    pub fn switch<T: ValueType>(&mut self, value: impl IntoExpr<T>) -> SwitchBuilder<'_, T> {
        SwitchBuilder {
            loc: Location::caller(),
            value: value.into_expr().into_ir(),
            cases: Vec::new(),
            default: None,
            _type: PhantomData,
            block: self,
        }
    }

    /// Leaves the innermost loop or switch.
    #[track_caller]
    pub fn break_(&mut self) {
        self.push(StmtKind::Break);
    }

    /// Jumps to the next iteration of the innermost loop.
    #[track_caller]
    pub fn continue_(&mut self) {
        self.push(StmtKind::Continue);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::ExprKind;
    use crate::prelude::*;

    fn body(f: impl FnOnce(&mut Block)) -> Vec<Stmt> {
        Block::record(f).stmts
    }

    fn switch_parts(stmt: &Stmt) -> (&[(ir::Expr, ir::Block)], Option<&ir::Block>, usize) {
        let StmtKind::Switch {
            cases,
            default,
            default_at,
            ..
        } = &stmt.kind
        else {
            panic!("not a switch");
        };
        (cases, default.as_ref(), *default_at)
    }

    #[test]
    fn cases_fall_through_unless_they_break() {
        let stmts = body(|b| {
            b.switch(Hero::Ana)
                .case(Hero::Ana, |b| {
                    b.pass();
                    b.break_();
                })
                .case(Hero::Ashe, |b| b.pass())
                .default(|b| b.abort());
        });
        assert_eq!(stmts.len(), 1);
        assert!(stmts[0].loc.is_some_and(|l| l.file.ends_with("switch.rs")));
        let (cases, default, default_at) = switch_parts(&stmts[0]);
        assert_eq!(cases.len(), 2);
        assert!(matches!(cases[0].0.kind(), ExprKind::Const(_)));
        assert!(matches!(cases[0].1.stmts[1].kind, StmtKind::Break));
        assert_eq!(cases[1].1.stmts.len(), 1);
        assert!(default.is_some());
        assert_eq!(default_at, 2);
    }

    #[test]
    fn default_may_sit_between_cases() {
        let stmts = body(|b| {
            b.switch::<Num>(1)
                .case(0, |b| b.pass())
                .default(|b| b.pass())
                .case(1, |b| b.pass());
        });
        let (cases, default, default_at) = switch_parts(&stmts[0]);
        assert_eq!(cases.len(), 2);
        assert!(default.is_some());
        assert_eq!(default_at, 1);
    }

    #[test]
    fn switch_without_default() {
        let stmts = body(|b| {
            b.switch(true).case(true, |b| b.pass());
        });
        let (_, default, default_at) = switch_parts(&stmts[0]);
        assert!(default.is_none());
        assert_eq!(default_at, 1);
    }

    #[test]
    fn nested_switches_and_loops() {
        let stmts = body(|b| {
            b.while_(true, |b| {
                b.switch::<Num>(1).case(1, |b| {
                    b.switch::<Num>(2).case(2, |b| b.continue_());
                    b.break_();
                });
            });
        });
        let StmtKind::While {
            body: loop_body, ..
        } = &stmts[0].kind
        else {
            panic!("not a loop")
        };
        let (cases, _, _) = switch_parts(&loop_body.stmts[0]);
        let inner = &cases[0].1;
        assert_eq!(inner.stmts.len(), 2);
        let (inner_cases, _, _) = switch_parts(&inner.stmts[0]);
        assert!(matches!(inner_cases[0].1.stmts[0].kind, StmtKind::Continue));
        assert!(matches!(inner.stmts[1].kind, StmtKind::Break));
    }
}
