//! The transpile-time debug log of a debug build.
//!
//! The core does no I/O: the log is data on [`CompileOutput`](crate::CompileOutput), and
//! [`main!`](crate::main) prints it to stderr, so `cargo run > mod.txt` still captures only the
//! workshop code.

use std::fmt::{self, Write as _};

use crate::count::Counts;
use crate::ir::MAX_SLOTS;
use crate::ir::{Block, Diagnostic, Event, Location, Program, Rule, StmtKind, SubId};
use crate::lower::debug_panels::PANEL_RULES;
use crate::lower::visit::walk_block;
use crate::workshop_debug::Scope;

/// What a debug build knows about the mod it compiled.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct DebugLog {
    /// Every variable, globals first.
    pub variables: Vec<DebugVariable>,
    /// Every subroutine.
    pub subroutines: Vec<DebugSubroutine>,
    /// Every rule, in output order.
    pub rules: Vec<DebugRule>,
    /// Element and slot totals.
    pub totals: DebugTotals,
    /// The same warnings as `CompileOutput::warnings`.
    pub warnings: Vec<Diagnostic>,
}

/// A variable in the [`DebugLog`].
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[allow(clippy::struct_excessive_bools)]
pub struct DebugVariable {
    /// The index it was given in the workshop.
    pub index: u8,
    /// Its workshop name.
    pub name: String,
    /// Global or player.
    pub scope: Scope,
    /// Where it was declared in the Rust source.
    pub location: Option<Location>,
    /// The label of its [`WorkshopDebug`](crate::WorkshopDebug) field, if it has one.
    pub label: Option<&'static str>,
    /// Chased with `Chase … Variable`.
    pub chased: bool,
    /// Used as a `For` loop variable.
    pub loop_var: bool,
    /// Read in a rule condition.
    pub in_condition: bool,
    /// Created by the compiler rather than the mod.
    pub internal: bool,
}

/// A subroutine in the [`DebugLog`].
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct DebugSubroutine {
    /// The index it was given in the workshop.
    pub index: u8,
    /// Its workshop name.
    pub name: String,
    /// The names of the rules that call or start it, in output order.
    pub callers: Vec<String>,
}

/// A rule in the [`DebugLog`].
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct DebugRule {
    /// The rule's name.
    pub name: String,
    /// Its event, as the workshop calls it.
    pub event: String,
    /// The elements it uses.
    pub elements: usize,
    /// Where the mod defines it.
    pub location: Option<Location>,
    /// Added by the compiler (setup, page fillers, debug panels) rather than written by the mod.
    pub generated: bool,
}

/// The totals in the [`DebugLog`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct DebugTotals {
    /// Elements the output uses.
    pub elements: usize,
    /// Elements it would use without the debug panels.
    pub elements_without_panels: usize,
    /// Global variables used, of 128.
    pub globals: usize,
    /// Player variables used, of 128.
    pub players: usize,
    /// Subroutines used, of 128.
    pub subroutines: usize,
}

fn event_name(event: Event) -> String {
    match event {
        Event::OngoingGlobal => "Ongoing - Global".to_owned(),
        Event::OngoingEachPlayer => "Ongoing - Each Player".to_owned(),
        Event::PlayerEarnedElimination => "Player Earned Elimination".to_owned(),
        Event::PlayerDealtFinalBlow => "Player Dealt Final Blow".to_owned(),
        Event::PlayerDealtDamage => "Player Dealt Damage".to_owned(),
        Event::PlayerTookDamage => "Player Took Damage".to_owned(),
        Event::PlayerDied => "Player Died".to_owned(),
        Event::PlayerDealtHealing => "Player Dealt Healing".to_owned(),
        Event::PlayerReceivedHealing => "Player Received Healing".to_owned(),
        Event::PlayerJoinedMatch => "Player Joined Match".to_owned(),
        Event::PlayerLeftMatch => "Player Left Match".to_owned(),
        Event::PlayerDealtKnockback => "Player Dealt Knockback".to_owned(),
        Event::PlayerReceivedKnockback => "Player Received Knockback".to_owned(),
        Event::Subroutine(_) => "Subroutine".to_owned(),
    }
}

/// The subroutines `block` calls or starts.
fn called(block: &Block) -> Vec<SubId> {
    let mut found = Vec::new();
    walk_block(block, &mut |stmt| match &stmt.kind {
        StmtKind::CallSubroutine(id) | StmtKind::StartRule(id, _) => found.push(*id),
        _ => {}
    });
    found
}

fn is_panel(rule: &Rule) -> bool {
    rule.location.is_none() && PANEL_RULES.contains(&rule.name.as_str())
}

