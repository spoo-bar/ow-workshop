//! Table assignment compiles to one array per column.

mod common;

use common::assert_workshop_code_eq;
use ow_workshop::prelude::*;

#[derive(GlobalVars)]
struct Globals {
    wave_heroes: Global<Array<Hero>>,
    wave_lengths: Global<Array<Num>>,
}

struct Waves {
    compressed: bool,
}

impl Workshop for Waves {
    type Globals = Globals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &Globals, (): &()) {
        ws.rule("Waves", OngoingGlobal, |r, _| {
            r.actions(|b| {
                let rows = [
                    (Hero::Ana, Some(3)),
                    (Hero::Soldier, Some(8)),
                    (Hero::WreckingBall, None),
                ];
                let columns = b.columns((g.wave_heroes, g.wave_lengths), rows);
                if self.compressed {
                    columns.compressed();
                }
            });
        });
    }
}

#[test]
fn rows_become_one_array_per_column() {
    let code = Waves { compressed: false }
        .compile(Lang::EnUs)
        .expect("compiles")
        .code;
    let expected = r#"
variables {
    global:
        0: wave_heroes
        1: wave_lengths
}

rule ("Waves") {
    event {
        Ongoing - Global;
    }
    actions {
        Set Global Variable(wave_heroes, Array(Hero(Ana), Hero(Soldier: 76), Hero(Wrecking Ball)));
        Set Global Variable(wave_lengths, Array(3, 8, Null));
    }
}
"#;
    assert_workshop_code_eq(&code, expected);
}

#[test]
fn compressed_leaves_the_column_with_null_and_heroes_alone() {
    // Heroes are not numbers, and `Null` is not 0: nothing here can be packed.
    let plain = Waves { compressed: false }
        .compile(Lang::EnUs)
        .expect("compiles");
    let packed = Waves { compressed: true }
        .compile(Lang::EnUs)
        .expect("compiles");
    assert_eq!(plain.code, packed.code);
}

#[derive(GlobalVars)]
struct SpawnGlobals {
    delays: Global<Array<Num>>,
    spots: Global<Array<Vector>>,
}

struct Spawns {
    compressed: bool,
}

impl Workshop for Spawns {
    type Globals = SpawnGlobals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &SpawnGlobals, (): &()) {
        ws.rule("Spawns", OngoingGlobal, |r, _| {
            r.actions(|b| {
                let rows = [
                    (3, vect(1, 2, 3)),
                    (8, vect(-4.5, 0, 250.25)),
                    (11, vect(10, 20, 30)),
                ];
                let columns = b.columns((g.delays, g.spots), rows);
                if self.compressed {
                    columns.compressed();
                }
            });
        });
    }
}

#[test]
fn compressed_literal_columns_become_decoded_strings() {
    let plain = Spawns { compressed: false }
        .compile(Lang::EnUs)
        .expect("compiles");
    assert!(
        plain
            .code
            .contains("Set Global Variable(delays, Array(3, 8, 11));"),
        "{}",
        plain.code
    );

    let packed = Spawns { compressed: true }
        .compile(Lang::EnUs)
        .expect("compiles");
    let code = &packed.code;
    assert!(!code.contains("Array(3, 8, 11)"), "{code}");
    // Each column is a mapped array over a split string.
    assert_eq!(code.matches("Mapped Array(").count(), 2 * 2, "{code}");
    assert!(
        code.contains("Set Global Variable(spots, Mapped Array("),
        "{code}"
    );
}
