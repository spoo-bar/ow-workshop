//! Untyped statements.

use ow_workshop_data::{ActionId, StartRuleBehavior};

use super::{Expr, LabelId, Location, OptOverride, SubId, VarId, WarningSet};

/// An assignable target.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Place {
    /// A global variable.
    Global(VarId),
    /// A player variable.
    Player {
        /// The owning player.
        player: Expr,
        /// The variable.
        var: VarId,
    },
    /// An element of another place; nests to any depth.
    Index {
        /// The indexed place.
        base: Box<Place>,
        /// The index.
        index: Expr,
    },
}

/// The operation of a `Modify` statement.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum ModifyOp {
    /// `Add`
    Add,
    /// `Subtract`
    Subtract,
    /// `Multiply`
    Multiply,
    /// `Divide`
    Divide,
    /// `Modulo`
    Modulo,
    /// `Raise To Power`
    RaiseToPower,
    /// `Min`
    Min,
    /// `Max`
    Max,
    /// `Append To Array`
    AppendToArray,
    /// `Remove From Array By Value`
    RemoveFromArrayByValue,
    /// `Remove From Array By Index`
    RemoveFromArrayByIndex,
}

/// Where a `Skip` lands.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum SkipTarget {
    /// A label, resolved to a distance by flatten.
    Label(LabelId),
    /// A computed distance.
    Dynamic(Expr),
}

/// A sequence of statements.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Block {
    /// The statements, in order.
    pub(crate) stmts: Vec<Stmt>,
    /// Optimizer settings for this block and its children; `None` inherits.
    pub(crate) opt: Option<OptOverride>,
}

/// The shape of a statement.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum StmtKind {
    /// A call of a workshop action.
    Action {
        /// Which action.
        id: ActionId,
        /// Arguments in signature order.
        args: Vec<Expr>,
    },
    /// Assigns a variable.
    Set {
        /// The variable.
        target: Place,
        /// The new value.
        value: Expr,
    },
    /// Modifies a variable in place.
    Modify {
        /// The variable.
        target: Place,
        /// The operation.
        op: ModifyOp,
        /// The operand.
        value: Expr,
    },
    /// `Chase Variable Over Time` / `At Rate`.
    Chase {
        /// The variable.
        target: Place,
        /// The value to approach.
        dest: Expr,
        /// Duration (over time) or rate.
        rate_or_duration: Expr,
        /// Reevaluation mode.
        reeval: Expr,
        /// `true` for over time, `false` for at rate.
        over_time: bool,
    },
    /// `Stop Chasing Variable`.
    StopChasing(Place),
    /// Structured conditional; flatten turns it into `If/Else If/Else/End`.
    If {
        /// Condition.
        cond: Expr,
        /// Taken when `cond` holds.
        then: Block,
        /// `Else If` branches.
        elifs: Vec<(Expr, Block)>,
        /// The `Else` branch.
        else_: Option<Block>,
    },
    /// `While`.
    While {
        /// Condition.
        cond: Expr,
        /// Loop body.
        body: Block,
    },
    /// `For ... Variable`.
    For {
        /// The loop variable.
        var: Place,
        /// First value.
        start: Expr,
        /// Exclusive end.
        stop: Expr,
        /// Increment.
        step: Expr,
        /// Loop body.
        body: Block,
    },
    /// `Skip`.
    Skip(SkipTarget),
    /// `Skip If`.
    SkipIf(Expr, SkipTarget),
    /// `Abort`.
    Abort,
    /// `Abort If`.
    AbortIf(Expr),
    /// `Abort If Condition Is True`.
    AbortIfConditionTrue,
    /// `Abort If Condition Is False`.
    AbortIfConditionFalse,
    /// `Loop`.
    Loop,
    /// `Loop If`.
    LoopIf(Expr),
    /// `Loop If Condition Is True`.
    LoopIfConditionTrue,
    /// `Loop If Condition Is False`.
    LoopIfConditionFalse,
    /// A jump target; removed by flatten.
    Label(LabelId),
    /// `Call Subroutine`.
    CallSubroutine(SubId),
    /// `Start Rule`.
    StartRule(SubId, StartRuleBehavior),
    /// No-op; removable.
    Pass,
    /// A standalone comment line.
    Comment(String),
    /// The `If(True)` that opens a lowered switch; the cases follow at the same depth until a
    /// matching `End`. Never removed or simplified by the optimizer.
    SwitchIf,
    /// Multi-way branch; removed by lowering.
    Switch {
        /// The scrutinee.
        value: Expr,
        /// Cases, in order (they fall through).
        cases: Vec<(Expr, Block)>,
        /// The default case.
        default: Option<Block>,
        /// How many cases precede the default; it may sit anywhere among them.
        default_at: usize,
    },
    /// Leaves the innermost loop or switch; removed by lowering.
    Break,
    /// Jumps to the next loop iteration; removed by lowering.
    Continue,
    /// Loop that runs its body before testing; removed by lowering.
    DoWhile {
        /// Condition.
        cond: Expr,
        /// Loop body.
        body: Block,
    },
}

