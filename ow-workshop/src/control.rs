//! Structured control flow recorded on a [`Block`].

use std::ops::Range;

use ow_workshop_data::{ActionId, Wait};

use crate::block::Block;
use crate::expr::Expr;
use crate::ir::{self, ExprKind, Location, Stmt, StmtKind};
use crate::ty::{Bool, IntoExpr, Num};
use crate::vars::Plain;

/// The bounds of a `for_range` loop: `start..stop` advancing by `step`.
#[derive(Clone, Debug)]
pub struct ForRange {
    start: ir::Expr,
    stop: ir::Expr,
    step: ir::Expr,
}

/// `start..stop` with a step of 1; unlike `a..b`, the bounds may have different types.
#[must_use]
pub fn range(start: impl IntoExpr<Num>, stop: impl IntoExpr<Num>) -> ForRange {
    ForRange {
        start: start.into_expr().into_ir(),
        stop: stop.into_expr().into_ir(),
        step: Expr::<Num>::from(1).into_ir(),
    }
}

impl ForRange {
    /// Advances by `step` instead of 1; a negative step counts down.
    #[must_use]
    pub fn step(mut self, step: impl IntoExpr<Num>) -> Self {
        self.step = step.into_expr().into_ir();
        self
    }
}

/// Something `for_range` accepts: `0..n`, `a..b` or a [`ForRange`].
pub trait IntoForRange {
    /// Converts into loop bounds.
    fn into_for_range(self) -> ForRange;
}

impl IntoForRange for ForRange {
    fn into_for_range(self) -> ForRange {
        self
    }
}

impl<T: IntoExpr<Num>> IntoForRange for Range<T> {
    fn into_for_range(self) -> ForRange {
        range(self.start, self.end)
    }
}

/// Adds `.step(s)` to `a..b`.
pub trait RangeStep: IntoForRange + Sized {
    /// Advances by `step` instead of 1.
    #[must_use]
    fn step(self, step: impl IntoExpr<Num>) -> ForRange {
        self.into_for_range().step(step)
    }
}

impl<T: IntoExpr<Num>> RangeStep for Range<T> {}

/// An `if` statement being built; it is recorded when dropped.
#[derive(Debug)]
pub struct IfBuilder<'a> {
    block: &'a mut Block,
    loc: Location,
    cond: ir::Expr,
    then: ir::Block,
    elifs: Vec<(ir::Expr, ir::Block)>,
    else_: Option<ir::Block>,
}

impl IfBuilder<'_> {
    /// Adds an `Else If` branch.
    #[must_use]
    pub fn elif(mut self, cond: impl IntoExpr<Bool>, body: impl FnOnce(&mut Block)) -> Self {
        self.elifs
            .push((cond.into_expr().into_ir(), Block::record(body)));
        self
    }

    /// Adds the `Else` branch.
    pub fn else_(mut self, body: impl FnOnce(&mut Block)) {
        self.else_ = Some(Block::record(body));
    }
}

impl Drop for IfBuilder<'_> {
    fn drop(&mut self) {
        let kind = StmtKind::If {
            cond: self.cond.clone(),
            then: std::mem::take(&mut self.then),
            elifs: std::mem::take(&mut self.elifs),
            else_: self.else_.take(),
        };
        self.block.ir.stmts.push(Stmt {
            kind,
            loc: Some(self.loc),
            comment: None,
        });
    }
}

/// "All conditions of the current rule"; `if_(rule_condition(), |b| b.loop_())` lowers to
/// `Loop If Condition Is True`.
#[track_caller]
#[must_use]
pub fn rule_condition() -> Expr<Bool> {
    Expr::from_ir(ir::Expr::here(ExprKind::RuleCondition))
}

impl Block {
    /// Runs `body` on a fresh block and returns what it recorded.
    pub(crate) fn record(body: impl FnOnce(&mut Block)) -> ir::Block {
        let mut block = Block::new();
        body(&mut block);
        block.into_ir()
    }

