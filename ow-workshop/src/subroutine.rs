//! Subroutines: rules called by name, sharing the caller's event values.

use crate::ir::ErrorKind;
use std::cell::RefCell;

use ow_workshop_data::StartRuleBehavior;

use crate::block::Block;
use crate::builder::Builder;
use crate::check::check_do_while;
use crate::event::{Ev, SubroutineEvent};
use crate::ir::{self, Diagnostics, Location, StmtKind, SubDecl, SubId};
use crate::rule_def::RuleDef;

thread_local! {
    /// The subroutines named while a mod's rules are being listed, in order of first use.
    static REGISTRY: RefCell<Vec<(String, Location)>> = const { RefCell::new(Vec::new()) };
}

/// Forgets the subroutines of the previous compile.
pub(crate) fn reset_registry() {
    REGISTRY.with(|r| r.borrow_mut().clear());
}

/// Declares on `ws` every subroutine named since [`reset_registry`], so ids match handles.
pub(crate) fn declare_registered(ws: &mut Builder) {
    let named = REGISTRY.with(|r| r.borrow().clone());
    for (name, loc) in named.into_iter().skip(ws.subs.entries.len()) {
        let mut decl = SubDecl::new(name.clone());
        decl.location = Some(loc);
        ws.subs.declare(decl);
        ws.sub_states.push(SubState {
            rule_name: format!("Subroutine {name}"),
            defined: false,
        });
    }
}

/// A subroutine; call it with [`Block::call`] or [`Block::start_rule`], and define its body
/// with [`subroutine`].
///
/// Make one with [`Sub::named`] at the top of [`Workshop::rules`](crate::Workshop::rules) so the
/// rules can call it whether or not they are listed before its definition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Sub(SubId);

impl Sub {
    /// The subroutine called `name`; naming it again gives the same handle.
    #[track_caller]
    pub fn named(name: impl Into<String>) -> Self {
        let name = name.into();
        let loc = Location::caller();
        let id = REGISTRY.with(|r| {
            let mut named = r.borrow_mut();
            let index = named
                .iter()
                .position(|(n, _)| *n == name)
                .unwrap_or_else(|| {
                    named.push((name, loc));
                    named.len() - 1
                });
            u32::try_from(index).unwrap_or(u32::MAX)
        });
        Self(SubId(id))
    }
}

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
    /// Defines the body of a subroutine; the rule is placed at this point of the output.
    pub(crate) fn define_with(
        &mut self,
        loc: Location,
        sub: Sub,
        options: SubroutineOptions,
        body: impl FnOnce(&mut Block, Ev<SubroutineEvent>),
    ) {
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
        let rule_name = options.rule_name.unwrap_or_else(|| state.rule_name.clone());
        let rule_name = self.resolve_name(&rule_name, false, loc);

        let mut block = Block::with_labels(self.labels());
        block.tags = self.tags;
        body(&mut block, Ev::new());
        let body = block.into_ir();
        check_do_while(&body, &rule_name, self.diagnostics_mut());

        let mut calls = Vec::new();
        collect_calls(&body, &mut calls);
        if let Some(decl) = self.subs.entries.get_mut(index) {
            decl.calls = calls;
            decl.index = options.index;
        }
        if let Some(state) = self.sub_states.get_mut(index) {
            state.defined = true;
        }
        let mut rule = ir::Rule::new(rule_name, ir::Event::Subroutine(sub.0), body);
        rule.location = Some(loc);
        self.push_rule(rule);
    }
}

/// Defines the body of `sub` as a rule.
#[track_caller]
pub fn subroutine<'a>(
    sub: Sub,
    body: impl FnOnce(&mut Block, Ev<SubroutineEvent>) + 'a,
) -> RuleDef<'a> {
    subroutine_with(sub, SubroutineOptions::new(), body)
}

