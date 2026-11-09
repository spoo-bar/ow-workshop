//! Subroutines: rules called by name, sharing the caller's event values.

use crate::ir::ErrorKind;
use ow_workshop_data::StartRuleBehavior;

use crate::block::Block;
use crate::builder::Builder;
use crate::check::check_do_while;
use crate::event::{Ev, SubroutineEvent};
use crate::ir::{self, Diagnostics, Location, StmtKind, SubDecl, SubId};

/// A declared subroutine; call it with [`Block::call`] or [`Block::start_rule`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Sub(SubId);

/// Optional settings of a subroutine.
#[derive(Clone, Debug, Default)]
pub struct SubroutineOptions {
    index: Option<u8>,
    rule_name: Option<String>,
}

impl SubroutineOptions {
    /// Default options: automatic index, rule named `Subroutine <name>`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Fixes the subroutine's index (0–127).
    #[must_use]
    pub fn index(mut self, index: u8) -> Self {
        self.index = Some(index);
        self
    }

    /// Names the subroutine's rule.
    #[must_use]
    pub fn rule_name(mut self, name: impl Into<String>) -> Self {
        self.rule_name = Some(name.into());
        self
    }
}

/// What the builder knows about a declared subroutine.
#[derive(Debug)]
pub(crate) struct SubState {
    rule_name: String,
    defined: bool,
}

/// Reports subroutines that were declared but never defined.
pub(crate) fn report_undefined(
    subs: &ir::Subs,
    states: &[SubState],
    diagnostics: &mut Diagnostics,
) {
    for (decl, state) in subs.entries.iter().zip(states) {
        if !state.defined {
            diagnostics.error(
                ErrorKind::UndefinedSubroutine,
                format!("subroutine `{}` is declared but never defined", decl.name),
                decl.location,
            );
        }
    }
}

fn collect_calls(block: &ir::Block, calls: &mut Vec<SubId>) {
    for stmt in &block.stmts {
        if let StmtKind::CallSubroutine(id) = stmt.kind {
            if !calls.contains(&id) {
                calls.push(id);
            }
        }
        for child in stmt.blocks() {
            collect_calls(child, calls);
        }
    }
}

impl Builder {
    /// Declares a subroutine so it can be called before it is defined.
    #[track_caller]
    pub fn declare_subroutine(&mut self, name: impl Into<String>) -> Sub {
        self.declare_subroutine_with(name, SubroutineOptions::new())
    }

    /// Declares a subroutine with options.
    #[track_caller]
    pub fn declare_subroutine_with(
        &mut self,
        name: impl Into<String>,
        options: SubroutineOptions,
    ) -> Sub {
        let name = name.into();
        let mut decl = SubDecl::new(name.clone());
        decl.index = options.index;
        let id = self.subs.declare(decl);
        self.sub_states.push(SubState {
            rule_name: options
                .rule_name
                .unwrap_or_else(|| format!("Subroutine {name}")),
            defined: false,
        });
        Sub(id)
    }

    /// Defines the body of a declared subroutine; the rule is placed at this point of the output.
    #[track_caller]
    pub fn define(&mut self, sub: Sub, body: impl FnOnce(&mut Block, Ev<SubroutineEvent>)) {
        let loc = Location::caller();
        let index = sub.0.0 as usize;
        let (Some(decl), Some(state)) = (self.subs.entries.get(index), self.sub_states.get(index))
        else {
            return;
        };
        let name = decl.name.clone();
        if state.defined {
            self.diagnostics_mut().error(
                ErrorKind::SubroutineDefinedTwice,
                format!("subroutine `{name}` is defined more than once"),
                Some(loc),
            );
            return;
        }
        let rule_name = self.resolve_name(&state.rule_name, false, loc);

        let mut block = Block::with_labels(self.labels());
        block.tags = self.tags;
        body(&mut block, Ev::new());
        let body = block.into_ir();
        check_do_while(&body, &rule_name, self.diagnostics_mut());

        let mut calls = Vec::new();
        collect_calls(&body, &mut calls);
        if let Some(decl) = self.subs.entries.get_mut(index) {
            decl.calls = calls;
        }
        if let Some(state) = self.sub_states.get_mut(index) {
            state.defined = true;
        }
        let mut rule = ir::Rule::new(rule_name, ir::Event::Subroutine(sub.0), body);
        rule.location = Some(loc);
        self.push_rule(rule);
    }

    /// Declares and defines a subroutine.
    #[track_caller]
    pub fn subroutine(
        &mut self,
        name: impl Into<String>,
        body: impl FnOnce(&mut Block, Ev<SubroutineEvent>),
    ) -> Sub {
        self.subroutine_with(name, SubroutineOptions::new(), body)
    }

