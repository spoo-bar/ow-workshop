use super::*;
use crate::ir::StmtKind;
use crate::opt::testutil::sh;
use crate::prelude::*;
use ow_workshop_data::Hero;

#[derive(GlobalVars)]
#[allow(dead_code)]
struct Globals {
    heroes: Global<Array<Hero>>,
    lengths: Global<Array<Num>>,
    spots: Global<Array<Vector>>,
    tags: Global<Array<Str>>,
}

fn globals() -> Globals {
    Globals::declare(&mut VarRegistry::default())
}

/// The `Set` statements of a block as `(variable, value)`.
fn sets(block: Block) -> Vec<(String, String)> {
    block
        .into_ir()
        .stmts
        .iter()
        .map(|stmt| {
            let StmtKind::Set { target, value } = &stmt.kind else {
                panic!("not a set: {:?}", stmt.kind);
            };
            let ir::Place::Global(var) = target else {
                panic!("not a global: {target:?}");
            };
            (format!("g{}", var.0), sh(value))
        })
        .collect()
}

#[test]
fn one_set_per_column() {
    let g = globals();
    let mut b = Block::new();
    b.columns(
        (g.heroes, g.lengths),
        [
            (Hero::Ana, Some(3)),
            (Hero::Soldier, Some(8)),
            (Hero::WreckingBall, None),
        ],
    );
    assert_eq!(
        sets(b),
        [
            (
                "g0".to_owned(),
                "[Hero.ana, Hero.soldier, Hero.wreckingBall]".to_owned()
            ),
            ("g1".to_owned(), "[3, 8, Null]".to_owned()),
        ]
    );
}

#[test]
fn plain_values_and_options_mix_in_any_column() {
    let g = globals();
    let mut b = Block::new();
    b.columns((g.lengths, g.tags), [(1, "one"), (2, "two")]);
    b.columns((g.lengths, g.tags), [(None::<i32>, Some("x"))]);
    let out = sets(b);
    assert_eq!(out[0].1, "[1, 2]");
    assert!(out[1].1.starts_with("[\"one\"("), "{}", out[1].1);
    assert_eq!(out[2].1, "[Null]");
}

#[test]
fn up_to_eight_columns() {
    #[derive(GlobalVars)]
    #[allow(dead_code)]
    struct Wide {
        a: Global<Array<Num>>,
        b: Global<Array<Num>>,
        c: Global<Array<Num>>,
        d: Global<Array<Num>>,
        e: Global<Array<Num>>,
        f: Global<Array<Num>>,
        g: Global<Array<Num>>,
        h: Global<Array<Num>>,
    }
    let w = Wide::declare(&mut VarRegistry::default());
    let mut b = Block::new();
    b.columns((w.a, w.b), [(1, 2)]);
    b.columns((w.a, w.b, w.c), [(1, 2, 3)]);
    b.columns((w.a, w.b, w.c, w.d), [(1, 2, 3, 4)]);
    b.columns((w.a, w.b, w.c, w.d, w.e), [(1, 2, 3, 4, 5)]);
    b.columns((w.a, w.b, w.c, w.d, w.e, w.f), [(1, 2, 3, 4, 5, 6)]);
    b.columns((w.a, w.b, w.c, w.d, w.e, w.f, w.g), [(1, 2, 3, 4, 5, 6, 7)]);
    b.columns(
        (w.a, w.b, w.c, w.d, w.e, w.f, w.g, w.h),
        [(1, 2, 3, 4, 5, 6, 7, 8)],
    );
    assert_eq!(sets(b).len(), 2 + 3 + 4 + 5 + 6 + 7 + 8);
}

#[test]
fn empty_tables_assign_empty_arrays() {
    let g = globals();
    let mut b = Block::new();
    b.columns((g.heroes, g.lengths), Vec::<(Hero, i32)>::new());
    assert_eq!(
        sets(b).iter().map(|s| s.1.as_str()).collect::<Vec<_>>(),
        ["[]", "[]"]
    );
}

#[test]
fn compressed_packs_literal_number_and_vector_columns() {
    let g = globals();
    let mut b = Block::new();
    b.columns(
        (g.lengths, g.spots),
        [
            (3, vect(1, 2, 3)),
            (8, vect(-4.5, 0, 250.25)),
            (11, vect(10, 20, 30)),
        ],
    )
    .compressed();
    let out = sets(b);
    assert!(out[0].1.starts_with("Decompress("), "{}", out[0].1);
    assert!(out[1].1.starts_with("Decompress("), "{}", out[1].1);
}

#[test]
fn compressed_leaves_other_columns_alone() {
    let g = globals();
    let mut b = Block::new();
    b.columns(
        (g.heroes, g.lengths),
        [(Hero::Ana, Some(3)), (Hero::Soldier, None)],
    )
    .compressed();
    let out = sets(b);
    assert_eq!(out[0].1, "[Hero.ana, Hero.soldier]");
    // `Null` is not a number, so the column stays as written.
    assert_eq!(out[1].1, "[3, Null]");
}

#[test]
fn compressed_leaves_columns_that_do_not_fit() {
    let g = globals();
    let mut b = Block::new();
    b.columns((g.lengths, g.heroes), [(1_000_000, Hero::Ana)])
        .compressed();
    assert_eq!(sets(b)[0].1, "[1000000]");
}

#[test]
fn statements_are_located_at_the_call() {
    let g = globals();
    let mut b = Block::new();
    let line = line!() + 1;
    b.columns((g.heroes, g.lengths), [(Hero::Ana, 1)]);
    let stmts = b.into_ir().stmts;
    assert!(stmts.iter().all(|s| s.loc.is_some_and(|l| l.line == line)));
}