    /// `If`; chain `.elif(..)` and `.else_(..)` on the result.
    #[track_caller]
    pub fn if_(
        &mut self,
        cond: impl IntoExpr<Bool>,
        body: impl FnOnce(&mut Block),
    ) -> IfBuilder<'_> {
        IfBuilder {
            loc: Location::caller(),
            cond: cond.into_expr().into_ir(),
            then: Block::record(body),
            elifs: Vec::new(),
            else_: None,
            block: self,
        }
    }

    /// `While`.
    #[track_caller]
    pub fn while_(&mut self, cond: impl IntoExpr<Bool>, body: impl FnOnce(&mut Block)) {
        self.push(StmtKind::While {
            cond: cond.into_expr().into_ir(),
            body: Block::record(body),
        });
    }

    /// Runs `body`, then repeats while `cond` holds.
    ///
    /// The workshop can only loop a whole rule, so this must be the first statement of a rule or
    /// subroutine body.
    #[track_caller]
    pub fn do_while(&mut self, cond: impl IntoExpr<Bool>, body: impl FnOnce(&mut Block)) {
        self.push(StmtKind::DoWhile {
            cond: cond.into_expr().into_ir(),
            body: Block::record(body),
        });
    }

    /// `For Global/Player Variable`: counts `place` over `range`.
    #[track_caller]
    pub fn for_range(
        &mut self,
        place: impl Plain<Num>,
        range: impl IntoForRange,
        body: impl FnOnce(&mut Block),
    ) {
        let range = range.into_for_range();
        self.push(StmtKind::For {
            var: place.into_place().0,
            start: range.start,
            stop: range.stop,
            step: range.step,
            body: Block::record(body),
        });
    }

    /// `Loop`: restarts the rule.
    #[track_caller]
    pub fn loop_(&mut self) {
        self.push(StmtKind::Loop);
    }

    /// `Abort`: stops the rule.
    #[track_caller]
    pub fn abort(&mut self) {
        self.push(StmtKind::Abort);
    }

    /// An explicit no-op.
    #[track_caller]
    pub fn pass(&mut self) {
        self.push(StmtKind::Pass);
    }

    /// `Wait(duration, Ignore Condition)`.
    #[track_caller]
    pub fn wait(&mut self, duration: impl IntoExpr<Num>) {
        self.wait_with(duration, Wait::IgnoreCondition);
    }

    /// `Wait(duration, behavior)`.
    #[track_caller]
    pub fn wait_with(&mut self, duration: impl IntoExpr<Num>, behavior: Wait) {
        self.push(StmtKind::Action {
            id: ActionId::Wait,
            args: vec![
                duration.into_expr().into_ir(),
                behavior.into_expr().into_ir(),
            ],
        });
    }

    /// `Wait Until(condition, timeout)`; only `True` ends the wait.
    #[track_caller]
    pub fn wait_until(&mut self, cond: impl IntoExpr<Bool>, timeout: impl IntoExpr<Num>) {
        self.push(StmtKind::Action {
            id: ActionId::WaitUntil,
            args: vec![cond.into_expr().into_ir(), timeout.into_expr().into_ir()],
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::Builder;
    use crate::event::OngoingGlobal;
    use crate::ir::{Place as IrPlace, Program, VarId};
    use crate::prelude::*;

    #[derive(GlobalVars)]
    struct Globals {
        i: Global<Num>,
        n: Global<Num>,
    }

    fn globals() -> Globals {
        Globals::declare(&mut VarRegistry::default())
    }

    fn body(f: impl FnOnce(&mut Block)) -> Vec<Stmt> {
        Block::record(f).stmts
    }

    #[test]
    fn if_elif_else() {
        let g = globals();
        let stmts = body(|b| {
            b.if_(g.n.get().gt(1), |b| b.abort())
                .elif(g.n.get().gt(0), |b| b.loop_())
                .else_(|b| b.pass());
        });
        assert_eq!(stmts.len(), 1);
        assert!(stmts[0].loc.is_some_and(|l| l.file.ends_with("control.rs")));
        let StmtKind::If {
            then, elifs, else_, ..
        } = &stmts[0].kind
        else {
            panic!("not an if")
        };
        assert!(matches!(then.stmts[0].kind, StmtKind::Abort));
        assert_eq!(elifs.len(), 1);
        assert!(matches!(elifs[0].1.stmts[0].kind, StmtKind::Loop));
        assert!(matches!(
            else_.as_ref().map(|b| &b.stmts[0].kind),
            Some(StmtKind::Pass)
        ));
    }

    #[test]
    fn if_without_else_is_recorded_on_drop() {
        let stmts = body(|b| {
            b.if_(true, |b| b.abort());
            b.abort();
        });
        assert!(matches!(stmts[0].kind, StmtKind::If { .. }));
        assert!(matches!(stmts[1].kind, StmtKind::Abort));
    }

    #[test]
    fn while_and_do_while() {
        let stmts = body(|b| {
            b.while_(true, |b| b.wait(1));
            b.do_while(false, |b| b.loop_());
        });
        assert!(matches!(&stmts[0].kind, StmtKind::While { body, .. } if body.stmts.len() == 1));
        assert!(matches!(&stmts[1].kind, StmtKind::DoWhile { .. }));
    }

    #[test]
    fn for_range_forms() {
        let g = globals();
        let n = g.n.get();
        let stmts = body(|b| {
            b.for_range(g.i, 0..10, |_| {});
            b.for_range(g.i, range(2, n.clone()), |_| {});
            b.for_range(g.i, n.clone()..n.clone(), |_| {});
            b.for_range(g.i, (0..10).step(2), |_| {});
            b.for_range(g.i, range(10, 0).step(-1), |_| {});
        });
        let steps: Vec<_> = stmts
            .iter()
            .map(|s| match &s.kind {
                StmtKind::For { var, step, .. } => {
                    assert_eq!(var, &IrPlace::Global(VarId(0)));
                    step.kind().clone()
                }
                other => panic!("unexpected {other:?}"),
            })
            .collect();
        assert_eq!(
            steps,
            [
                ExprKind::Num(1.0),
                ExprKind::Num(1.0),
                ExprKind::Num(1.0),
                ExprKind::Num(2.0),
                ExprKind::Num(-1.0)
            ]
        );
    }

    #[test]
    fn waits() {
        let stmts = body(|b| {
            b.wait(0.5);
            b.wait_with(1, Wait::AbortWhenFalse);
            b.wait_until(true, 5);
        });
        let ids: Vec<_> = stmts
            .iter()
            .map(|s| match &s.kind {
                StmtKind::Action { id, args } => (*id, args.len()),
                other => panic!("unexpected {other:?}"),
            })
            .collect();
        assert_eq!(
            ids,
            [
                (ActionId::Wait, 2),
                (ActionId::Wait, 2),
                (ActionId::WaitUntil, 2)
            ]
        );
    }

    #[test]
    fn rule_condition_is_recorded() {
        let stmts = body(|b| {
            b.if_(rule_condition(), |b| b.loop_());
            b.if_(!rule_condition(), |b| b.abort());
        });
        let StmtKind::If { cond, .. } = &stmts[0].kind else {
            panic!("not an if")
        };
        assert_eq!(cond.kind(), &ExprKind::RuleCondition);
        let StmtKind::If { cond, .. } = &stmts[1].kind else {
            panic!("not an if")
        };
        assert!(matches!(cond.kind(), ExprKind::Not(_)));
    }

    fn compile_rule(f: impl FnOnce(&mut Block)) -> crate::ir::Diagnostics {
        let mut ws = Builder::new();
        ws.rule("r", OngoingGlobal, |r, _| r.actions(f));
        ws.finish(Program::default()).1
    }

    #[test]
    fn do_while_first_is_fine() {
        let diags = compile_rule(|b| {
            b.pass();
            b.do_while(true, |b| b.wait(1));
        });
        assert_eq!(diags.0, vec![]);
    }

    #[test]
    fn misplaced_do_while_is_an_error() {
        let diags = compile_rule(|b| {
            b.wait(1);
            b.do_while(true, |b| b.wait(1));
        });
        assert_eq!(diags.0.len(), 1);
        assert_eq!(diags.0[0].code, "misplaced-do-while");
        assert!(diags.0[0].location.is_some());

        let nested = compile_rule(|b| {
            b.if_(true, |b| b.do_while(true, |b| b.pass()))
                .else_(|_| {});
        });
        assert_eq!(nested.0.len(), 1);
    }
}