    /// Declares and defines a subroutine with options.
    #[track_caller]
    pub fn subroutine_with(
        &mut self,
        name: impl Into<String>,
        options: SubroutineOptions,
        body: impl FnOnce(&mut Block, Ev<SubroutineEvent>),
    ) -> Sub {
        let sub = self.declare_subroutine_with(name, options);
        self.define(sub, body);
        sub
    }
}

impl Block {
    /// `Call Subroutine`: runs the subroutine inline and waits for it.
    #[track_caller]
    pub fn call(&mut self, sub: Sub) {
        self.push(StmtKind::CallSubroutine(sub.0));
    }

    /// `Start Rule`: runs the subroutine concurrently.
    #[track_caller]
    pub fn start_rule(&mut self, sub: Sub, behavior: StartRuleBehavior) {
        self.push(StmtKind::StartRule(sub.0, behavior));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Event, Program};
    use crate::prelude::*;

    fn finish(ws: Builder) -> (Program, Diagnostics) {
        ws.finish(Program::default())
    }

    #[test]
    fn define_in_place_and_call() {
        let mut ws = Builder::new();
        let spawn = ws.subroutine("spawn", |b, _| b.wait(1));
        ws.rule("Main", OngoingGlobal, |r, _| {
            r.actions(|b| {
                b.call(spawn);
                b.start_rule(spawn, StartRuleBehavior::Restart);
            });
        });
        let (program, diags) = finish(ws);
        assert_eq!(diags.0, vec![]);
        assert_eq!(program.rules.len(), 2);
        assert_eq!(program.rules[0].name, "Subroutine spawn");
        assert!(matches!(program.rules[0].event, Event::Subroutine(_)));
        assert!(program.rules[0].validate().is_ok());
        let stmts = &program.rules[1].body.stmts;
        assert!(matches!(stmts[0].kind, StmtKind::CallSubroutine(_)));
        assert!(matches!(
            stmts[1].kind,
            StmtKind::StartRule(_, StartRuleBehavior::Restart)
        ));
        assert_eq!(program.subroutines.entries[0].name, "spawn");
        assert!(program.subroutines.entries[0].location.is_some());
    }

    #[test]
    fn call_before_define() {
        let mut ws = Builder::new();
        let later = ws.declare_subroutine("later");
        ws.rule("Main", OngoingGlobal, |r, _| r.actions(|b| b.call(later)));
        ws.define(later, |b, _| b.pass());
        let (program, diags) = finish(ws);
        assert_eq!(diags.0, vec![]);
        let names: Vec<_> = program.rules.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, ["Main", "Subroutine later"]);
    }

    #[test]
    fn options_set_the_index_and_rule_name() {
        let mut ws = Builder::new();
        ws.subroutine_with(
            "fixed",
            SubroutineOptions::new().index(7).rule_name("Do the thing"),
            |_, _| {},
        );
        let (program, _) = finish(ws);
        assert_eq!(program.rules[0].name, "Do the thing");
        assert_eq!(program.subroutines.entries[0].index, Some(7));
    }

    #[test]
    fn calls_are_recorded_in_the_call_graph() {
        let mut ws = Builder::new();
        let inner = ws.subroutine("inner", |_, _| {});
        let outer = ws.subroutine("outer", |b, _| {
            b.if_(true, |b| b.call(inner));
            b.start_rule(inner, StartRuleBehavior::Noop);
        });
        let _ = outer;
        let (program, _) = finish(ws);
        assert_eq!(program.subroutines.entries[1].calls.len(), 1);
        assert_eq!(program.subroutines.entries[0].calls.len(), 0);
    }

    #[test]
    fn subroutines_see_every_event_value() {
        let mut ws = Builder::new();
        ws.subroutine("uses events", |b, ev| {
            b.wait(ev.event_damage());
            let _ = (ev.event_player(), ev.healer(), ev.was_critical_hit());
        });
        let (_, diags) = finish(ws);
        assert_eq!(diags.0, vec![]);
    }

    #[test]
    fn undefined_subroutine_is_an_error() {
        let mut ws = Builder::new();
        ws.declare_subroutine("ghost");
        let (_, diags) = finish(ws);
        assert_eq!(diags.0.len(), 1);
        assert_eq!(diags.0[0].code(), "undefined-subroutine");
        assert!(diags.0[0].location.is_some());
    }

    #[test]
    fn defining_twice_is_an_error() {
        let mut ws = Builder::new();
        let s = ws.subroutine("s", |_, _| {});
        ws.define(s, |_, _| {});
        let (program, diags) = finish(ws);
        assert_eq!(diags.0[0].code(), "subroutine-defined-twice");
        assert_eq!(program.rules.len(), 1);
    }

    #[test]
    fn misplaced_do_while_in_a_subroutine() {
        let mut ws = Builder::new();
        ws.subroutine("s", |b, _| {
            b.wait(1);
            b.do_while(true, |b| b.pass());
        });
        let (_, diags) = finish(ws);
        assert_eq!(diags.0[0].code(), "misplaced-do-while");
    }
}
