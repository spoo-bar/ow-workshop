//! The statement list actions are recorded on.

use std::cell::Cell;
use std::rc::Rc;

use ow_workshop_data::{ChaseRateReeval, ChaseTimeReeval};

use crate::ir::{self, ModifyOp, Stmt, StmtKind, WarningSet};
use crate::ty::{Array, IntoExpr, Num, ValueType};
use crate::vars::{Place, Plain};
use crate::warning::Warning;

/// A list of statements: a rule's actions or the body of a control-flow construct.
#[derive(Debug, Default)]
pub struct Block {
    pub(crate) ir: ir::Block,
    /// Label counter shared by every block of the mod, so label ids are unique across rules.
    labels: Rc<Cell<u32>>,
    /// Warnings suppressed for the statements recorded from here on.
    pub(crate) suppressed: WarningSet,
    /// Whether the mod enables text tags (`CompileOptions::tags`), which helpers may use.
    pub(crate) tags: bool,
}

// Constructed by the rule builder.
#[allow(dead_code)]
impl Block {
    /// An empty block.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// An empty block drawing label ids from `labels`.
    pub(crate) fn with_labels(labels: Rc<Cell<u32>>) -> Self {
        Self {
            ir: ir::Block::default(),
            labels,
            suppressed: WarningSet::default(),
            tags: false,
        }
    }

    /// Runs `body` on a fresh block that shares this block's label counter.
    pub(crate) fn record(&self, body: impl FnOnce(&mut Block)) -> ir::Block {
        let mut block = Self::with_labels(Rc::clone(&self.labels));
        block.suppressed = self.suppressed;
        block.tags = self.tags;
        body(&mut block);
        block.into_ir()
    }

    /// Hands out the next label id.
    pub(crate) fn next_label(&self) -> u32 {
        let id = self.labels.get();
        self.labels.set(id + 1);
        id
    }

    /// The recorded statements.
    pub(crate) fn into_ir(self) -> ir::Block {
        self.ir
    }

    /// Records a statement located at the caller.
    #[track_caller]
    pub(crate) fn push(&mut self, kind: StmtKind) {
        let mut stmt = Stmt::new(kind);
        stmt.suppress = self.suppressed;
        self.ir.stmts.push(stmt);
    }

    /// Runs `body` with `warning` suppressed for everything it records, nested blocks included.
    pub fn suppress(&mut self, warning: Warning, body: impl FnOnce(&mut Block)) {
        let outer = self.suppressed;
        self.suppressed.insert(warning);
        body(self);
        self.suppressed = outer;
    }

    /// Records a no-op.
    #[cfg(test)]
    #[track_caller]
    pub(crate) fn pass_placeholder(&mut self) {
        self.push(StmtKind::Pass);
    }

    #[track_caller]
    fn modify<T: ValueType>(
        &mut self,
        op: ModifyOp,
        target: impl Place<T>,
        value: impl IntoExpr<T>,
    ) {
        self.push(StmtKind::Modify {
            target: target.into_place().0,
            op,
            value: value.into_expr().into_ir(),
        });
    }

    /// Assigns `value` to `place`.
    #[track_caller]
    pub fn set<T: ValueType>(&mut self, place: impl Place<T>, value: impl IntoExpr<T>) {
        self.push(StmtKind::Set {
            target: place.into_place().0,
            value: value.into_expr().into_ir(),
        });
    }

    /// `Modify Variable (Add)`.
    #[track_caller]
    pub fn add_to<T: ValueType>(&mut self, place: impl Place<T>, value: impl IntoExpr<T>) {
        self.modify(ModifyOp::Add, place, value);
    }

    /// `Modify Variable (Subtract)`.
    #[track_caller]
    pub fn sub_from<T: ValueType>(&mut self, place: impl Place<T>, value: impl IntoExpr<T>) {
        self.modify(ModifyOp::Subtract, place, value);
    }

    /// `Modify Variable (Multiply)`.
    #[track_caller]
    pub fn mul_by<T: ValueType>(&mut self, place: impl Place<T>, value: impl IntoExpr<T>) {
        self.modify(ModifyOp::Multiply, place, value);
    }

    /// `Modify Variable (Divide)`.
    #[track_caller]
    pub fn div_by<T: ValueType>(&mut self, place: impl Place<T>, value: impl IntoExpr<T>) {
        self.modify(ModifyOp::Divide, place, value);
    }

