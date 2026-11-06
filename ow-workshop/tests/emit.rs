//! End-to-end: a mod compiles to workshop text.

mod common;

use common::assert_workshop_code_eq;
use ow_workshop::prelude::*;

#[derive(GlobalVars)]
struct Globals {
    round: Global<Num>,
}

struct Mode;

impl Workshop for Mode {
    type Globals = Globals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &Globals, (): &()) {
        ws.rule("Init", OngoingGlobal, |r, _| {
            r.actions(|b| b.set(g.round, 1));
        });
    }
}

#[test]
fn compiles_variables_and_a_rule() {
    let code = Mode.compile(Lang::EnUs).expect("compiles").code;
    let expected = r#"
variables {
    global:
        0: round
}

rule ("Init") {
    event {
        Ongoing - Global;
    }
    actions {
        Set Global Variable(round, 1);
    }
}
"#;
    assert_workshop_code_eq(&code, expected);
}
