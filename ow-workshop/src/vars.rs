use crate::{Action, Chase, DebugOpts, ModifyOp, Value};

/// A handle to a global variable.
#[derive(Clone, Debug, PartialEq)]
pub struct Global {
    /// The name in the script's `variables` block.
    pub name: &'static str,
    /// The slot in the script's `variables` block, from 0 to 127.
    pub index: u8,
    /// How it is shown in debug builds.
    pub debug: Option<DebugOpts>,
}

/// A handle to a player variable.
#[derive(Clone, Debug, PartialEq)]
pub struct PlayerVar {
    /// The name in the script's `variables` block.
    pub name: &'static str,
    /// The slot in the script's `variables` block, from 0 to 127.
    pub index: u8,
    /// How it is shown in debug builds.
    pub debug: Option<DebugOpts>,
}

impl Global {
    /// A read of this variable.
    pub fn get(&self) -> Value {
        self.into()
    }

    /// `Set Global Variable(self, value)`.
    pub fn set(&self, value: impl Into<Value>) -> Action {
        Action::SetGlobal {
            var: self.clone(),
            value: value.into(),
        }
    }

    /// `Set Global Variable At Index(self, index, value)`.
    pub fn set_at(&self, index: impl Into<Value>, value: impl Into<Value>) -> Action {
        Action::SetGlobalAt {
            var: self.clone(),
            index: index.into(),
            value: value.into(),
        }
    }

    /// `Modify Global Variable(self, op, value)`.
    pub fn modify(&self, op: ModifyOp, value: impl Into<Value>) -> Action {
        Action::ModifyGlobal {
            var: self.clone(),
            index: None,
            op,
            value: value.into(),
        }
    }

    /// `Modify Global Variable At Index(self, index, op, value)`.
    pub fn modify_at(
        &self,
        index: impl Into<Value>,
        op: ModifyOp,
        value: impl Into<Value>,
    ) -> Action {
        Action::ModifyGlobal {
            var: self.clone(),
            index: Some(index.into()),
            op,
            value: value.into(),
        }
    }

    /// `Chase Global Variable At Rate` or `Chase Global Variable Over Time`.
    pub fn chase(&self, destination: impl Into<Value>, mode: Chase) -> Action {
        Action::ChaseGlobal {
            var: self.clone(),
            destination: destination.into(),
            mode,
        }
    }

    /// `Stop Chasing Global Variable(self)`.
    pub fn stop_chasing(&self) -> Action {
        Action::StopChasingGlobal(self.clone())
    }
}

impl PlayerVar {
    /// A read of this variable on `player`.
    pub fn of(&self, player: Value) -> Value {
        Value::PlayerVar {
            player: Box::new(player),
            var: self.clone(),
        }
    }

    /// `Set Player Variable(player, self, value)`. `player` may be any player value.
    pub fn set(&self, player: Value, value: impl Into<Value>) -> Action {
        Action::SetPlayerVar {
            player,
            var: self.clone(),
            value: value.into(),
        }
    }

    /// `Set Player Variable At Index(player, self, index, value)`.
    pub fn set_at(
        &self,
        player: Value,
        index: impl Into<Value>,
        value: impl Into<Value>,
    ) -> Action {
        Action::SetPlayerVarAt {
            player,
            var: self.clone(),
            index: index.into(),
            value: value.into(),
        }
    }

    /// `Modify Player Variable(player, self, op, value)`.
    pub fn modify(&self, player: Value, op: ModifyOp, value: impl Into<Value>) -> Action {
        Action::ModifyPlayerVar {
            player,
            var: self.clone(),
            index: None,
            op,
            value: value.into(),
        }
    }

    /// `Modify Player Variable At Index(player, self, index, op, value)`.
    pub fn modify_at(
        &self,
        player: Value,
        index: impl Into<Value>,
        op: ModifyOp,
        value: impl Into<Value>,
    ) -> Action {
        Action::ModifyPlayerVar {
            player,
            var: self.clone(),
            index: Some(index.into()),
            op,
            value: value.into(),
        }
    }

    /// `Chase Player Variable At Rate` or `Chase Player Variable Over Time`.
    pub fn chase(&self, player: Value, destination: impl Into<Value>, mode: Chase) -> Action {
        Action::ChasePlayerVar {
            player,
            var: self.clone(),
            destination: destination.into(),
            mode,
        }
    }

    /// `Stop Chasing Player Variable(player, self)`.
    pub fn stop_chasing(&self, player: Value) -> Action {
        Action::StopChasingPlayerVar {
            player,
            var: self.clone(),
        }
    }
}

/// A mod's global variables. Derive it on a struct whose fields are [`Global`].
pub trait Globals {
    /// Creates the handles.
    fn declare() -> Self;
    /// Every variable, in declaration order. Each knows its own index.
    fn vars(&self) -> Vec<Global>;
}

/// A mod's player variables. Derive it on a struct whose fields are [`PlayerVar`].
pub trait Players {
    /// Creates the handles.
    fn declare() -> Self;
    /// Every variable, in declaration order. Each knows its own index.
    fn vars(&self) -> Vec<PlayerVar>;
}

impl Globals for () {
    fn declare() -> Self {}
    fn vars(&self) -> Vec<Global> {
        Vec::new()
    }
}

impl Players for () {
    fn declare() -> Self {}
    fn vars(&self) -> Vec<PlayerVar> {
        Vec::new()
    }
}
