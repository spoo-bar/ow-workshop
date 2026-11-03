//! Hand-built rules and programs.

use ow_workshop_data::{ActionId, Hero};

use super::*;

fn action(id: ActionId) -> Stmt {
    Stmt::new(StmtKind::Action { id, args: vec![] })
}

fn block(stmts: Vec<Stmt>) -> Block {
    Block {
        stmts,
        ..Block::default()
    }
}

/// A player-event rule, a subroutine rule, and a nested block override.
fn sample() -> Program {
    let mut program = Program::default();
    let sub = program.subroutines.declare(SubDecl::new("heal"));

    let nested = Block {
        stmts: vec![action(ActionId::Wait)],
        opt: Some(OptOverride {
            enabled: false,
            size: false,
            strict: Some(true),
        }),
    };
    let outer = Block {
        stmts: vec![Stmt::new(StmtKind::While {
            cond: Expr::new(ExprKind::Bool(true), None),
            body: nested,
        })],
        opt: Some(OptOverride {
            enabled: true,
            size: true,
            strict: None,
        }),
    };

    let mut died = Rule::new("Score", Event::PlayerDied, outer);
    died.filter = Some(PlayerFilter {
        team: TeamFilter::Team1,
        player: PlayerSlot::Hero(Hero::Ana),
    });
    died.conditions.push(Cond {
        expr: Expr::new(ExprKind::Bool(true), None),
        comment: Some("always".into()),
    });
    died.new_page = Some("filler".into());

    let mut helper = Rule::new(
        "heal",
        Event::Subroutine(sub),
        block(vec![action(ActionId::Wait)]),
    );
    helper.delimiter = true;

    program.rules = vec![died, helper];
    program
}

#[test]
fn hand_built_program_is_valid() {
    let program = sample();
    assert_eq!(program.validate(), Ok(()));
    assert_eq!(program, program.clone());
    let overridden = &program.rules[0].body;
    assert_eq!(overridden.opt.map(|o| o.size), Some(true));
}

#[test]
fn filter_on_a_global_rule_is_rejected() {
    let mut rule = Rule::new("Init", Event::OngoingGlobal, Block::default());
    rule.filter = Some(PlayerFilter::default());
    assert_eq!(rule.validate(), Err(RuleError::FilterNotAllowed));

    let mut program = Program::default();
    program.rules.push(rule);
    let diags = program.validate().unwrap_err();
    assert_eq!(diags.0[0].code, "filter-not-allowed");
}

#[test]
fn filter_on_a_subroutine_rule_is_rejected() {
    let mut rule = Rule::new("s", Event::Subroutine(SubId(0)), Block::default());
    rule.filter = Some(PlayerFilter::default());
    assert_eq!(rule.validate(), Err(RuleError::FilterNotAllowed));
}

#[test]
fn each_player_rules_take_a_filter() {
    let mut rule = Rule::new("p", Event::OngoingEachPlayer, Block::default());
    rule.filter = Some(PlayerFilter {
        team: TeamFilter::Team2,
        player: PlayerSlot::Slot(11),
    });
    assert_eq!(rule.validate(), Ok(()));
    rule.filter = Some(PlayerFilter {
        team: TeamFilter::All,
        player: PlayerSlot::Slot(12),
    });
    assert_eq!(rule.validate(), Err(RuleError::SlotOutOfRange(12)));
}

#[test]
fn subroutine_rules_have_no_conditions() {
    let mut rule = Rule::new("s", Event::Subroutine(SubId(0)), Block::default());
    rule.conditions.push(Cond {
        expr: Expr::new(ExprKind::Bool(true), None),
        comment: None,
    });
    assert_eq!(rule.validate(), Err(RuleError::SubroutineHasConditions));
}
