//! Tests of the examples' output against `tests/fixtures/examples/<name>.txt`, which hold the bare
//! workshop code so they can be pasted into the game. CI also runs each example.
//!
//! Regenerate with `OW_UPDATE_FIXTURES=1 cargo test -p ow-workshop --test examples`.
// The fixtures are read from the filesystem, which WASM does not have.
#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::assert_workshop_code_eq;
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
    mode.compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
        .expect("compiles")
        .code
}

fn check(name: &str, code: &str) {
    let path = format!(
        "{}/tests/fixtures/examples/{name}.txt",
        env!("CARGO_MANIFEST_DIR")
    );
    if std::env::var_os("OW_UPDATE_FIXTURES").is_some() {
        std::fs::write(&path, code).expect("writes the fixture");
    }
    let expected = std::fs::read_to_string(&path).expect("reads the fixture");
    assert_workshop_code_eq(code, &expected);
}

#[test]
fn hello_world() {
    check("hello_world", &code(&hello_world::HelloWorld));
}

#[test]
fn zombie_survival() {
    check(
        "zombie_survival",
        &code(&zombie_survival::ZombieSurvival { waves: 5 }),
    );
}

#[test]
fn custom_settings() {
    let code = code(&custom_settings::CustomSettings);
    assert!(code.contains("Manche {0} sur {1}"), "{code}");
    check("custom_settings", &code);
}

/// The debug log of a debug build, with the examples' source locations.
fn log(mode: &impl Workshop) -> String {
    let out = mode
        .compile_with_debug(Lang::EnUs, Some(DebugMode::On(DebugOptions::default())))
        .expect("compiles");
    // The examples are included from `tests/`, which shows in the paths of their source.
    out.debug_log
        .expect("a log")
        .to_string()
        .replace("/tests/../", "/")
}

#[test]
fn hello_world_debug_log() {
    insta::assert_snapshot!(log(&hello_world::HelloWorld));
}

#[test]
fn zombie_survival_debug_log() {
    insta::assert_snapshot!(log(&zombie_survival::ZombieSurvival { waves: 5 }));
}
