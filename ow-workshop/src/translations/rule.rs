//! The generated rule that detects each player's language in player-variable mode.
//!
//! `Start Facing` is reevaluated on every client, so making each client face a direction that
//! encodes its language index and reading the angle back on the server stores the language once.

use ow_workshop_data::{ActionId, ValueId};

use super::detect::{Detection, LANGUAGE_INDEX, choose, helper_init};
use super::{PlayerVarMode, TranslationLang};
use crate::ir::{
    Block, CmpOp, Cond, Event, Expr, ExprKind, ModifyOp, Place, PlayerFilter, Program, Rule, Stmt,
    StmtKind, VarId,
};

/// Name of the generated rule.
pub(crate) const RULE_NAME: &str = "Translation setup - detect player language";

fn ex(kind: ExprKind) -> Expr {
    Expr::new(kind, None)
}

fn num(n: f64) -> Expr {
    ex(ExprKind::Num(n))
}

fn call(id: ValueId, args: Vec<Expr>) -> Expr {
    ex(ExprKind::Value { id, args })
}

fn konst(enum_name: &'static str, key: &'static str) -> Expr {
    ex(ExprKind::Const(crate::ir::ConstRef { enum_name, key }))
}

fn event_player() -> Expr {
    call(ValueId::EventPlayer, vec![])
}

fn compare(op: CmpOp, lhs: Expr, rhs: Expr) -> Expr {
    ex(ExprKind::Compare { op, lhs, rhs })
}

fn index_var(var: VarId) -> Expr {
    ex(ExprKind::PlayerVar {
        player: event_player(),
        var,
    })
}

fn action(id: ActionId, args: Vec<Expr>) -> Stmt {
    Stmt {
        kind: StmtKind::Action { id, args },
        loc: None,
        comment: None,
        suppress: crate::ir::WarningSet::default(),
    }
}

fn stmt(kind: StmtKind) -> Stmt {
    Stmt {
        kind,
        loc: None,
        comment: None,
        suppress: crate::ir::WarningSet::default(),
    }
}

/// `Round To Integer(value, To Nearest)`.
fn round(value: Expr) -> Expr {
    call(
        ValueId::Round,
        vec![value, konst("Rounding", "__roundToNearest__")],
    )
}

/// The actions of the rule.
fn actions(
    var: VarId,
    langs: &[TranslationLang],
    detection: Detection,
    mode: PlayerVarMode,
) -> Vec<Stmt> {
    let place = || Place::Player {
        player: event_player(),
        var,
    };
    let vertical_off = || {
        call(
            ValueId::Abs,
            vec![ex(ExprKind::Sub(
                call(ValueId::GetVerticalFacingAngle, vec![event_player()]),
                num(5.0),
            ))],
        )
    };
    let aligned = || compare(CmpOp::Lt, vertical_off(), num(0.01));
    let horizontal = || call(ValueId::GetHorizontalFacingAngle, vec![event_player()]);

    // The language's position among the names, as a horizontal angle: 10 degrees each.
    let position = call(
        ValueId::Index,
        vec![
            helper_init(langs, detection),
            call(
                ValueId::Split,
                vec![detection.expr(), ex(ExprKind::Array(vec![]))],
            ),
        ],
    );
    let direction = call(
        ValueId::DirectionFromAngles,
        vec![ex(ExprKind::Mul(num(10.0), position)), num(5.0)],
    );
    let settled = ex(ExprKind::And(
        ex(ExprKind::Not(ex(ExprKind::Mod(
            round(ex(ExprKind::Mul(horizontal(), num(100.0)))),
            num(1000.0),
        )))),
        aligned(),
    ));
    let angle_index = call(
        ValueId::Max,
        vec![
            num(1.0),
            ex(ExprKind::Mul(
                aligned(),
                round(ex(ExprKind::Div(horizontal(), num(10.0)))),
            )),
        ],
    );
    let stored = ex(ExprKind::Index {
        array: index_var(var),
        index: num(0.0),
    });
    let final_index = match mode {
        PlayerVarMode::Default => stored,
        PlayerVarMode::NoTlErr => ex(ExprKind::Sub(stored, num(1.0))),
    };

    vec![
        stmt(StmtKind::Modify {
            target: place(),
            op: ModifyOp::AppendToArray,
            value: call(ValueId::GetFacingDirection, vec![event_player()]),
        }),
        action(
            ActionId::StartFacing,
            vec![
                event_player(),
                direction,
                num(999_999_999_999.0),
                konst("Relativity", "TO_WORLD"),
                konst("FacingReeval", "DIRECTION_AND_TURN_RATE"),
            ],
        ),
        action(ActionId::WaitUntil, vec![settled, num(15.0)]),
        stmt(StmtKind::Set {
            target: Place::Index {
                base: Box::new(place()),
                index: num(0.0),
            },
            value: angle_index,
        }),
        action(ActionId::StopFacing, vec![event_player()]),
        action(
            ActionId::SetFacing,
            vec![
                event_player(),
                call(ValueId::Last, vec![index_var(var)]),
                konst("Relativity", "TO_WORLD"),
            ],
        ),
        stmt(StmtKind::Set {
            target: place(),
            value: final_index,
        }),
    ]
}

/// The rule, for the language index variable `var`.
pub(crate) fn detect_language(
    var: VarId,
    langs: &[TranslationLang],
    detection: Detection,
    mode: PlayerVarMode,
) -> Rule {
    let start = if mode == PlayerVarMode::NoTlErr {
        0.1
    } else {
        1.1
    };
    let stmts = actions(var, langs, detection, mode);
    let cond = |expr| Cond {
        expr,
        comment: None,
    };
    Rule {
        filter: Some(PlayerFilter::default()),
        conditions: vec![
            cond(compare(
                CmpOp::Eq,
                call(ValueId::HasSpawned, vec![event_player()]),
                ex(ExprKind::Bool(true)),
            )),
            cond(compare(
                CmpOp::Eq,
                call(ValueId::IsDummy, vec![event_player()]),
                ex(ExprKind::Bool(false)),
            )),
            cond(compare(CmpOp::Eq, index_var(var), num(start))),
        ],
        location: None,
        ..Rule::new(
            RULE_NAME,
            Event::OngoingEachPlayer,
            Block {
                stmts,
                ..Block::default()
            },
        )
    }
}

/// The rule for `program`, when its translations keep the language in a player variable.
pub(crate) fn generate(program: &Program) -> Option<Rule> {
    let t = program.translations.as_ref()?;
    let mode = t.player_var?;
    let var = program
        .players
        .entries
        .iter()
        .position(|d| d.name == LANGUAGE_INDEX)
        .and_then(|i| u32::try_from(i).ok())
        .map(VarId)?;
    let detection = choose(&t.langs).ok()?;
    Some(detect_language(var, &t.langs, detection, mode))
}
