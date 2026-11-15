use crate::{DebugOpts, Value};

/// A handle to a global variable.
#[derive(Clone, Debug, PartialEq)]
pub struct Global {
    /// The name in the script's `variables` block.
    pub name: &'static str,
    /// How it is shown in debug builds.
    pub debug: Option<DebugOpts>,
}

/// A handle to a player variable.
#[derive(Clone, Debug, PartialEq)]
pub struct PlayerVar {
    /// The name in the script's `variables` block.
    pub name: &'static str,
    /// How it is shown in debug builds.
    pub debug: Option<DebugOpts>,
}

impl PlayerVar {
    /// A read of this variable on `player`.
    pub fn of(&self, player: Value) -> Value {
        Value::PlayerVar {
            player: Box::new(player),
            var: self.clone(),
        }
    }
}

/// A mod's global variables. Derive it on a struct whose fields are [`Global`].
pub trait Globals {
    /// Creates the handles.
    fn declare() -> Self;
    /// Every variable, in declaration order. The position is the variable's index.
    fn vars(&self) -> Vec<Global>;
}

/// A mod's player variables. Derive it on a struct whose fields are [`PlayerVar`].
pub trait Players {
    /// Creates the handles.
    fn declare() -> Self;
    /// Every variable, in declaration order. The position is the variable's index.
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
