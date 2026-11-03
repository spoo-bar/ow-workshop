//! The statement list actions are recorded on.

use ow_workshop_data::{ChaseRateReeval, ChaseTimeReeval};

use crate::ir::{self, ModifyOp, Stmt, StmtKind};
use crate::ty::{Array, IntoExpr, Num, ValueType};
use crate::vars::{Place, Plain};

/// A list of statements: a rule's actions or the body of a control-flow construct.
#[derive(Debug, Default)]
pub struct Block {
    ir: ir::Block,
}

// Constructed by the rule builder.
#[allow(dead_code)]
impl Block {
    /// An empty block.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// The recorded statements.
    pub(crate) fn into_ir(self) -> ir::Block {
        self.ir
    }

    /// Records a statement located at the caller.
    #[track_caller]
    pub(crate) fn push(&mut self, kind: StmtKind) {
        self.ir.stmts.push(Stmt::new(kind));
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
