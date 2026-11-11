//! The recorder that turns a mod's rule definitions into IR; internal to the compiler.

use std::cell::Cell;
use std::rc::Rc;

use crate::event::{Ev, EventType};
use crate::ir::{Diagnostics, Location, Program, Rule};
use crate::rule::Rule as RuleBuilder;
use crate::rule_name::{RuleMeta, default_rule_name, module_path};
use crate::workshop_debug::DebugField;

/// Records the rules and subroutines of a mod in definition order.
#[derive(Debug, Default)]
pub(crate) struct Builder {
    rules: Vec<Rule>,
    diagnostics: Diagnostics,
    labels: Rc<Cell<u32>>,
    pub(crate) subs: crate::ir::Subs,
    pub(crate) sub_states: Vec<crate::subroutine::SubState>,
    template: Option<fn(&RuleMeta) -> String>,
    /// Whether the mod enables text tags.
    pub(crate) tags: bool,
    /// Fields registered with [`Builder::debug`].
    debug_fields: Vec<DebugField>,
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

    /// The final name of a rule recorded at `loc` in the current group.
    pub(crate) fn resolve_name(&self, name: &str, is_delimiter: bool, loc: Location) -> String {
        let meta = RuleMeta {
            name: name.to_owned(),
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

    /// Records a rule listening to `event` that was defined at `loc`.
    pub(crate) fn record_rule<E: EventType>(
        &mut self,
        loc: Location,
        name: &str,
        event: E,
        build: impl FnOnce(&mut RuleBuilder<E>, Ev<E>),
    ) {
        let mut rule = RuleBuilder::new(Rc::clone(&self.labels));
        rule.tags = self.tags;
        build(&mut rule, Ev::new());
        let name = self.resolve_name(name, rule.is_delimiter(), loc);
        let rule = rule.finish(loc, name, event, &mut self.diagnostics);
        self.rules.push(rule);
    }

    /// Defines a rule listening to `event`; rules appear in output in call order.
    #[track_caller]
    pub(crate) fn rule<E: EventType>(
        &mut self,
        name: impl Into<String>,
        event: E,
        build: impl FnOnce(&mut RuleBuilder<E>, Ev<E>),
    ) {
        self.record_rule(Location::caller(), &name.into(), event, build);
    }

    /// Records every definition in `defs`, in order.
    pub(crate) fn run(&mut self, defs: Vec<crate::rule_def::RuleDef<'_>>) {
        crate::subroutine::declare_registered(self);
        for def in defs {
            def.run(self);
        }
    }

    /// Adds the fields of `fields` to the debug panels, after those of the mod's `Globals` and
    /// `Players`.
    pub(crate) fn add_debug_fields(&mut self, fields: Vec<DebugField>) {
        self.debug_fields.extend(fields);
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
        let mut program = Program {
            rules: self.rules,
            subroutines: self.subs,
            ..program
        };
        program.debug_fields.extend(self.debug_fields);
        (program, diagnostics)
    }
}
