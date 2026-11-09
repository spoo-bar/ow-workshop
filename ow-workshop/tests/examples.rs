//! Snapshot tests of the examples' output; CI also runs each example.
// `insta` reads snapshots from the filesystem, which WASM does not have.
#![cfg(not(target_arch = "wasm32"))]

use ow_workshop::prelude::*;

#[allow(dead_code)]
#[path = "../examples/hello_world.rs"]
mod hello_world;

#[allow(dead_code)]
#[path = "../examples/zombie_survival.rs"]
mod zombie_survival;

#[allow(dead_code)]
#[path = "../examples/custom_settings.rs"]
mod custom_settings;

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

#[test]
fn custom_settings() {
    let code = code(&custom_settings::CustomSettings);
    assert!(code.contains("Manche {0} sur {1}"), "{code}");
    insta::assert_snapshot!(code);
}
