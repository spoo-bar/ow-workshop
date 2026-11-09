//! Snapshot tests of the examples' output; CI also runs each example.

use ow_workshop::prelude::*;

#[allow(dead_code)]
#[path = "../examples/hello_world.rs"]
mod hello_world;

#[allow(dead_code)]
#[path = "../examples/zombie_survival.rs"]
mod zombie_survival;

fn code(mode: &impl Workshop) -> String {
    mode.compile(Lang::EnUs).expect("compiles").code
}

#[test]
fn hello_world() {
    insta::assert_snapshot!(code(&hello_world::HelloWorld));
}

#[test]
fn zombie_survival() {
    insta::assert_snapshot!(code(&zombie_survival::ZombieSurvival { waves: 5 }));
}
