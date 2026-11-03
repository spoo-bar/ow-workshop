//! The builder a mod records its rules and subroutines on.

use std::cell::Cell;
use std::rc::Rc;

use crate::event::{Ev, EventType};
use crate::ir::{Diagnostics, Program, Rule};
use crate::rule::Rule as RuleBuilder;

/// Records the rules and subroutines of a mod in call order.
#[derive(Debug, Default)]
pub struct Builder {
    rules: Vec<Rule>,
    diagnostics: Diagnostics,
    labels: Rc<Cell<u32>>,
    pub(crate) subs: crate::ir::Subs,
    pub(crate) sub_states: Vec<crate::subroutine::SubState>,
}

// Recording methods are used as the API grows.
#[allow(dead_code)]
impl Builder {
    /// The label counter shared by the blocks of this builder.
    pub(crate) fn labels(&self) -> Rc<Cell<u32>> {
        Rc::clone(&self.labels)
    }

    /// An empty builder.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Defines a rule listening to `event`; rules appear in output in call order.
    #[track_caller]
    pub fn rule<E: EventType>(
        &mut self,
        name: impl Into<String>,
        event: E,
        build: impl FnOnce(&mut RuleBuilder<E>, Ev<E>),
    ) {
        let mut rule = RuleBuilder::new(Rc::clone(&self.labels));
        build(&mut rule, Ev::new());
        let rule = rule.finish(name.into(), event, &mut self.diagnostics);
        self.rules.push(rule);
    }

    /// Appends a finished rule.
    pub(crate) fn push_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    /// Diagnostics found while recording.
    pub(crate) fn diagnostics_mut(&mut self) -> &mut Diagnostics {
        &mut self.diagnostics
    }

    /// Finishes recording, returning the program and the diagnostics found so far.
    pub(crate) fn finish(self, program: Program) -> (Program, Diagnostics) {
        let mut diagnostics = self.diagnostics;
        crate::subroutine::report_undefined(&self.subs, &self.sub_states, &mut diagnostics);
        let program = Program {
            rules: self.rules,
            subroutines: self.subs,
            ..program
        };
        (program, diagnostics)
    }
}