/// A statement node.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Stmt {
    /// The statement's shape.
    pub(crate) kind: StmtKind,
    /// Where the user wrote it, if known.
    pub(crate) loc: Option<Location>,
    /// Printed as `"text"` on the line above.
    pub(crate) comment: Option<String>,
    /// Warnings suppressed for this statement by the blocks around it.
    pub(crate) suppress: WarningSet,
}

impl Stmt {
    /// Creates a statement located at the caller, without a comment.
    #[track_caller]
    pub(crate) fn new(kind: StmtKind) -> Self {
        Self {
            kind,
            loc: Some(Location::caller()),
            comment: None,
            suppress: WarningSet::default(),
        }
    }
}

impl Stmt {
    /// Whether the statement does nothing at runtime.
    pub(crate) fn is_noop(&self) -> bool {
        matches!(self.kind, StmtKind::Pass | StmtKind::Comment(_))
    }

    /// The blocks nested directly inside this statement.
    pub(crate) fn blocks(&self) -> Vec<&Block> {
        match &self.kind {
            StmtKind::If {
                then, elifs, else_, ..
            } => std::iter::once(then)
                .chain(elifs.iter().map(|(_, b)| b))
                .chain(else_)
                .collect(),
            StmtKind::While { body, .. }
            | StmtKind::For { body, .. }
            | StmtKind::DoWhile { body, .. } => vec![body],
            StmtKind::Switch { cases, default, .. } => {
                cases.iter().map(|(_, b)| b).chain(default).collect()
            }
            _ => Vec::new(),
        }
    }
}

impl Place {
    /// The expressions inside the place (owning player, indices).
    pub(crate) fn exprs(&self) -> Vec<&Expr> {
        match self {
            Self::Global(_) => Vec::new(),
            Self::Player { player, .. } => vec![player],
            Self::Index { base, index } => {
                let mut exprs = base.exprs();
                exprs.push(index);
                exprs
            }
        }
    }
}

impl Stmt {
    /// The expressions of this statement itself, not of nested blocks.
    pub(crate) fn exprs(&self) -> Vec<&Expr> {
        match &self.kind {
            StmtKind::Action { args, .. } => args.iter().collect(),
            StmtKind::Set { target, value } | StmtKind::Modify { target, value, .. } => {
                let mut exprs = target.exprs();
                exprs.push(value);
                exprs
            }
            StmtKind::Chase {
                target,
                dest,
                rate_or_duration,
                reeval,
                ..
            } => {
                let mut exprs = target.exprs();
                exprs.extend([dest, rate_or_duration, reeval]);
                exprs
            }
            StmtKind::StopChasing(place) => place.exprs(),
            StmtKind::If { cond, elifs, .. } => std::iter::once(cond)
                .chain(elifs.iter().map(|(c, _)| c))
                .collect(),
            StmtKind::While { cond, .. }
            | StmtKind::DoWhile { cond, .. }
            | StmtKind::AbortIf(cond)
            | StmtKind::LoopIf(cond) => vec![cond],
            StmtKind::For {
                var,
                start,
                stop,
                step,
                ..
            } => {
                let mut exprs = var.exprs();
                exprs.extend([start, stop, step]);
                exprs
            }
            StmtKind::Skip(SkipTarget::Dynamic(e)) => vec![e],
            StmtKind::SkipIf(cond, target) => {
                let mut exprs = vec![cond];
                if let SkipTarget::Dynamic(e) = target {
                    exprs.push(e);
                }
                exprs
            }
            StmtKind::Switch { value, cases, .. } => std::iter::once(value)
                .chain(cases.iter().map(|(c, _)| c))
                .collect(),
            _ => Vec::new(),
        }
    }
}
