//! Snapshot tests of the examples' output; CI also runs each example.

use ow_workshop::prelude::*;

#[allow(dead_code)]
#[path = "../examples/hello_world.rs"]
mod hello_world;

fn code(mode: &impl Workshop) -> String {
    mode.compile(Lang::EnUs).expect("compiles").code
}

#[test]
fn hello_world() {
    insta::assert_snapshot!(code(&hello_world::HelloWorld));
}
