//! Prints a [`Script`] as workshop text.

use std::fmt::Write as _;

use crate::script::{RuleData, Script, Trigger};
use crate::settings::{Entry, quoted};
use crate::{Action, Chase, Event, PlayerFilter, TeamFilter, Value};

pub(crate) fn write(script: &Script) -> String {
    let mut out = String::new();
    settings(&mut out, script);
    variables(&mut out, script);
    if !script.subroutines.is_empty() {
        out.push_str("subroutines {\n");
        for (index, name) in script.subroutines.iter().enumerate() {
            let _ = writeln!(out, "    {index}: {name}");
        }
        out.push_str("}\n");
    }
    for data in &script.rules {
        rule(&mut out, data);
    }
    out
}

struct Tabbed<'a> {
    out: &'a mut String,
    depth: usize,
}

impl Tabbed<'_> {
    fn line(&mut self, text: &str) {
        let _ = writeln!(self.out, "{}{text}", "\t".repeat(self.depth));
    }

    fn block(&mut self, title: &str, body: impl FnOnce(&mut Self)) {
        self.line(title);
        self.line("{");
        self.depth += 1;
        body(self);
        self.depth -= 1;
        self.line("}");
    }

    fn entries(&mut self, entries: &[Entry]) {
        for Entry { key, value, .. } in entries {
            self.line(&format!("{key}: {value}"));
        }
    }

    fn list(&mut self, title: &str, items: &[String]) {
        if !items.is_empty() {
            self.block(title, |t| items.iter().for_each(|item| t.line(item)));
        }
    }
}

fn settings(out: &mut String, script: &Script) {
    let resolved = &script.settings;
    let mut tabbed = Tabbed { out, depth: 0 };
    tabbed.block("settings", |t| {
        t.block("main", |t| t.entries(&resolved.main));
        if !resolved.lobby.is_empty() {
            t.block("lobby", |t| t.entries(&resolved.lobby));
        }
        if !resolved.modes.is_empty() {
            t.block("modes", |t| {
                for mode in &resolved.modes {
                    let prefix = if mode.enabled { "" } else { "disabled " };
                    t.block(&format!("{prefix}{}", mode.name), |t| {
                        t.entries(&mode.entries);
                        t.list("enabled maps", &mode.enabled_maps);
                        t.list("disabled maps", &mode.disabled_maps);
                    });
                }
            });
        }
        if !resolved.heroes.is_empty() {
            t.block("heroes", |t| {
                for scope in &resolved.heroes {
                    t.block(&scope.scope, |t| {
                        t.entries(&scope.entries);
                        for (hero, entries) in &scope.heroes {
                            t.block(hero, |t| t.entries(entries));
                        }
                        t.list("enabled heroes", &scope.enabled_heroes);
                        t.list("disabled heroes", &scope.disabled_heroes);
                    });
                }
            });
        }
        if !script.extensions.is_empty() {
            t.block("extensions", |t| {
                for extension in &script.extensions {
                    t.line(extension.name());
                }
            });
        }
    });
}

fn variables(out: &mut String, script: &Script) {
    if script.globals.is_empty() && script.players.is_empty() {
        return;
    }
    out.push_str("variables {\n");
    let mut section = |title: &str, names: Vec<(u8, &str)>| {
        if names.is_empty() {
            return;
        }
        let _ = writeln!(out, "    {title}:");
        for (index, name) in names {
            let _ = writeln!(out, "        {index}: {name}");
        }
    };
    section(
        "global",
        script
            .globals
            .iter()
            .map(|var| (var.index, var.name))
            .collect(),
    );
    section(
        "player",
        script
            .players
            .iter()
            .map(|var| (var.index, var.name))
            .collect(),
    );
    out.push_str("}\n");
}

fn rule(out: &mut String, data: &RuleData) {
    let prefix = if data.enabled { "" } else { "disabled " };
    let _ = writeln!(out, "{prefix}rule ({}) {{", quoted(&data.name));
    out.push_str("    event {\n");
    for line in trigger_lines(&data.trigger) {
        let _ = writeln!(out, "        {line};");
    }
    out.push_str("    }\n");
    if !data.conditions.is_empty() {
        out.push_str("    conditions {\n");
        for condition in &data.conditions {
            let (lhs, rhs) = (value(&condition.lhs), value(&condition.rhs));
            let _ = writeln!(out, "        {lhs} {} {rhs};", condition.op.name());
        }
        out.push_str("    }\n");
    }
    out.push_str("    actions {\n");
    actions(out, &data.actions, 2);
    out.push_str("    }\n}\n\n");
}

fn trigger_lines(trigger: &Trigger) -> Vec<String> {
    match trigger {
        Trigger::Subroutine(name) => vec!["Subroutine".to_owned(), name.clone()],
        Trigger::Event(Event::OngoingGlobal) => vec!["Ongoing - Global".to_owned()],
        Trigger::Event(Event::Player { kind, team, player }) => {
            let team = match team {
                TeamFilter::All => "All",
                TeamFilter::Team1 => "Team 1",
                TeamFilter::Team2 => "Team 2",
            };
            let player = match player {
                PlayerFilter::All => "All".to_owned(),
                PlayerFilter::Slot(slot) => format!("Slot {slot}"),
                PlayerFilter::Hero(hero) => hero.name().to_owned(),
            };
            vec![kind.name().to_owned(), team.to_owned(), player]
        }
    }
}

