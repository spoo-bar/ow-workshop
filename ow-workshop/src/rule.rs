use crate::{Action, CompareOp, DebugOpts, Event, Value, WorkshopDebug};

/// One line of a rule's `conditions` block: `lhs op rhs`.
#[derive(Clone, Debug, PartialEq)]
pub struct Condition {
    /// Left operand.
    pub lhs: Value,
    /// Operator.
    pub op: CompareOp,
    /// Right operand.
    pub rhs: Value,
}

/// A workshop rule.
///
/// Implement it on your own type, or use [`BasicRule`]. A rule must not depend
/// on a particular [`Workshop`](crate::Workshop), so it can be shared between mods.
pub trait Rule: WorkshopDebug {
    /// The rule's name.
    fn name(&self) -> String;
    /// Disabled rules are written as `disabled rule` in debug builds and left out of release builds.
    fn enabled(&self) -> bool {
        true
    }
    /// The triggering events. Each event produces one rule in the script.
    fn events(&self) -> Vec<Event>;
    /// The conditions, all of which must hold.
    fn conditions(&self) -> Vec<Condition> {
        Vec::new()
    }
    /// The actions.
    fn actions(&self) -> Vec<Action>;
}

/// A subroutine that rules call with [`Action::CallSubroutine`] or [`Action::StartRule`].
///
/// A mod lists its subroutines in [`Workshop::subroutines`](crate::Workshop::subroutines).
/// Each one is written to the `subroutines` block and as a rule with the `Subroutine` event.
pub trait Subroutine: WorkshopDebug {
    /// The identifier used in `Call Subroutine` and the `subroutines` block.
    fn name(&self) -> String;
    /// The title of the subroutine's rule. Free text, independent of [`name`](Self::name).
    fn title(&self) -> String {
        format!("Subroutine: {}", self.name())
    }
    /// The actions.
    fn actions(&self) -> Vec<Action>;
}

/// A rule held as plain data.
#[derive(Clone, Debug, PartialEq)]
pub struct BasicRule {
    /// The rule's name.
    pub name: String,
    /// Whether the rule runs.
    pub enabled: bool,
    /// The triggering events.
    pub events: Vec<Event>,
    /// The conditions.
    pub conditions: Vec<Condition>,
    /// The actions.
    pub actions: Vec<Action>,
    /// How it is shown in debug builds.
    pub debug: Option<DebugOpts>,
}

impl BasicRule {
    /// An enabled rule with no conditions, actions or debug display.
    pub fn new(name: impl Into<String>, event: Event) -> Self {
        Self {
            name: name.into(),
            enabled: true,
            events: vec![event],
            conditions: Vec::new(),
            actions: Vec::new(),
            debug: None,
        }
    }
}

impl WorkshopDebug for BasicRule {
    fn debug_opts(&self) -> Option<DebugOpts> {
        self.debug.clone()
    }
}

impl Rule for BasicRule {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn enabled(&self) -> bool {
        self.enabled
    }
    fn events(&self) -> Vec<Event> {
        self.events.clone()
    }
    fn conditions(&self) -> Vec<Condition> {
        self.conditions.clone()
    }
    fn actions(&self) -> Vec<Action> {
        self.actions.clone()
    }
}

/// A subroutine held as plain data.
#[derive(Clone, Debug, PartialEq)]
pub struct BasicSubroutine {
    /// The identifier used in `Call Subroutine`.
    pub name: String,
    /// The title of the subroutine's rule.
    pub title: String,
    /// The actions.
    pub actions: Vec<Action>,
    /// How it is shown in debug builds.
    pub debug: Option<DebugOpts>,
}

impl BasicSubroutine {
    /// A subroutine titled `Subroutine: <name>`, with no actions or debug display.
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            title: format!("Subroutine: {name}"),
            name,
            actions: Vec::new(),
            debug: None,
        }
    }

    /// Replaces the rule title.
    #[must_use]
    pub fn titled(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
}

impl WorkshopDebug for BasicSubroutine {
    fn debug_opts(&self) -> Option<DebugOpts> {
        self.debug.clone()
    }
}

impl Subroutine for BasicSubroutine {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn title(&self) -> String {
        self.title.clone()
    }
    fn actions(&self) -> Vec<Action> {
        self.actions.clone()
    }
}