    /// `Modify Variable (Modulo)`.
    #[track_caller]
    pub fn mod_by<T: ValueType>(&mut self, place: impl Place<T>, value: impl IntoExpr<T>) {
        self.modify(ModifyOp::Modulo, place, value);
    }

    /// `Modify Variable (Raise To Power)`.
    #[track_caller]
    pub fn pow_by<T: ValueType>(&mut self, place: impl Place<T>, value: impl IntoExpr<T>) {
        self.modify(ModifyOp::RaiseToPower, place, value);
    }

    /// `Modify Variable (Min)`.
    #[track_caller]
    pub fn min_with<T: ValueType>(&mut self, place: impl Place<T>, value: impl IntoExpr<T>) {
        self.modify(ModifyOp::Min, place, value);
    }

    /// `Modify Variable (Max)`.
    #[track_caller]
    pub fn max_with<T: ValueType>(&mut self, place: impl Place<T>, value: impl IntoExpr<T>) {
        self.modify(ModifyOp::Max, place, value);
    }

    /// `Modify Variable (Append To Array)`.
    #[track_caller]
    pub fn append<E: ValueType>(&mut self, place: impl Place<Array<E>>, value: impl IntoExpr<E>) {
        self.push(StmtKind::Modify {
            target: place.into_place().0,
            op: ModifyOp::AppendToArray,
            value: value.into_expr().into_ir(),
        });
    }

    /// `Modify Variable (Remove From Array By Value)`.
    #[track_caller]
    pub fn remove_value<E: ValueType>(
        &mut self,
        place: impl Place<Array<E>>,
        value: impl IntoExpr<E>,
    ) {
        self.push(StmtKind::Modify {
            target: place.into_place().0,
            op: ModifyOp::RemoveFromArrayByValue,
            value: value.into_expr().into_ir(),
        });
    }

    /// `Modify Variable (Remove From Array By Index)`.
    #[track_caller]
    pub fn remove_index<E: ValueType>(
        &mut self,
        place: impl Place<Array<E>>,
        index: impl IntoExpr<Num>,
    ) {
        self.push(StmtKind::Modify {
            target: place.into_place().0,
            op: ModifyOp::RemoveFromArrayByIndex,
            value: index.into_expr().into_ir(),
        });
    }

    /// `Chase Variable At Rate`; only plain variables can be chased.
    #[track_caller]
    pub fn chase_at_rate<T: ValueType>(
        &mut self,
        place: impl Plain<T>,
        dest: impl IntoExpr<T>,
        rate: impl IntoExpr<Num>,
        reeval: ChaseRateReeval,
    ) {
        self.push(StmtKind::Chase {
            target: place.into_place().0,
            dest: dest.into_expr().into_ir(),
            rate_or_duration: rate.into_expr().into_ir(),
            reeval: reeval.into_expr().into_ir(),
            over_time: false,
        });
    }

    /// `Chase Variable Over Time`; only plain variables can be chased.
    #[track_caller]
    pub fn chase_over_time<T: ValueType>(
        &mut self,
        place: impl Plain<T>,
        dest: impl IntoExpr<T>,
        duration: impl IntoExpr<Num>,
        reeval: ChaseTimeReeval,
    ) {
        self.push(StmtKind::Chase {
            target: place.into_place().0,
            dest: dest.into_expr().into_ir(),
            rate_or_duration: duration.into_expr().into_ir(),
            reeval: reeval.into_expr().into_ir(),
            over_time: true,
        });
    }

