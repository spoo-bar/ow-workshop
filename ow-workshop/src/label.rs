//! Labels and skips (`goto` forward within a rule).

use crate::block::Block;
use crate::ir::{LabelId, SkipTarget, StmtKind};
use crate::ty::{Bool, IntoExpr, Num};

/// A jump target; place it with [`Block::place`] after the skips that use it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Label(LabelId);

impl Block {
    /// Declares a label.
    #[must_use]
    pub fn label(&self) -> Label {
        Label(LabelId(self.next_label()))
    }

    /// Marks the label's position; do this at most once, in the rule that uses it.
    #[track_caller]
    pub fn place(&mut self, label: Label) {
        self.push(StmtKind::Label(label.0));
    }

    /// `Skip` to a label; the compiler counts the actions in between.
    #[track_caller]
    pub fn skip_to(&mut self, label: Label) {
        self.push(StmtKind::Skip(SkipTarget::Label(label.0)));
    }

    /// `Skip If` to a label.
    #[track_caller]
    pub fn skip_to_if(&mut self, cond: impl IntoExpr<Bool>, label: Label) {
        self.push(StmtKind::SkipIf(
            cond.into_expr().into_ir(),
            SkipTarget::Label(label.0),
        ));
    }

    /// `Skip(n)` over a computed number of actions; this disables most control-flow optimization
    /// for the rule.
    #[track_caller]
    pub fn skip(&mut self, count: impl IntoExpr<Num>) {
        self.push(StmtKind::Skip(SkipTarget::Dynamic(
            count.into_expr().into_ir(),
        )));
    }

    /// `Skip If(cond, n)` over a computed number of actions.
    #[track_caller]
    pub fn skip_if(&mut self, cond: impl IntoExpr<Bool>, count: impl IntoExpr<Num>) {
        self.push(StmtKind::SkipIf(
            cond.into_expr().into_ir(),
            SkipTarget::Dynamic(count.into_expr().into_ir()),
        ));
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::Builder;
    use crate::check::check_labels;
    use crate::event::OngoingGlobal;
    use crate::ir::{Diagnostics, ExprKind, Program, StmtKind};

    fn rules(build: impl FnOnce(&mut Builder)) -> Program {
        let mut ws = Builder::new();
        build(&mut ws);
        ws.finish(Program::default()).0
    }

    fn codes(program: &Program) -> Vec<&'static str> {
        let mut diags = Diagnostics::default();
        check_labels(&program.rules, &mut diags);
        assert!(diags.0.iter().all(|d| d.location.is_some()));
        diags.0.iter().map(crate::ir::Diagnostic::code).collect()
    }

    #[test]
    fn skip_forward_to_a_label() {
        let program = rules(|ws| {
            ws.rule("r", OngoingGlobal, |r, _| {
                r.actions(|b| {
                    let l = b.label();
                    b.skip_to_if(true, l);
                    b.skip_to(l);
                    b.pass();
                    b.place(l);
                });
            });
        });
        let stmts = &program.rules[0].body.stmts;
        assert!(matches!(
            stmts[0].kind,
            StmtKind::SkipIf(_, crate::ir::SkipTarget::Label(_))
        ));
        assert!(matches!(
            stmts[1].kind,
            StmtKind::Skip(crate::ir::SkipTarget::Label(_))
        ));
        assert!(matches!(stmts[3].kind, StmtKind::Label(_)));
        assert_eq!(codes(&program), [] as [&str; 0]);
    }

    #[test]
    fn labels_are_unique_across_rules_and_blocks() {
        let program = rules(|ws| {
            for name in ["a", "b"] {
                ws.rule(name, OngoingGlobal, |r, _| {
                    r.actions(|b| {
                        let outer = b.label();
                        b.if_(true, |b| {
                            let inner = b.label();
                            assert_ne!(outer, inner);
                        });
                        let next = b.label();
                        assert_ne!(outer, next);
                    });
                });
            }
        });
        assert_eq!(program.rules.len(), 2);
        let mut ws = Builder::new();
        let mut seen = Vec::new();
        for _ in 0..2 {
            ws.rule("x", OngoingGlobal, |r, _| {
                r.actions(|b| seen.push(b.label()));
            });
        }
        assert_ne!(seen[0], seen[1]);
    }

    #[test]
    fn dynamic_skips() {
        let program = rules(|ws| {
            ws.rule("r", OngoingGlobal, |r, _| {
                r.actions(|b| {
                    b.skip(2);
                    b.skip_if(true, 1);
                });
            });
        });
        let StmtKind::Skip(crate::ir::SkipTarget::Dynamic(n)) =
            &program.rules[0].body.stmts[0].kind
        else {
            panic!("not a dynamic skip");
        };
        assert_eq!(n.kind(), &ExprKind::Num(2.0));
        assert_eq!(codes(&program), [] as [&str; 0]);
    }

    #[test]
    fn placed_twice() {
        let program = rules(|ws| {
            ws.rule("r", OngoingGlobal, |r, _| {
                r.actions(|b| {
                    let l = b.label();
                    b.skip_to(l);
                    b.place(l);
                    b.place(l);
                });
            });
        });
        assert_eq!(codes(&program), ["label-placed-twice"]);
    }

    #[test]
    fn never_placed() {
        let program = rules(|ws| {
            ws.rule("r", OngoingGlobal, |r, _| {
                r.actions(|b| {
                    let l = b.label();
                    b.skip_to(l);
                });
            });
        });
        assert_eq!(codes(&program), ["label-never-placed"]);
    }

    #[test]
    fn used_in_another_rule() {
        let mut shared = None;
        let program = rules(|ws| {
            ws.rule("a", OngoingGlobal, |r, _| {
                r.actions(|b| {
                    let l = b.label();
                    b.place(l);
                    shared = Some(l);
                });
            });
            ws.rule("b", OngoingGlobal, |r, _| {
                r.actions(|b| {
                    if let Some(l) = shared {
                        b.skip_to(l);
                    }
                });
            });
        });
        assert_eq!(codes(&program), ["label-in-other-rule"]);
    }

    #[test]
    fn placed_before_the_skip() {
        let program = rules(|ws| {
            ws.rule("r", OngoingGlobal, |r, _| {
                r.actions(|b| {
                    let l = b.label();
                    b.place(l);
                    b.skip_to(l);
                });
            });
        });
        assert_eq!(codes(&program), ["label-before-skip"]);
    }
}
