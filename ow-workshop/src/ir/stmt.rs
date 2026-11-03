//! Untyped statements.

use ow_workshop_data::{ActionId, StartRuleBehavior};

use super::{Expr, LabelId, Location, OptOverride, SubId, VarId};

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
    /// Multi-way branch; removed by lowering.
    Switch {
        /// The scrutinee.
        value: Expr,
        /// Cases, in order (they fall through).
        cases: Vec<(Expr, Block)>,
        /// The default case.
        default: Option<Block>,
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
}

impl Stmt {
    /// Creates a statement located at the caller, without a comment.
    #[track_caller]
    pub(crate) fn new(kind: StmtKind) -> Self {
        Self {
            kind,
            loc: Some(Location::caller()),
            comment: None,
        }
    }
}
