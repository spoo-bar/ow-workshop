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

/// The release output: debug off, whatever the profile of the test build is.
fn code(mode: &impl Workshop) -> String {
    mode.compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
        .expect("compiles")
        .code
}

/// The debug build, with the default debug options.
fn debug_code(mode: &impl Workshop) -> String {
    mode.compile_with_debug(Lang::EnUs, Some(DebugMode::On(DebugOptions::default())))
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

/// `hello_world` in a debug build: the output of the release build, plus the header comment.
#[test]
fn hello_world_debug() {
    let debug = debug_code(&hello_world::HelloWorld);
    assert!(debug.starts_with("// ow-workshop debug build\n"));
    let release = code(&hello_world::HelloWorld);
    for rule in release.split("\nrule (").skip(1) {
        let rule = rule.trim_end();
        assert!(
            debug.contains(rule),
            "a rule of the release build changed:\n{rule}"
        );
    }
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
