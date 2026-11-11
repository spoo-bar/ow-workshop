//! Suppression levels, deduplication and ordering of warnings, end to end.

use ow_workshop::prelude::*;
use ow_workshop::{Diagnostic, Warning};

/// Which levels suppress `Wait9999`.
#[derive(Clone, Copy, Default)]
struct Levels {
    global: bool,
    rule: bool,
    block: bool,
}

struct Mod(Levels);

impl Workshop for Mod {
    fn settings(&self) -> ow_workshop::Settings {
        ow_workshop::Settings::unchecked()
    }

    type Globals = ();
    type Players = ();

    fn build(&self, ws: &mut Builder, (): &(), (): &()) {
        let levels = self.0;
        ws.rule("Rule", OngoingGlobal, |r, _| {
            if levels.rule {
                r.suppress(Warning::Wait9999);
            }
            r.actions(|b| {
                b.wait(9999);
                if levels.block {
                    b.suppress(Warning::Wait9999, |b| {
                        b.wait(9999.0);
                        // Nested blocks inherit the suppression.
                        b.if_(true, |b| b.wait(9999.0 + 0.0));
                    });
                }
            });
        });
    }

    fn options(&self) -> CompileOptions {
        let mut options = CompileOptions::default();
        if self.0.global {
            options.suppress.insert(Warning::Wait9999);
        }
        options
    }
}

fn warned(levels: Levels) -> Vec<Diagnostic> {
    Mod(levels)
        .compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
        .expect("compiles")
        .warnings
}

#[test]
fn nothing_is_suppressed_by_default() {
    assert_eq!(warned(Levels::default()).len(), 1);
}

#[test]
fn a_block_suppresses_only_what_it_encloses() {
    let warnings = warned(Levels {
        block: true,
        ..Levels::default()
    });
    // The `wait` before the block still warns; the two inside it, one nested, do not.
    assert_eq!(warnings.len(), 1, "{warnings:?}");
}

#[test]
fn a_rule_suppresses_everything_in_it() {
    let warnings = warned(Levels {
        rule: true,
        ..Levels::default()
    });
    assert!(warnings.is_empty(), "{warnings:?}");
}

#[test]
fn the_options_suppress_everywhere() {
    let warnings = warned(Levels {
        global: true,
        ..Levels::default()
    });
    assert!(warnings.is_empty(), "{warnings:?}");
}

#[test]
fn suppressing_one_warning_keeps_the_others() {
    struct Other;
    impl Workshop for Other {
        fn settings(&self) -> ow_workshop::Settings {
            ow_workshop::Settings::unchecked()
        }

        type Globals = ();
        type Players = ();

        fn build(&self, ws: &mut Builder, (): &(), (): &()) {
            ws.rule("Rule", OngoingGlobal, |r, _| {
                r.actions(|b| {
                    b.suppress(Warning::DarkColor, |b| b.wait(9999));
                });
            });
        }
    }
    let warnings = Other
        .compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
        .expect("compiles")
        .warnings;
    assert_eq!(warnings.len(), 1);
}

struct Repeated;

impl Workshop for Repeated {
    fn settings(&self) -> ow_workshop::Settings {
        ow_workshop::Settings::unchecked()
    }

    type Globals = ();
    type Players = ();

    fn build(&self, ws: &mut Builder, (): &(), (): &()) {
        ws.rule("Rule", OngoingGlobal, |r, _| {
            r.actions(|b| {
                // The same call site raised twice gives one warning.
                for _ in 0..2 {
                    b.wait(9999);
                }
            });
        });
    }
}

#[test]
fn identical_warnings_are_reported_once() {
    let warnings = Repeated
        .compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
        .expect("compiles")
        .warnings;
    assert_eq!(warnings.len(), 1, "{warnings:?}");
}

fn early(b: &mut Block) {
    b.wait(9999);
}

fn late(b: &mut Block) {
    b.wait(9999);
}

struct Unordered;

impl Workshop for Unordered {
    fn settings(&self) -> ow_workshop::Settings {
        ow_workshop::Settings::unchecked()
    }

    type Globals = ();
    type Players = ();

    fn build(&self, ws: &mut Builder, (): &(), (): &()) {
        ws.rule("Rule", OngoingGlobal, |r, _| {
            r.actions(|b| {
                late(b);
                early(b);
            });
        });
    }
}

#[test]
fn warnings_are_ordered_by_location() {
    let warnings = Unordered
        .compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
        .expect("compiles")
        .warnings;
    let lines: Vec<_> = warnings
        .iter()
        .map(|w| w.location.expect("located").line)
        .collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[0] < lines[1], "{lines:?}");
}