#[allow(clippy::too_many_lines)]
fn actions(out: &mut String, list: &[Action], depth: usize) {
    let pad = "    ".repeat(depth);
    for action in list {
        match action {
            Action::Call { id, args } => {
                let _ = writeln!(out, "{pad}{};", call(id.name(), args));
            }
            Action::SetGlobal { var, value: v } => {
                let _ = writeln!(out, "{pad}Set Global Variable({}, {});", var.name, value(v));
            }
            Action::SetPlayerVar {
                player,
                var,
                value: v,
            } => {
                let (player, v) = (value(player), value(v));
                let _ = writeln!(
                    out,
                    "{pad}Set Player Variable({player}, {}, {v});",
                    var.name
                );
            }
            Action::SetGlobalAt {
                var,
                index,
                value: v,
            } => {
                let (index, v) = (value(index), value(v));
                let _ = writeln!(
                    out,
                    "{pad}Set Global Variable At Index({}, {index}, {v});",
                    var.name
                );
            }
            Action::SetPlayerVarAt {
                player,
                var,
                index,
                value: v,
            } => {
                let (player, index, v) = (value(player), value(index), value(v));
                let _ = writeln!(
                    out,
                    "{pad}Set Player Variable At Index({player}, {}, {index}, {v});",
                    var.name
                );
            }
            Action::ModifyGlobal {
                var,
                index,
                op,
                value: v,
            } => {
                let (op, v) = (op.name(), value(v));
                let _ = match index {
                    None => writeln!(out, "{pad}Modify Global Variable({}, {op}, {v});", var.name),
                    Some(index) => writeln!(
                        out,
                        "{pad}Modify Global Variable At Index({}, {}, {op}, {v});",
                        var.name,
                        value(index)
                    ),
                };
            }
            Action::ModifyPlayerVar {
                player,
                var,
                index,
                op,
                value: v,
            } => {
                let (player, op, v) = (value(player), op.name(), value(v));
                let _ = match index {
                    None => writeln!(
                        out,
                        "{pad}Modify Player Variable({player}, {}, {op}, {v});",
                        var.name
                    ),
                    Some(index) => writeln!(
                        out,
                        "{pad}Modify Player Variable At Index({player}, {}, {}, {op}, {v});",
                        var.name,
                        value(index)
                    ),
                };
            }
            Action::ChaseGlobal {
                var,
                destination,
                mode,
            } => {
                let (kind, tail) = chase(mode);
                let _ = writeln!(
                    out,
                    "{pad}Chase Global Variable {kind}({}, {}, {tail});",
                    var.name,
                    value(destination)
                );
            }
            Action::ChasePlayerVar {
                player,
                var,
                destination,
                mode,
            } => {
                let (kind, tail) = chase(mode);
                let _ = writeln!(
                    out,
                    "{pad}Chase Player Variable {kind}({}, {}, {}, {tail});",
                    value(player),
                    var.name,
                    value(destination)
                );
            }
            Action::StopChasingGlobal(var) => {
                let _ = writeln!(out, "{pad}Stop Chasing Global Variable({});", var.name);
            }
            Action::StopChasingPlayerVar { player, var } => {
                let _ = writeln!(
                    out,
                    "{pad}Stop Chasing Player Variable({}, {});",
                    value(player),
                    var.name
                );
            }
            Action::If {
                cond,
                then,
                otherwise,
            } => {
                let _ = writeln!(out, "{pad}If({});", value(cond));
                actions(out, then, depth + 1);
                if !otherwise.is_empty() {
                    let _ = writeln!(out, "{pad}Else;");
                    actions(out, otherwise, depth + 1);
                }
                let _ = writeln!(out, "{pad}End;");
            }
            Action::CallSubroutine(name) => {
                let _ = writeln!(out, "{pad}Call Subroutine({name});");
            }
            Action::Commented { comment, action } => {
                let _ = writeln!(out, "{pad}{}", quoted(comment));
                actions(out, std::slice::from_ref(action), depth);
            }
        }
    }
}

fn value(v: &Value) -> String {
    match v {
        Value::Number(n) => n.to_string(),
        Value::Bool(true) => "True".to_owned(),
        Value::Bool(false) => "False".to_owned(),
        Value::Call { id, args } => call(id.name(), args),
        Value::Constant(name) => (*name).to_owned(),
        Value::CustomString { text, args } => {
            let mut parts = vec![quoted(text)];
            parts.extend(args.iter().map(value));
            format!("Custom String({})", parts.join(", "))
        }
        Value::Global(var) => format!("Global.{}", var.name),
        Value::PlayerVar { player, var } => format!("({}).{}", value(player), var.name),
        Value::Compare { lhs, op, rhs } => {
            format!("Compare({}, {}, {})", value(lhs), op.name(), value(rhs))
        }
    }
}

fn call(name: &str, args: &[Value]) -> String {
    if args.is_empty() {
        return name.to_owned();
    }
    let args: Vec<String> = args.iter().map(value).collect();
    format!("{name}({})", args.join(", "))
}

/// The action-name suffix and the trailing arguments of a chase.
fn chase(mode: &Chase) -> (&'static str, String) {
    match mode {
        Chase::AtRate { rate, reevaluation } => (
            "At Rate",
            format!("{}, {}", value(rate), reevaluation.name()),
        ),
        Chase::OverTime {
            duration,
            reevaluation,
        } => (
            "Over Time",
            format!("{}, {}", value(duration), reevaluation.name()),
        ),
    }
}
