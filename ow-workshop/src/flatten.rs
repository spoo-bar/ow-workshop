//! Flattens structured blocks into the workshop's flat action list with explicit `End`s.
//!
//! Skip distances and element counts are measured on this form, so it runs after optimization.

// Consumed by the counting and printing passes in the following changes.
#![allow(dead_code)]

use crate::ir::{Block, Event, Expr, LabelId, OptOverride, Place, Rule, Stmt, StmtKind};
use crate::options::{CompileOptions, OptLevel};
use ow_workshop_data::ActionId;

/// The optimizer settings in effect for a line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct OptState {
    /// Whether the optimizer runs.
    pub(crate) enabled: bool,
    /// Whether it optimizes for size.
    pub(crate) size: bool,
    /// Whether it skips rewrites that change behaviour under type coercion.
    pub(crate) strict: bool,
}

impl OptState {
    /// What a mod gets without any option set; the emitter only prints deviations from it.
    pub(crate) const DEFAULT: Self = Self {
        enabled: true,
        size: false,
        strict: false,
    };

    /// The state the compile options start the program in.
    pub(crate) fn from_options(options: &CompileOptions) -> Self {
        Self {
            enabled: options.optimize != OptLevel::Off,
            size: options.optimize == OptLevel::Size,
            strict: options.strict,
        }
    }

    /// The state inside a block that may override it.
    pub(crate) fn enter(self, over: Option<OptOverride>) -> Self {
        match over {
            None => self,
            Some(o) => Self {
                enabled: o.enabled,
                size: o.enabled && o.size,
                strict: o.strict.unwrap_or(self.strict),
            },
        }
    }
}

/// One line of a flattened rule body.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FlatStmt<'a> {
    /// Indentation depth, relative to the rule's action list.
    pub(crate) depth: usize,
    /// What the line is.
    pub(crate) kind: FlatKind<'a>,
    /// Printed as a string literal on the line above.
    pub(crate) comment: Option<&'a str>,
    /// The optimizer settings in effect on the line.
    pub(crate) opt: OptState,
}

/// The shape of a flattened line.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum FlatKind<'a> {
    /// A statement that has no nested block.
    Stmt(&'a Stmt),
    /// `If(c);`
    If(&'a Expr),
    /// `Else If(c);`
    ElseIf(&'a Expr),
    /// `Else;`
    Else,
    /// `While(c);`
    While(&'a Expr),
    /// `For Global/Player Variable(...)`.
    For {
        /// The loop variable.
        var: &'a Place,
        /// First value.
        start: &'a Expr,
        /// Exclusive end.
        stop: &'a Expr,
        /// Increment.
        step: &'a Expr,
    },
    /// `End;`
    End,
    /// A jump target; occupies no action slot.
    Label(LabelId),
    /// A standalone comment; occupies no action slot.
    Comment(&'a str),
    /// A `Pass` that survived optimization, printed as `disabled Abort;`.
    DisabledAbort,
}

impl FlatKind<'_> {
    /// Whether the line is an action of its own in the game (counts and takes a skip slot).
    pub(crate) fn is_action(&self) -> bool {
        !matches!(self, Self::Label(_) | Self::Comment(_))
    }
}

struct Flattener<'a> {
    out: Vec<FlatStmt<'a>>,
    subroutine: bool,
}

/// Flattens the body of `rule`.
pub(crate) fn flatten_rule<'a>(rule: &'a Rule, options: &CompileOptions) -> Vec<FlatStmt<'a>> {
    let mut f = Flattener {
        out: Vec::new(),
        subroutine: matches!(rule.event, Event::Subroutine(_)),
    };
    f.block(
        &rule.body,
        0,
        true,
        OptState::from_options(options).enter(rule.body.opt),
    );
    f.out
}

impl<'a> Flattener<'a> {
    fn push(&mut self, depth: usize, kind: FlatKind<'a>, comment: Option<&'a str>, opt: OptState) {
        self.out.push(FlatStmt {
            depth,
            kind,
            comment,
            opt,
        });
    }

    /// `tail`: nothing runs after this block ends but the end of the rule.
    fn block(&mut self, block: &'a Block, depth: usize, tail: bool, opt: OptState) {
        // A lowered switch opens an `If(True)` that its own `End` action closes.
        let mut open = 0;
        let last = block.stmts.len().saturating_sub(1);
        for (i, stmt) in block.stmts.iter().enumerate() {
            let comment = stmt.comment.as_deref();
            let at = depth + open;
            match &stmt.kind {
                StmtKind::Label(id) => self.push(at, FlatKind::Label(*id), None, opt),
                StmtKind::Comment(text) => self.push(at, FlatKind::Comment(text), None, opt),
                StmtKind::Pass => {
                    self.push(at, FlatKind::DisabledAbort, comment.or(Some("pass")), opt);
                }
                StmtKind::SwitchIf => {
                    self.push(at, FlatKind::Stmt(stmt), comment, opt);
                    open += 1;
                }
                StmtKind::Action {
                    id: ActionId::Else, ..
                } if open > 0 => {
                    self.push(at - 1, FlatKind::Stmt(stmt), comment, opt);
                }
                StmtKind::Action {
                    id: ActionId::End, ..
                } if open > 0 => {
                    open -= 1;
                    self.push(at - 1, FlatKind::Stmt(stmt), comment, opt);
                }
                StmtKind::If {
                    cond,
                    then,
                    elifs,
                    else_,
                } => {
                    let is_last = i == last;
                    let omit = is_last && tail && !self.subroutine && opt.enabled;
                    self.push(at, FlatKind::If(cond), comment, opt);
                    let mut branches = elifs.len() + usize::from(else_.is_some());
                    // Only the last branch can leave its nested `End`s off: an `Else` that
                    // follows would otherwise bind to the inner `If`.
                    let last_branch = |remaining: usize| remaining == 0;
                    self.block(
                        then,
                        at + 1,
                        is_last && tail && last_branch(branches),
                        opt.enter(then.opt),
                    );
                    for (cond, body) in elifs {
                        branches -= 1;
                        self.push(at, FlatKind::ElseIf(cond), None, opt);
                        self.block(
                            body,
                            at + 1,
                            is_last && tail && last_branch(branches),
                            opt.enter(body.opt),
                        );
                    }
                    if let Some(body) = else_ {
                        self.push(at, FlatKind::Else, None, opt);
                        self.block(body, at + 1, is_last && tail, opt.enter(body.opt));
                    }
                    if !omit {
                        self.push(at, FlatKind::End, None, opt);
                    }
                }
                StmtKind::While { cond, body } => {
                    self.push(at, FlatKind::While(cond), comment, opt);
                    self.block(body, at + 1, false, opt.enter(body.opt));
                    self.push(at, FlatKind::End, None, opt);
                }
                StmtKind::For {
                    var,
                    start,
                    stop,
                    step,
                    body,
                } => {
                    let kind = FlatKind::For {
                        var,
                        start,
                        stop,
                        step,
                    };
                    self.push(at, kind, comment, opt);
                    self.block(body, at + 1, false, opt.enter(body.opt));
                    self.push(at, FlatKind::End, None, opt);
                }
                _ => self.push(at, FlatKind::Stmt(stmt), comment, opt),
            }
        }
    }
}

#[cfg(test)]
mod tests;