/// Defines the body of `sub` as a rule, with options.
#[track_caller]
pub fn subroutine_with<'a>(
    sub: Sub,
    options: SubroutineOptions,
    body: impl FnOnce(&mut Block, Ev<SubroutineEvent>) + 'a,
) -> RuleDef<'a> {
    let loc = Location::caller();
    RuleDef::new(move |ws| ws.define_with(loc, sub, options, body))
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

    fn compile(defs: impl FnOnce() -> Vec<RuleDef<'static>>) -> (Program, Diagnostics) {
        reset_registry();
        let defs = defs();
        let mut ws = Builder::new();
        ws.run(defs);
        ws.finish(Program::default())
    }

    #[test]
    fn define_and_call() {
        let (program, diags) = compile(|| {
            let spawn = Sub::named("spawn");
            vec![
                subroutine(spawn, |b, _| b.wait(1)),
                rule("Main", OngoingGlobal, move |r, _| {
                    r.actions(|b| {
                        b.call(spawn);
                        b.start_rule(spawn, StartRuleBehavior::Restart);
                    });
                }),
            ]
        });
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
    fn rule_order_does_not_matter() {
        let (program, diags) = compile(|| {
            let later = Sub::named("later");
            vec![
                rule("Main", OngoingGlobal, move |r, _| {
                    r.actions(|b| b.call(later));
                }),
                subroutine(later, |b, _| b.pass()),
            ]
        });
        assert_eq!(diags.0, vec![]);
        let names: Vec<_> = program.rules.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, ["Main", "Subroutine later"]);
    }

    #[test]
    fn naming_twice_gives_the_same_handle() {
        reset_registry();
        assert_eq!(Sub::named("a"), Sub::named("a"));
        assert_ne!(Sub::named("a"), Sub::named("b"));
    }

    #[test]
    fn options_set_the_index_and_rule_name() {
        let (program, _) = compile(|| {
            vec![subroutine_with(
                Sub::named("fixed"),
                SubroutineOptions::new().index(7).rule_name("Do the thing"),
                |_, _| {},
            )]
        });
        assert_eq!(program.rules[0].name, "Do the thing");
        assert_eq!(program.subroutines.entries[0].index, Some(7));
    }

    #[test]
    fn calls_are_recorded_in_the_call_graph() {
        let (program, _) = compile(|| {
            let inner = Sub::named("inner");
            let outer = Sub::named("outer");
            vec![
                subroutine(inner, |_, _| {}),
                subroutine(outer, move |b, _| {
                    b.if_(true, |b| b.call(inner));
                    b.start_rule(inner, StartRuleBehavior::Noop);
                }),
            ]
        });
        assert_eq!(program.subroutines.entries[1].calls.len(), 1);
        assert_eq!(program.subroutines.entries[0].calls.len(), 0);
    }

    #[test]
    fn subroutines_see_every_event_value() {
        let (_, diags) = compile(|| {
            vec![subroutine(Sub::named("uses events"), |b, ev| {
                b.wait(ev.event_damage());
                let _ = (ev.event_player(), ev.healer(), ev.was_critical_hit());
            })]
        });
        assert_eq!(diags.0, vec![]);
    }

    #[test]
    fn undefined_subroutine_is_an_error() {
        let (_, diags) = compile(|| {
            let ghost = Sub::named("ghost");
            vec![rule("Main", OngoingGlobal, move |r, _| {
                r.actions(|b| b.call(ghost));
            })]
        });
        assert_eq!(diags.0.len(), 1);
        assert_eq!(diags.0[0].code(), "undefined-subroutine");
        assert!(diags.0[0].location.is_some());
    }

    #[test]
    fn defining_twice_is_an_error() {
        let (program, diags) = compile(|| {
            let s = Sub::named("s");
            vec![subroutine(s, |_, _| {}), subroutine(s, |_, _| {})]
        });
        assert_eq!(diags.0[0].code(), "subroutine-defined-twice");
        assert_eq!(program.rules.len(), 1);
    }

    #[test]
    fn misplaced_do_while_in_a_subroutine() {
        let (_, diags) = compile(|| {
            vec![subroutine(Sub::named("s"), |b, _| {
                b.wait(1);
                b.do_while(true, |b| b.pass());
            })]
        });
        assert_eq!(diags.0[0].code(), "misplaced-do-while");
    }
}
