//! The builder a mod records its rules and subroutines on.

use std::cell::Cell;
use std::rc::Rc;

use crate::event::{Ev, EventType};
use crate::ir::{Diagnostics, Location, Program, Rule};
use crate::rule::Rule as RuleBuilder;
use crate::rule_name::{RuleMeta, default_rule_name, module_path};

/// Records the rules and subroutines of a mod in call order.
#[derive(Debug, Default)]
pub struct Builder {
    rules: Vec<Rule>,
    diagnostics: Diagnostics,
    labels: Rc<Cell<u32>>,
    pub(crate) subs: crate::ir::Subs,
    pub(crate) sub_states: Vec<crate::subroutine::SubState>,
    prefix: String,
    template: Option<fn(&RuleMeta) -> String>,
    /// Whether the mod enables text tags.
    pub(crate) tags: bool,
}

// Recording methods are used as the API grows.
#[allow(dead_code)]
impl Builder {
    /// An empty builder naming rules with `template`.
    pub(crate) fn with_template(template: fn(&RuleMeta) -> String) -> Self {
        Self {
            template: Some(template),
            ..Self::default()
        }
    }

    /// Tells the blocks recorded from here on whether the mod enables text tags.
    pub(crate) fn with_tags(mut self, tags: bool) -> Self {
        self.tags = tags;
        self
    }

    /// Runs `build` with `prefix` on the names of the rules and subroutines it defines.
    ///
    /// Groups nest; the inner prefix replaces the outer one.
    pub fn group(&mut self, prefix: impl Into<String>, build: impl FnOnce(&mut Builder)) {
        let outer = std::mem::replace(&mut self.prefix, prefix.into());
        build(self);
        self.prefix = outer;
    }

    /// The final name of a rule recorded at `loc` in the current group.
    pub(crate) fn resolve_name(&self, name: &str, is_delimiter: bool, loc: Location) -> String {
        let meta = RuleMeta {
            name: name.to_owned(),
            prefix: self.prefix.clone(),
            is_delimiter,
            module_path: module_path(loc.file),
        };
        (self.template.unwrap_or(default_rule_name))(&meta)
    }

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
        let loc = Location::caller();
        let mut rule = RuleBuilder::new(Rc::clone(&self.labels));
        rule.tags = self.tags;
        build(&mut rule, Ev::new());
        let name = self.resolve_name(&name.into(), rule.is_delimiter(), loc);
        let rule = rule.finish(name, event, &mut self.diagnostics);
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
