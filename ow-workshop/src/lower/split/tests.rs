use super::*;
use crate::ir::{StmtKind, VarId};
use crate::lower::testutil::*;
use crate::options::OptLevel;

fn var(k: u32) -> Expr {
    expr(ExprKind::GlobalVar(VarId(k)))
}

fn template(tokens: Vec<StrToken>, args: Vec<Expr>) -> Expr {
    expr(ExprKind::Str(StrExpr::Custom { tokens, args }))
}

fn text(s: &str) -> StrToken {
    StrToken::Text(s.to_owned())
}

fn split_one(e: Expr) -> Expr {
    let mut p = program(vec![StmtKind::Action {
        id: ow_workshop_data::ActionId::Wait,
        args: vec![e],
    }]);
    run_pass(&mut p);
    let StmtKind::Action { args, .. } = &p.rules[0].body.stmts[0].kind else {
        panic!("not an action");
    };
    args[0].clone()
}

fn run_pass(p: &mut crate::ir::Program) {
    crate::lower::testutil::run(super::run, p, &options(OptLevel::Off)).expect("splits");
}

/// The text a string stands for, with arguments rendered as `<k>`.
fn expand(e: &Expr) -> String {
    match e.kind() {
        ExprKind::GlobalVar(v) => format!("<{}>", v.0),
        ExprKind::Str(StrExpr::Custom { tokens, args }) => tokens
            .iter()
            .map(|t| match t {
                StrToken::Text(s) => s.clone(),
                StrToken::Arg(i) => expand(&args[*i]),
            })
            .collect(),
        other => panic!("unexpected {other:?}"),
    }
}

/// Asserts every nested custom string respects both limits.
fn assert_valid(e: &Expr) {
    let ExprKind::Str(StrExpr::Custom { tokens, args }) = e.kind() else {
        return;
    };
    let chars: usize = tokens
        .iter()
        .map(|t| match t {
            StrToken::Text(s) => s.chars().count(),
            StrToken::Arg(_) => 3,
        })
        .sum();
    assert!(chars <= MAX_CHARS, "{chars} characters in {tokens:?}");
    assert!(args.len() <= MAX_ARGS, "{} arguments", args.len());
    assert!(
        tokens
            .iter()
            .all(|t| !matches!(t, StrToken::Arg(i) if *i >= args.len()))
    );
    args.iter().for_each(assert_valid);
}

fn depth(e: &Expr) -> usize {
    match e.kind() {
        ExprKind::Str(StrExpr::Custom { args, .. }) => {
            1 + args.iter().map(depth).max().unwrap_or(0)
        }
        _ => 0,
    }
}

#[test]
fn short_strings_are_untouched() {
    let e = template(vec![text("hello")], vec![]);
    let out = split_one(e.clone());
    assert_eq!(show(&out), show(&e));
    let e = template(
        vec![text("a"), StrToken::Arg(0), text("b"), StrToken::Arg(1)],
        vec![var(1), var(2)],
    );
    assert_eq!(show(&split_one(e.clone())), show(&e));
}

#[test]
fn a_300_character_string_nests() {
    let long = "x".repeat(300);
    let out = split_one(template(vec![text(&long)], vec![]));
    assert_valid(&out);
    assert_eq!(expand(&out), long);
    assert_eq!(depth(&out), 3);
    // The first string keeps room for the placeholder of the remainder.
    let ExprKind::Str(StrExpr::Custom { tokens, .. }) = out.kind() else {
        panic!()
    };
    assert_eq!(tokens, &[text(&"x".repeat(125)), StrToken::Arg(0)]);
}

#[test]
fn seven_distinct_args_nest() {
    let tokens: Vec<_> = (0..7).map(StrToken::Arg).collect();
    let args: Vec<_> = (1..=7).map(var).collect();
    let out = split_one(template(tokens, args));
    assert_valid(&out);
    assert_eq!(expand(&out), "<1><2><3><4><5><6><7>");
    assert_eq!(
        show(&out),
        "\"{0}{1}{2}\"(g1, g2, \"{0}{1}{2}\"(g3, g4, \"{0}{1}{2}\"(g5, g6, g7)))"
    );
}

#[test]
fn a_reused_arg_is_deduplicated() {
    let tokens: Vec<_> = (0..5).map(StrToken::Arg).collect();
    let out = split_one(template(tokens, vec![var(1); 5]));
    assert_eq!(show(&out), "\"{0}{0}{0}{0}{0}\"(g1)");
}

#[test]
fn random_arguments_are_never_deduplicated() {
    use ow_workshop_data::ValueId;
    let roll = call(ValueId::RandomRandint, vec![num(1.0), num(6.0)]);
    let out = split_one(template(
        vec![StrToken::Arg(0), StrToken::Arg(1)],
        vec![roll.clone(), roll],
    ));
    let ExprKind::Str(StrExpr::Custom { tokens, args }) = out.kind() else {
        panic!()
    };
    assert_eq!(args.len(), 2);
    assert_eq!(tokens, &[StrToken::Arg(0), StrToken::Arg(1)]);
}

#[test]
fn an_arg_near_the_limit_moves_to_the_next_string() {
    let out = split_one(template(
        vec![text(&"a".repeat(124)), StrToken::Arg(0), text("zz")],
        vec![var(1)],
    ));
    assert_valid(&out);
    assert_eq!(expand(&out), format!("{}<1>zz", "a".repeat(124)));
}

#[test]
fn nested_templates_inside_arguments_are_split_too() {
    let long = template(vec![text(&"y".repeat(200))], vec![]);
    let out = split_one(template(vec![StrToken::Arg(0)], vec![long]));
    assert_valid(&out);
    assert_eq!(expand(&out), "y".repeat(200));
}

#[cfg(not(target_arch = "wasm32"))]
mod random {
    use proptest::prelude::*;

    use super::*;

    fn pool_arg() -> impl Strategy<Value = u32> {
        1u32..=5
    }

    fn piece() -> impl Strategy<Value = Result<String, u32>> {
        prop_oneof![
            3 => "[a-c é{}\u{3000}]{0,200}".prop_map(Ok),
            2 => pool_arg().prop_map(Err),
        ]
    }

    proptest! {
        #[test]
        fn random_templates_split_validly_and_losslessly(pieces in prop::collection::vec(piece(), 0..30)) {
            let mut tokens = Vec::new();
            let mut args = Vec::new();
            let mut original = String::new();
            for p in &pieces {
                match p {
                    Ok(s) => {
                        original.push_str(s);
                        tokens.push(StrToken::Text(s.clone()));
                    }
                    Err(k) => {
                        original.push_str(&["<", &k.to_string(), ">"].concat());
                        tokens.push(StrToken::Arg(args.len()));
                        args.push(var(*k));
                    }
                }
            }
            let out = split_one(template(tokens, args));
            assert_valid(&out);
            prop_assert_eq!(expand(&out), original);
        }
    }
}
