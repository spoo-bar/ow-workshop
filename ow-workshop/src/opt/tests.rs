use super::*;
use crate::ir::OptOverride;
use crate::lower::testutil::*;
use crate::options::{CompileOptions, OptLevel};

fn over(enabled: bool, size: bool, strict: Option<bool>) -> Block {
    Block {
        opt: Some(OptOverride {
            enabled,
            size,
            strict,
        }),
        ..Block::default()
    }
}

fn scope(optimize: OptLevel) -> Scope {
    Scope::new(&options(optimize))
}

#[test]
fn defaults_follow_the_options() {
    let speed = scope(OptLevel::Speed);
    assert!(speed.enabled() && !speed.size() && !speed.strict() && !speed.aggressive_size());
    let size = scope(OptLevel::Size);
    assert!(size.enabled() && size.size());
    let off = scope(OptLevel::Off);
    assert!(!off.enabled() && !off.size());
}

#[test]
fn strict_and_aggressive_come_from_the_options() {
    let opts = CompileOptions {
        optimize: OptLevel::Size,
        strict: true,
        aggressive_size: true,
        ..CompileOptions::default()
    };
    let s = Scope::new(&opts);
    assert!(s.strict() && s.aggressive_size());
}

#[test]
fn aggressive_size_needs_size() {
    let opts = |optimize| CompileOptions {
        optimize,
        aggressive_size: true,
        ..CompileOptions::default()
    };
    assert!(!Scope::new(&opts(OptLevel::Speed)).aggressive_size());
    assert!(!Scope::new(&opts(OptLevel::Off)).aggressive_size());
    assert!(Scope::new(&opts(OptLevel::Size)).aggressive_size());
    // A block that turns size off also turns the whole-program rewrites off.
    let inner = Scope::new(&opts(OptLevel::Size)).enter(&over(true, false, None));
    assert!(!inner.size() && !inner.aggressive_size());
}

#[test]
fn a_block_overrides_its_parent_and_leaving_restores_it() {
    let top = scope(OptLevel::Speed);
    let sized = top.enter(&over(true, true, Some(true)));
    assert!(sized.size() && sized.strict());
    let disabled = sized.enter(&over(false, true, None));
    assert!(!disabled.enabled() && !disabled.size());
    assert!(disabled.strict(), "strict is inherited when not overridden");
    let back_on = disabled.enter(&over(true, false, Some(false)));
    assert!(back_on.enabled() && !back_on.size() && !back_on.strict());
    // The scopes are values: the outer one is untouched.
    assert_eq!(top, scope(OptLevel::Speed));
    assert!(sized.size());
    assert_eq!(top.enter(&Block::default()), top);
}

#[test]
fn a_block_can_turn_the_optimizer_on() {
    let on = scope(OptLevel::Off).enter(&over(true, false, None));
    assert!(on.enabled() && !on.size());
}

#[test]
fn the_pass_leaves_programs_alone_without_rules() {
    let mut p = program(vec![action()]);
    let before = p.clone();
    run_pass(&mut p);
    assert_eq!(p, before);
}

fn run_pass(p: &mut Program) {
    crate::lower::testutil::run(super::run, p, &options(OptLevel::Size)).expect("optimizes");
}
