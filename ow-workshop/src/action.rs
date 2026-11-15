use crate::{ActionId, Global, PlayerVar, Value};

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
