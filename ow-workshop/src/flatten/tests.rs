use ow_workshop_data::ActionId;

use super::{FlatKind, FlatStmt, flatten_rule};
use crate::ir::{Block, Event, Rule, StmtKind, SubId};
use crate::lower::testutil::{action, block, boolean, if_, num, options, program};
use crate::options::OptLevel;

/// `depth:kind` for each line, e.g. `1:If`.
fn shape(lines: &[FlatStmt<'_>]) -> Vec<String> {
    lines
        .iter()
        .map(|l| {
            let name = match &l.kind {
                FlatKind::Stmt(_) => "Stmt",
                FlatKind::If(_) => "If",
                FlatKind::ElseIf(_) => "ElseIf",
                FlatKind::Else => "Else",
                FlatKind::While(_) => "While",
                FlatKind::For { .. } => "For",
                FlatKind::End => "End",
                FlatKind::Label(_) => "Label",
                FlatKind::Comment(_) => "Comment",
                FlatKind::DisabledAbort => "DisabledAbort",
            };
            format!("{}:{name}", l.depth)
        })
        .collect()
}

fn run(body: Vec<StmtKind>, level: OptLevel) -> Vec<String> {
    let program = program(body);
    shape(&flatten_rule(&program.rules[0], &options(level)))
}

#[test]
fn nested_ifs_inside_a_while_keep_their_end() {
    let inner = if_(boolean(true), vec![action()]);
    let body = vec![StmtKind::While {
        cond: boolean(true),
        body: block(vec![inner]),
    }];
    assert_eq!(
        run(body, OptLevel::Speed),
        ["0:While", "1:If", "2:Stmt", "1:End", "0:End"]
    );
}

#[test]
fn trailing_if_drops_its_end() {
    let body = vec![action(), if_(boolean(true), vec![action()])];
    assert_eq!(run(body, OptLevel::Speed), ["0:Stmt", "0:If", "1:Stmt"]);
}

#[test]
fn if_with_following_statement_keeps_its_end() {
    let body = vec![if_(boolean(true), vec![action()]), action()];
    assert_eq!(
        run(body, OptLevel::Speed),
        ["0:If", "1:Stmt", "0:End", "0:Stmt"]
    );
}

#[test]
fn optimization_off_keeps_the_end() {
    let body = vec![if_(boolean(true), vec![action()])];
    assert_eq!(run(body, OptLevel::Off), ["0:If", "1:Stmt", "0:End"]);
}

#[test]
fn subroutine_keeps_the_end() {
    let mut program = program(vec![if_(boolean(true), vec![action()])]);
    program.rules[0].event = Event::Subroutine(SubId(0));
    let lines = flatten_rule(&program.rules[0], &options(OptLevel::Speed));
    assert_eq!(shape(&lines), ["0:If", "1:Stmt", "0:End"]);
}

#[test]
fn nested_trailing_ifs_all_drop_their_end() {
    let inner = if_(boolean(true), vec![action()]);
    let body = vec![if_(boolean(true), vec![inner])];
    assert_eq!(run(body, OptLevel::Speed), ["0:If", "1:If", "2:Stmt"]);
}

#[test]
fn inner_if_keeps_its_end_when_an_else_follows_it() {
    let inner = if_(boolean(true), vec![action()]);
    let outer = StmtKind::If {
        cond: boolean(true),
        then: block(vec![inner]),
        elifs: vec![],
        else_: Some(block(vec![action()])),
    };
    assert_eq!(
        run(vec![outer], OptLevel::Speed),
        ["0:If", "1:If", "2:Stmt", "1:End", "0:Else", "1:Stmt"]
    );
}

#[test]
fn elifs_and_else_sit_at_the_if_depth() {
    let outer = StmtKind::If {
        cond: boolean(true),
        then: block(vec![action()]),
        elifs: vec![(boolean(false), block(vec![action()]))],
        else_: Some(block(vec![action()])),
    };
    assert_eq!(
        run(vec![outer, action()], OptLevel::Speed),
        [
            "0:If", "1:Stmt", "0:ElseIf", "1:Stmt", "0:Else", "1:Stmt", "0:End", "0:Stmt"
        ]
    );
}

#[test]
fn block_override_disabling_optimization_keeps_the_end() {
    let mut rule_body = block(vec![if_(boolean(true), vec![action()])]);
    rule_body.opt = Some(crate::ir::OptOverride {
        enabled: false,
        size: false,
        strict: None,
    });
    let rule = Rule::new("r", Event::OngoingGlobal, rule_body);
    let lines = flatten_rule(&rule, &options(OptLevel::Speed));
    assert_eq!(shape(&lines), ["0:If", "1:Stmt", "0:End"]);
}

#[test]
fn labels_and_comments_are_zero_width_markers() {
    let body = vec![
        StmtKind::Label(crate::ir::LabelId(0)),
        StmtKind::Comment("hi".to_owned()),
        action(),
    ];
    let program = program(body);
    let lines = flatten_rule(&program.rules[0], &options(OptLevel::Speed));
    assert_eq!(shape(&lines), ["0:Label", "0:Comment", "0:Stmt"]);
    assert!(!lines[0].kind.is_action() && !lines[1].kind.is_action());
    assert!(lines[2].kind.is_action());
}

#[test]
fn surviving_pass_becomes_a_commented_disabled_abort() {
    let mut program = program(vec![StmtKind::Pass, StmtKind::Pass]);
    program.rules[0].body.stmts[1].comment = Some("mine".to_owned());
    let lines = flatten_rule(&program.rules[0], &options(OptLevel::Speed));
    assert_eq!(lines[0].kind, FlatKind::DisabledAbort);
    assert_eq!(lines[0].comment, Some("pass"));
    assert_eq!(lines[1].comment, Some("mine"));
}

#[test]
fn lowered_switch_indents_its_body_until_the_end_action() {
    let end = |id| StmtKind::Action { id, args: vec![] };
    let body = vec![
        StmtKind::SwitchIf,
        StmtKind::Skip(crate::ir::SkipTarget::Dynamic(num(1.0))),
        action(),
        end(ActionId::Else),
        action(),
        end(ActionId::End),
    ];
    assert_eq!(
        run(body, OptLevel::Speed),
        ["0:Stmt", "1:Stmt", "1:Stmt", "0:Stmt", "1:Stmt", "0:Stmt"]
    );
}

#[test]
fn empty_block_is_fine() {
    let rule = Rule::new("r", Event::OngoingGlobal, Block::default());
    assert_eq!(flatten_rule(&rule, &options(OptLevel::Speed)).len(), 0);
}