/// The log of `program`, which has been compiled; `counts` are its element counts.
pub(crate) fn build(program: &Program, counts: &Counts) -> DebugLog {
    let variables = |scope, vars: &crate::ir::Vars| {
        vars.entries
            .iter()
            .enumerate()
            .map(|(i, decl)| DebugVariable {
                index: decl.index.unwrap_or(0),
                name: decl.name.clone(),
                scope,
                location: decl.location,
                label: program
                    .debug_fields
                    .iter()
                    .find(|f| f.scope == scope && f.var.0 as usize == i)
                    .map(|f| f.label),
                chased: decl.flags.chased,
                loop_var: decl.flags.for_loop,
                in_condition: decl.flags.in_condition,
                internal: decl.internal,
            })
            .collect::<Vec<_>>()
    };
    let mut all = variables(Scope::Global, &program.globals);
    all.extend(variables(Scope::Player, &program.players));

    let mut callers: Vec<Vec<String>> = vec![Vec::new(); program.subroutines.entries.len()];
    for rule in &program.rules {
        for id in called(&rule.body) {
            if let Some(names) = callers.get_mut(id.0 as usize) {
                if !names.contains(&rule.name) {
                    names.push(rule.name.clone());
                }
            }
        }
    }
    let subroutines = program
        .subroutines
        .entries
        .iter()
        .zip(callers)
        .map(|(decl, callers)| DebugSubroutine {
            index: decl.index.unwrap_or(0),
            name: decl.name.clone(),
            callers,
        })
        .collect();

    let rules: Vec<DebugRule> = program
        .rules
        .iter()
        .zip(&counts.rules)
        .map(|(rule, count)| DebugRule {
            name: rule.name.clone(),
            event: event_name(rule.event),
            elements: count.total,
            location: rule.location,
            generated: rule.location.is_none(),
        })
        .collect();
    let panels: usize = program
        .rules
        .iter()
        .zip(&counts.rules)
        .filter(|(rule, _)| is_panel(rule))
        .map(|(_, count)| count.total)
        .sum();
    DebugLog {
        variables: all,
        subroutines,
        rules,
        totals: DebugTotals {
            elements: counts.total,
            elements_without_panels: counts.total - panels,
            globals: program.globals.entries.len(),
            players: program.players.entries.len(),
            subroutines: program.subroutines.entries.len(),
        },
        warnings: Vec::new(),
    }
}

impl fmt::Display for DebugLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ow-workshop debug log")?;
        writeln!(f, "\nVariables")?;
        if self.variables.is_empty() {
            writeln!(f, "  (none)")?;
        }
        for var in &self.variables {
            let scope = match var.scope {
                Scope::Global => "global",
                Scope::Player => "player",
            };
            let mut line = format!("  {scope} {:>3}  {}", var.index, var.name);
            if let Some(label) = var.label {
                write!(line, "  label \"{label}\"")?;
            }
            if let Some(location) = var.location {
                write!(line, "  at {location}")?;
            }
            let flags: Vec<&str> = [
                (var.chased, "chased"),
                (var.loop_var, "loop variable"),
                (var.in_condition, "used in a condition"),
                (var.internal, "internal"),
            ]
            .into_iter()
            .filter_map(|(set, name)| set.then_some(name))
            .collect();
            if !flags.is_empty() {
                write!(line, "  [{}]", flags.join(", "))?;
            }
            writeln!(f, "{line}")?;
        }

        writeln!(f, "\nSubroutines")?;
        if self.subroutines.is_empty() {
            writeln!(f, "  (none)")?;
        }
        for sub in &self.subroutines {
            let callers = if sub.callers.is_empty() {
                "never called".to_owned()
            } else {
                format!("called by {}", sub.callers.join(", "))
            };
            writeln!(f, "  {:>3}  {}  {callers}", sub.index, sub.name)?;
        }

        writeln!(f, "\nRules")?;
        if self.rules.is_empty() {
            writeln!(f, "  (none)")?;
        }
        for (i, rule) in self.rules.iter().enumerate() {
            let origin = match (&rule.location, rule.generated) {
                (Some(location), _) => format!("at {location}"),
                (None, true) => "[generated]".to_owned(),
                (None, false) => String::new(),
            };
            writeln!(
                f,
                "  {i:>3}  \"{}\"  {}  {} elements  {origin}",
                rule.name, rule.event, rule.elements
            )?;
        }

        let totals = &self.totals;
        writeln!(f, "\nTotals")?;
        writeln!(
            f,
            "  elements: {} ({} without the debug panels)",
            totals.elements, totals.elements_without_panels
        )?;
        writeln!(f, "  global variables: {} of {MAX_SLOTS}", totals.globals)?;
        writeln!(f, "  player variables: {} of {MAX_SLOTS}", totals.players)?;
        writeln!(f, "  subroutines: {} of {MAX_SLOTS}", totals.subroutines)?;

        writeln!(f, "\nWarnings")?;
        if self.warnings.is_empty() {
            writeln!(f, "  (none)")?;
        }
        for warning in &self.warnings {
            writeln!(f, "  {warning}")?;
        }
        Ok(())
    }
}
