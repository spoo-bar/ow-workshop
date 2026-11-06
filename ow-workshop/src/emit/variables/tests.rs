use ow_workshop_data::Lang;

use super::{write_subroutines, write_variables};
use crate::emit::{Cx, emit};
use crate::ir::{Event, Program, Rule, SubDecl, VarDecl};
use crate::lower::testutil::block;

fn cx(program: &Program) -> Cx<'_> {
    Cx {
        lang: Lang::EnUs,
        program,
        options: &program.options,
    }
}

fn variables(program: &mut Program) -> String {
    program.globals.allocate().unwrap();
    program.players.allocate().unwrap();
    let mut out = String::new();
    write_variables(&mut out, cx(program)).unwrap();
    out
}

fn subroutines(program: &mut Program) -> String {
    program.subroutines.allocate().unwrap();
    let mut out = String::new();
    write_subroutines(&mut out, cx(program)).unwrap();
    out
}

#[test]
fn globals_only() {
    let mut program = Program::default();
    program.globals.declare(VarDecl::new("A"));
    program.globals.declare(VarDecl::new("foo"));
    assert_eq!(
        variables(&mut program),
        "variables {\n    global:\n        0: A\n        1: foo\n}\n\n"
    );
}

#[test]
fn players_only() {
    let mut program = Program::default();
    program.players.declare(VarDecl::new("score"));
    assert_eq!(
        variables(&mut program),
        "variables {\n    player:\n        0: score\n}\n\n"
    );
}

#[test]
fn both_sections() {
    let mut program = Program::default();
    program.globals.declare(VarDecl::new("A"));
    program.players.declare(VarDecl::new("score"));
    assert_eq!(
        variables(&mut program),
        "variables {\n    global:\n        0: A\n    player:\n        0: score\n}\n\n"
    );
}

#[test]
fn no_variables_print_nothing() {
    assert_eq!(variables(&mut Program::default()), "");
}

#[test]
fn indices_ascend_whatever_the_declaration_order() {
    let mut program = Program::default();
    program.globals.declare(VarDecl {
        index: Some(5),
        ..VarDecl::new("late")
    });
    program.globals.declare(VarDecl {
        index: Some(1),
        ..VarDecl::new("early")
    });
    let text = variables(&mut program);
    assert!(text.find("1: early").unwrap() < text.find("5: late").unwrap());
}

#[test]
fn subroutine_block() {
    let mut program = Program::default();
    program.subroutines.declare(SubDecl::new("Sub0"));
    program.subroutines.declare(SubDecl::new("bar"));
    assert_eq!(
        subroutines(&mut program),
        "subroutines {\n    0: Sub0\n    1: bar\n}\n\n"
    );
    assert_eq!(subroutines(&mut Program::default()), "");
}

#[test]
fn blocks_are_localized() {
    let mut program = Program::default();
    program.globals.declare(VarDecl::new("A"));
    program.globals.allocate().unwrap();
    let mut out = String::new();
    let cx = Cx {
        lang: Lang::DeDe,
        ..cx(&program)
    };
    write_variables(&mut out, cx).unwrap();
    assert!(out.starts_with("Variablen {\n    global:\n"), "{out}");
}

#[test]
fn excluded_variables_block_leaves_only_the_rules() {
    let mut program = Program::default();
    program.globals.declare(VarDecl::new("A"));
    program.subroutines.declare(SubDecl::new("Sub0"));
    program.globals.allocate().unwrap();
    program.subroutines.allocate().unwrap();
    program
        .rules
        .push(Rule::new("r", Event::OngoingGlobal, block(vec![])));
    let full = emit(&program, Lang::EnUs, None).unwrap();
    assert!(full.starts_with("variables {"));
    assert!(full.contains("subroutines {"));
    assert!(full.ends_with("}\n\n"));

    program.options.exclude_variables_block = true;
    let rules_only = emit(&program, Lang::EnUs, None).unwrap();
    assert!(rules_only.starts_with("rule ("), "{rules_only}");
}

#[test]
fn output_order_is_variables_subroutines_rules() {
    let mut program = Program::default();
    program.globals.declare(VarDecl::new("A"));
    program.subroutines.declare(SubDecl::new("Sub0"));
    program.globals.allocate().unwrap();
    program.subroutines.allocate().unwrap();
    program
        .rules
        .push(Rule::new("r", Event::OngoingGlobal, block(vec![])));
    let text = emit(&program, Lang::EnUs, None).unwrap();
    let at = |needle: &str| text.find(needle).unwrap();
    assert!(at("variables {") < at("subroutines {"));
    assert!(at("subroutines {") < at("rule ("));
}
