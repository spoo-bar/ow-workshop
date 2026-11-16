use crate::{ActionId, ChaseRateReeval, ChaseTimeReeval, Global, ModifyOp, PlayerVar, Value};

/// How a chase action moves a variable toward its destination.
#[derive(Clone, Debug, PartialEq)]
pub enum Chase {
    /// `... At Rate`: change by `rate` per second.
    AtRate {
        /// Units per second.
        rate: Value,
        /// Which arguments are re-read while chasing.
        reevaluation: ChaseRateReeval,
    },
    /// `... Over Time`: arrive after `duration` seconds.
    OverTime {
        /// Seconds until arrival.
        duration: Value,
        /// Which arguments are re-read while chasing.
        reevaluation: ChaseTimeReeval,
    },
}

impl Chase {
    /// Chase at `rate` per second, re-reading destination and rate.
    pub fn at_rate(rate: impl Into<Value>) -> Self {
        Self::AtRate {
            rate: rate.into(),
            reevaluation: ChaseRateReeval::DestinationAndRate,
        }
    }

    /// Chase for `duration` seconds, re-reading destination and duration.
    pub fn over_time(duration: impl Into<Value>) -> Self {
        Self::OverTime {
            duration: duration.into(),
            reevaluation: ChaseTimeReeval::DestinationAndDuration,
        }
    }
}

/// One statement in a rule's `actions` block.
#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    /// A workshop action, such as `Set Damage Dealt(Event Player, 0)`.
    Call {
        /// Which action.
        id: ActionId,
        /// Its arguments, in workshop order.
        args: Vec<Value>,
    },
    /// `Set Global Variable(var, value)`.
    SetGlobal {
        /// The variable written.
        var: Global,
        /// The new value.
        value: Value,
    },
    /// `Set Player Variable(player, var, value)`.
    SetPlayerVar {
        /// Whose variable. May be an array of players.
        player: Value,
        /// The variable written.
        var: PlayerVar,
        /// The new value.
        value: Value,
    },
    /// `Set Global Variable At Index(var, index, value)`.
    SetGlobalAt {
        /// The array variable written.
        var: Global,
        /// The element.
        index: Value,
        /// The new value.
        value: Value,
    },
    /// `Set Player Variable At Index(player, var, index, value)`.
    SetPlayerVarAt {
        /// Whose variable. May be an array of players.
        player: Value,
        /// The array variable written.
        var: PlayerVar,
        /// The element.
        index: Value,
        /// The new value.
        value: Value,
    },
    /// `Modify Global Variable(var, op, value)`, or `... At Index` when `index` is set.
    ModifyGlobal {
        /// The variable modified.
        var: Global,
        /// The element, when modifying one.
        index: Option<Value>,
        /// The operation.
        op: ModifyOp,
        /// The operand.
        value: Value,
    },
    /// `Modify Player Variable(player, var, op, value)`, or `... At Index` when `index` is set.
    ModifyPlayerVar {
        /// Whose variable. May be an array of players.
        player: Value,
        /// The variable modified.
        var: PlayerVar,
        /// The element, when modifying one.
        index: Option<Value>,
        /// The operation.
        op: ModifyOp,
        /// The operand.
        value: Value,
    },
    /// `Chase Global Variable At Rate` or `Over Time`.
    ChaseGlobal {
        /// The variable chased.
        var: Global,
        /// The value it moves toward.
        destination: Value,
        /// The speed.
        mode: Chase,
    },
    /// `Chase Player Variable At Rate` or `Over Time`.
    ChasePlayerVar {
        /// Whose variable. May be an array of players.
        player: Value,
        /// The variable chased.
        var: PlayerVar,
        /// The value it moves toward.
        destination: Value,
        /// The speed.
        mode: Chase,
    },
    /// `Stop Chasing Global Variable(var)`.
    StopChasingGlobal(Global),
    /// `Stop Chasing Player Variable(player, var)`.
    StopChasingPlayerVar {
        /// Whose variable. May be an array of players.
        player: Value,
        /// The variable no longer chased.
        var: PlayerVar,
    },
    /// `If` with an optional `Else`. The emitter writes `End`.
    If {
        /// The condition.
        cond: Value,
        /// Actions run when the condition holds.
        then: Vec<Action>,
        /// Actions run otherwise. Empty means no `Else`.
        otherwise: Vec<Action>,
    },
    /// `Call Subroutine(name)`.
    CallSubroutine(String),
    /// An action with a comment line above it. Comments are dropped in release builds.
    Commented {
        /// The comment text.
        comment: String,
        /// The action it describes.
        action: Box<Action>,
    },
}

impl Action {
    /// A workshop action with arguments.
    pub fn call(id: ActionId, args: impl IntoIterator<Item = Value>) -> Self {
        Self::Call {
            id,
            args: args.into_iter().collect(),
        }
    }

    /// `If(cond)` followed by `then` and `End`.
    pub fn when(cond: Value, then: impl IntoIterator<Item = Action>) -> Self {
        Self::If {
            cond,
            then: then.into_iter().collect(),
            otherwise: Vec::new(),
        }
    }

    /// Puts a comment line above this action.
    #[must_use]
    pub fn commented(self, comment: impl Into<String>) -> Self {
        Self::Commented {
            comment: comment.into(),
            action: Box::new(self),
        }
    }
}
