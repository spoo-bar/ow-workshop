//! Debug mode selection: which builds get the debug header, and that `Off` is the release output.
#![cfg(not(target_arch = "wasm32"))]

use ow_workshop::prelude::*;

struct Mode;

impl Workshop for Mode {
    fn mode_name(&self) -> &'static str {
        "Test"
    }

    fn description(&self) -> &'static str {
        "Test mode"
    }

    fn settings(&self) -> ow_workshop::Settings {
        ow_workshop::Settings::unchecked()
    }

    type Globals = ();
    type Players = ();

    fn build(&self, ws: &mut Builder, (): &(), (): &()) {
        ws.rule("Header", OngoingGlobal, |r, _| {
            r.actions(|b| b.hud_header("Hello"));
        });
    }
}

struct Empty;

impl Workshop for Empty {
    fn mode_name(&self) -> &'static str {
        "Test"
    }

    fn description(&self) -> &'static str {
        "Test mode"
    }

    fn settings(&self) -> ow_workshop::Settings {
        ow_workshop::Settings::unchecked()
    }

    type Globals = ();
    type Players = ();

    fn build(&self, _: &mut Builder, (): &(), (): &()) {}
}

const HEADER: &str = "// ow-workshop debug build\n";

fn code(mode: DebugMode) -> String {
    Mode.compile_with_debug(Lang::EnUs, Some(mode))
        .expect("compiles")
        .code
}

#[test]
fn a_debug_build_starts_with_a_recognizable_comment() {
    let on = code(DebugMode::On(DebugOptions::default()));
    assert!(on.starts_with(HEADER), "{on}");
}

#[test]
fn off_is_the_release_output() {
    let off = code(DebugMode::Off);
    let on = code(DebugMode::On(DebugOptions::default()));
    assert!(!off.contains("ow-workshop debug build"));
    assert_eq!(on.strip_prefix(HEADER), Some(off.as_str()));
}

#[test]
fn auto_follows_the_build_profile() {
    let auto = code(DebugMode::Auto);
    assert_eq!(auto.starts_with(HEADER), cfg!(debug_assertions));
    // With no override, the mod's own option (the default, `Auto`) applies.
    assert_eq!(Mode.compile(Lang::EnUs).expect("compiles").code, auto);
}

#[test]
fn an_empty_mod_stays_empty() {
    for mode in [
        DebugMode::Off,
        DebugMode::Auto,
        DebugMode::On(DebugOptions::default()),
    ] {
        let out = Empty
            .compile_with_debug(Lang::EnUs, Some(mode))
            .expect("compiles");
        assert_eq!(out.code, "", "{mode:?}");
    }
}

#[test]
fn the_header_costs_no_elements() {
    let count = |mode| {
        Mode.compile_with_debug(Lang::EnUs, Some(mode))
            .expect("compiles")
            .element_count
    };
    assert_eq!(
        count(DebugMode::Off),
        count(DebugMode::On(DebugOptions::default()))
    );
}