    /// `Stop Chasing Variable`.
    #[track_caller]
    pub fn stop_chasing<T: ValueType>(&mut self, place: impl Plain<T>) {
        self.push(StmtKind::StopChasing(place.into_place().0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{ExprKind, Place as IrPlace, VarId};
    use crate::prelude::*;
    use crate::ty::Position;

    #[derive(GlobalVars)]
    #[allow(dead_code)]
    struct Globals {
        #[ow(index = 2)]
        round: Global<Num>,
        #[ow(init = 3)]
        lives: Global<Num>,
        #[ow(name = "Spawn Points")]
        spawns: Global<Array<Position>>,
        list: Global<Array<Num>>,
    }

    #[derive(PlayerVars)]
    #[allow(dead_code)]
    struct Players {
        score: PlayerVar<Num>,
        history: PlayerVar<Array<Num>>,
    }

    fn globals() -> (Globals, VarRegistry) {
        let mut reg = VarRegistry::default();
        (Globals::declare(&mut reg), reg)
    }

    fn only(b: Block) -> ir::Stmt {
        let mut stmts = b.into_ir().stmts;
        assert_eq!(stmts.len(), 1);
        stmts.remove(0)
    }

    #[test]
    fn derive_records_names_index_and_init() {
        let (_, reg) = globals();
        let entries = &reg.vars.entries;
        assert_eq!(entries.len(), 4);
        assert_eq!(entries[0].name, "round");
        assert_eq!(entries[0].index, Some(2));
        assert_eq!(entries[1].name, "lives");
        assert_eq!(
            entries[1].init.as_ref().map(ir::Expr::kind),
            Some(&ExprKind::Num(3.0))
        );
        assert_eq!(entries[2].name, "Spawn Points");
        assert_eq!(entries[2].index, None);
        assert!(
            entries[0]
                .location
                .is_some_and(|l| l.file.ends_with("block.rs"))
        );
    }

    #[test]
    fn player_derive_declares_in_its_own_registry() {
        let mut reg = VarRegistry::default();
        let p = Players::declare(&mut reg);
        assert_eq!(reg.vars.entries.len(), 2);
        assert_eq!(reg.vars.entries[0].name, "score");
        let me: Expr<Player> = null();
        let mut b = Block::new();
        b.add_to(me.var(p.score), 1);
        let StmtKind::Modify { target, .. } = only(b).kind else {
            panic!("not a modify")
        };
        assert!(matches!(target, IrPlace::Player { var: VarId(0), .. }));
    }

    #[test]
    fn set_and_read() {
        let (g, _) = globals();
        let mut b = Block::new();
        b.set(g.round, g.lives);
        let stmt = only(b);
        assert!(stmt.loc.is_some());
        let StmtKind::Set { target, value } = stmt.kind else {
            panic!("not a set")
        };
        assert_eq!(target, IrPlace::Global(VarId(0)));
        assert!(matches!(value.kind(), ExprKind::GlobalVar(VarId(1))));
    }

    #[test]
    fn every_modify_op() {
        let (g, _) = globals();
        let mut b = Block::new();
        b.add_to(g.round, 1);
        b.sub_from(g.round, 1);
        b.mul_by(g.round, 2);
        b.div_by(g.round, 2);
        b.mod_by(g.round, 2);
        b.pow_by(g.round, 2);
        b.min_with(g.round, 2);
        b.max_with(g.round, 2);
        b.append(g.list, 5);
        b.remove_value(g.list, 5);
        b.remove_index(g.list, 0);
        let ops: Vec<_> = b
            .into_ir()
            .stmts
            .into_iter()
            .map(|s| match s.kind {
                StmtKind::Modify { op, .. } => op,
                other => panic!("unexpected {other:?}"),
            })
            .collect();
        assert_eq!(
            ops,
            [
                ir::ModifyOp::Add,
                ir::ModifyOp::Subtract,
                ir::ModifyOp::Multiply,
                ir::ModifyOp::Divide,
                ir::ModifyOp::Modulo,
                ir::ModifyOp::RaiseToPower,
                ir::ModifyOp::Min,
                ir::ModifyOp::Max,
                ir::ModifyOp::AppendToArray,
                ir::ModifyOp::RemoveFromArrayByValue,
                ir::ModifyOp::RemoveFromArrayByIndex,
            ]
        );
    }

    #[test]
    fn indexed_places_read_and_write() {
        let (g, _) = globals();
        let mut b = Block::new();
        b.set(g.list.at(2), g.list.at(1));
        let StmtKind::Set { target, value } = only(b).kind else {
            panic!("not a set")
        };
        assert!(matches!(target, IrPlace::Index { .. }));
        assert!(matches!(value.kind(), ExprKind::Index { .. }));
    }

    #[test]
    fn chasing_records_mode_and_target() {
        let (g, _) = globals();
        let mut b = Block::new();
        b.chase_at_rate(g.round, 10, 2, ChaseRateReeval::DestinationAndRate);
        b.chase_over_time(g.round, 10, 2, ChaseTimeReeval::DestinationAndDuration);
        b.stop_chasing(g.round);
        let kinds: Vec<_> = b.into_ir().stmts.into_iter().map(|s| s.kind).collect();
        assert!(matches!(
            kinds[0],
            StmtKind::Chase {
                over_time: false,
                ..
            }
        ));
        assert!(matches!(
            kinds[1],
            StmtKind::Chase {
                over_time: true,
                ..
            }
        ));
        assert!(matches!(
            kinds[2],
            StmtKind::StopChasing(IrPlace::Global(_))
        ));
    }
}

#[cfg(test)]
mod multi_dim_tests {
    use crate::builder::Builder;
    use crate::check::check_multi_dim_indices;
    use crate::event::OngoingGlobal;
    use crate::ir::{self, Diagnostics, ExprKind, Place as IrPlace, Program, StmtKind};
    use crate::prelude::*;

    #[derive(GlobalVars)]
    struct Globals {
        grid: Global<Array<Array<Num>>>,
        cube: Global<Array<Array<Array<Num>>>>,
    }

    fn globals() -> Globals {
        Globals::declare(&mut VarRegistry::default())
    }

    fn depth(place: &IrPlace) -> usize {
        match place {
            IrPlace::Index { base, .. } => 1 + depth(base),
            _ => 0,
        }
    }

    fn only_target(b: Block) -> IrPlace {
        match b.into_ir().stmts.remove(0).kind {
            StmtKind::Set { target, .. } | StmtKind::Modify { target, .. } => target,
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn two_and_three_dimensional_assignment() {
        let g = globals();
        let mut b = Block::new();
        b.set(g.grid.at(1).at(2), 5);
        assert_eq!(depth(&only_target(b)), 2);

        let mut b = Block::new();
        b.set(g.cube.at(1).at(2).at(3), 5);
        assert_eq!(depth(&only_target(b)), 3);
    }

    #[test]
    fn removing_from_a_nested_array() {
        let g = globals();
        let mut b = Block::new();
        b.remove_index(g.grid.at(1), 0);
        b.remove_index(g.cube.at(1).at(2), 0);
        let stmts = b.into_ir().stmts;
        let depths: Vec<_> = stmts
            .iter()
            .map(|s| match &s.kind {
                StmtKind::Modify { target, op, .. } => {
                    assert_eq!(*op, ir::ModifyOp::RemoveFromArrayByIndex);
                    depth(target)
                }
                other => panic!("unexpected {other:?}"),
            })
            .collect();
        assert_eq!(depths, [1, 2]);
    }

    #[test]
    fn reading_a_nested_element() {
        let g = globals();
        let e = g.grid.at(1).at(2).get();
        let ExprKind::Index { array, .. } = e.ir().kind() else {
            panic!("not an index")
        };
        assert!(matches!(array.kind(), ExprKind::Index { .. }));
    }

    fn errors(build: impl FnOnce(&mut Block)) -> Vec<&'static str> {
        let mut ws = Builder::new();
        ws.rule("r", OngoingGlobal, |r, _| r.actions(build));
        let (program, _): (Program, _) = ws.finish(Program::default());
        let mut diags = Diagnostics::default();
        check_multi_dim_indices(&program.rules, &mut diags);
        assert!(diags.0.iter().all(|d| d.location.is_some()));
        diags.0.iter().map(Diagnostic::code).collect()
    }

    #[test]
    fn random_index_in_a_multi_dimensional_place_is_an_error() {
        let g = globals();
        assert_eq!(
            errors(|b| b.set(g.grid.at(random::int(0, 3)).at(1), 5)),
            ["random-in-multi-dim-index"]
        );
        assert_eq!(
            errors(|b| b.add_to(g.grid.at(0).at(random::int(0, 3) + 1), 5)),
            ["random-in-multi-dim-index"]
        );
    }

    #[test]
    fn random_index_in_a_one_dimensional_place_is_fine() {
        let g = globals();
        assert_eq!(
            errors(|b| b.set(g.grid.at(random::int(0, 3)), [1])),
            [] as [&str; 0]
        );
        assert_eq!(
            errors(|b| b.set(g.grid.at(1).at(2), random::int(0, 3))),
            [] as [&str; 0]
        );
    }
}
