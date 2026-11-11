//! The transpile-time debug log.
#![cfg(not(target_arch = "wasm32"))]

use ow_workshop::prelude::*;
use ow_workshop::{DebugLog, Scope};

#[derive(GlobalVars)]
struct Globals {
    wave: Global<Num>,
    #[ow(debug_label = "Counter")]
    counter: Global<Num>,
    #[ow(debug = false)]
    hidden: Global<Num>,
}

#[derive(PlayerVars)]
struct Players {
    kills: PlayerVar<Num>,
}

struct Mode;

impl Workshop for Mode {
    const NAME: &'static str = "Test";
    const DESCRIPTION: &'static str = "Test mode";

    fn settings(&self) -> ow_workshop::Settings {
        ow_workshop::Settings::unchecked()
    }

    type Globals = Globals;
    type Players = Players;

    fn rules<'a>(&'a self, g: &'a Globals, p: &'a Players) -> Vec<RuleDef<'a>> {
        let mut defs = Vec::new();

        let helper = Sub::named("helper");
        defs.push(subroutine(helper, |b, _| b.set(g.hidden, 1)));
        defs.push(rule("Run", OngoingGlobal, move |r, _| {
            r.condition(g.wave.get().gt(0));
            r.actions(|b| {
                b.chase_at_rate(g.counter, 10, 5, ChaseRateReeval::DestinationAndRate);
                b.call(helper);
            });
        }));
        defs.push(rule("Score", OngoingEachPlayer, |r, ev| {
            r.actions(|b| b.set(ev.event_player().var(p.kills), 0));
        }));
        defs
    }
}

fn compile(debug: DebugMode) -> CompileOutput {
    Mode.compile_with_debug(Lang::EnUs, Some(debug))
        .expect("compiles")
}

fn log() -> DebugLog {
    compile(DebugMode::On(DebugOptions::default()))
        .debug_log
        .expect("a log")
}

#[test]
fn only_a_debug_build_with_the_log_option_has_one() {
    assert!(compile(DebugMode::Off).debug_log.is_none());
    let mut options = DebugOptions::default();
    options.log = false;
    assert!(compile(DebugMode::On(options)).debug_log.is_none());
    assert!(
        compile(DebugMode::On(DebugOptions::default()))
            .debug_log
            .is_some()
    );
}

#[test]
fn variables_have_their_index_label_and_flags() {
    let log = log();
    let wave = log
        .variables
        .iter()
        .find(|v| v.name == "wave")
        .expect("wave");
    assert_eq!(wave.scope, Scope::Global);
    assert_eq!(wave.label, Some("wave"));
    assert!(wave.in_condition && !wave.chased);
    assert!(wave.location.is_some());
    let counter = log
        .variables
        .iter()
        .find(|v| v.name == "counter")
        .expect("counter");
    assert_eq!(counter.label, Some("Counter"));
    assert!(counter.chased);
    let hidden = log
        .variables
        .iter()
        .find(|v| v.name == "hidden")
        .expect("hidden");
    assert_eq!(hidden.label, None);
    let kills = log.variables.last().expect("kills");
    assert_eq!((kills.name.as_str(), kills.scope), ("kills", Scope::Player));
    let indices: Vec<_> = log.variables.iter().map(|v| v.index).collect();
    assert_eq!(indices, [0, 1, 2, 0]);
}

#[test]
fn subroutines_list_their_callers() {
    let log = log();
    assert_eq!(log.subroutines.len(), 1);
    assert_eq!(log.subroutines[0].name, "helper");
    assert_eq!(log.subroutines[0].callers, ["Run"]);
}

#[test]
fn rules_are_in_output_order_and_generated_ones_are_marked() {
    let log = log();
    let rules: Vec<_> = log
        .rules
        .iter()
        .map(|r| (r.name.as_str(), r.generated))
        .collect();
    assert_eq!(
        rules,
        [
            ("Debug: globals", true),
            ("Debug: players", true),
            ("Subroutine helper", false),
            ("Run", false),
            ("Score", false),
        ]
    );
    assert_eq!(log.rules[3].event, "Ongoing - Global");
    assert_eq!(log.rules[2].event, "Subroutine");
    assert!(log.rules[3].location.is_some());
    assert!(log.rules.iter().all(|r| r.elements > 0));
}

#[test]
fn totals_count_elements_with_and_without_the_panels() {
    let out = compile(DebugMode::On(DebugOptions::default()));
    let release = compile(DebugMode::Off);
    let totals = out.debug_log.expect("a log").totals;
    assert_eq!(totals.elements, out.element_count);
    assert_eq!(totals.elements_without_panels, release.element_count);
    assert!(totals.elements > totals.elements_without_panels);
    assert_eq!(
        (totals.globals, totals.players, totals.subroutines),
        (3, 1, 1)
    );
}

#[test]
fn warnings_are_the_outputs_warnings() {
    struct Chased;

    impl Workshop for Chased {
        const NAME: &'static str = "Test";
        const DESCRIPTION: &'static str = "Test mode";

        fn settings(&self) -> ow_workshop::Settings {
            ow_workshop::Settings::unchecked()
        }

        type Globals = Globals;
        type Players = ();

        fn rules<'a>(&'a self, g: &'a Globals, (): &'a ()) -> Vec<RuleDef<'a>> {
            vec![rule("Bad", OngoingGlobal, |r, _| {
                r.actions(|b| {
                    b.chase_at_rate(g.counter, 10, 9999, ChaseRateReeval::DestinationAndRate);
                    b.for_range(g.counter, range(0, 2), |_| {});
                });
            })]
        }
    }

    let out = Chased
        .compile_with_debug(Lang::EnUs, Some(DebugMode::On(DebugOptions::default())))
        .expect("compiles");
    assert_ne!(out.warnings, vec![]);
    let log = out.debug_log.expect("a log");
    assert_eq!(log.warnings, out.warnings);
    assert!(log.to_string().contains("chased-var-in-for"), "{log}");
}

#[test]
fn the_log_is_plain_text() {
    let text = log().to_string();
    assert!(text.starts_with("ow-workshop debug log"));
    for heading in ["Variables", "Subroutines", "Rules", "Totals", "Warnings"] {
        assert!(
            text.contains(&format!("\n{heading}\n")),
            "{heading}\n{text}"
        );
    }
    assert!(text.contains("global variables: 3 of 128"), "{text}");
    assert!(text.contains("called by Run"), "{text}");
    assert!(text.contains("[generated]"), "{text}");
}
