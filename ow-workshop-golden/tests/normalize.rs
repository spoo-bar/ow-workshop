//! One test per normalization rule, on small excerpts.

use ow_workshop_golden::{Options, normalize, normalize_with};

fn actions(body: &str) -> String {
    format!("rule (\"R\") {{\n    actions {{\n{body}    }}\n}}\n")
}

#[test]
fn numbers_use_the_shortest_round_trip_form() {
    let out = normalize(&actions(
        "        Wait(0.50, Ignore Condition);\n        Set Global Variable(v, Vector(-8.34, 0.00, 29.98));\n",
    ));
    assert!(out.contains("Wait(0.5, Ignore Condition);"));
    assert!(out.contains("Vector(-8.34, 0, 29.98)"));
}

#[test]
fn numbers_in_strings_and_names_are_untouched() {
    let out = normalize(&actions(
        "        Small Message(x, Custom String(\"v1.50\"));\n        Set Global Variable(a1.50x, 3);\n",
    ));
    assert!(out.contains("\"v1.50\""));
    assert!(out.contains("a1.50x"));
}

#[test]
fn missing_end_is_added() {
    let out = normalize(&actions(
        "        While(True);\n            Wait(1, Ignore Condition);\n",
    ));
    let closed = normalize(&actions(
        "        While(True);\n            Wait(1, Ignore Condition);\n        End;\n",
    ));
    assert_eq!(out, closed);
    assert!(out.contains("        End;\n"));
}

#[test]
fn nested_unclosed_blocks_each_get_an_end() {
    let out = normalize(&actions(
        "        If(a);\n            While(b);\n                Wait(1, Ignore Condition);\n",
    ));
    assert_eq!(out.matches("End;").count(), 2);
}

#[test]
fn else_does_not_open_a_block() {
    let out = normalize(&actions(
        "        If(a);\n        Else;\n        Wait(1, Ignore Condition);\n",
    ));
    assert_eq!(out.matches("End;").count(), 1);
}

#[test]
fn custom_string_loses_trailing_null_padding() {
    let out = normalize(&actions(
        "        Big Message(All Players(All Teams), Custom String(\"Hi\", Null, Null, Null));\n        Big Message(x, Custom String(\"{0}\", Event Player, Null, Null));\n        Big Message(x, Custom String(\"{0} {1}\", Null, 5, Null));\n",
    ));
    assert!(out.contains("Custom String(\"Hi\")"));
    assert!(out.contains("Custom String(\"{0}\", Event Player)"));
    assert!(out.contains("Custom String(\"{0} {1}\", Null, 5)"));
}

#[test]
fn custom_string_padding_is_removed_when_nested() {
    let out = normalize(&actions(
        "        Set Global Variable(s, Custom String(\"{0}\", Custom String(\"a, b)\", Null, Null, Null), Null, Null));\n",
    ));
    assert!(out.contains("Custom String(\"{0}\", Custom String(\"a, b)\"))"));
}

#[test]
fn quoted_comment_lines_are_removed() {
    let with = normalize(&actions(
        "        \"advance\"\n        Wait(1, Ignore Condition);\n",
    ));
    let without = normalize(&actions("        Wait(1, Ignore Condition);\n"));
    assert_eq!(with, without);
}

#[test]
fn skip_label_line_is_removed() {
    let with = normalize(&actions(
        "        //skip:\n        Wait(1, Ignore Condition);\n",
    ));
    let without = normalize(&actions("        Wait(1, Ignore Condition);\n"));
    assert_eq!(with, without);
}

const SETTINGS_A: &str = "settings\n{\n\tmain\n\t{\n\t\tMode Name: \"X\"\n\t\tDescription: \"d\"\n\t}\n\n\tlobby\n\t{\n\t\tB: 1\n\t\tA: 2\n\t}\n}\n";
const SETTINGS_B: &str = "settings\n{\n\tlobby\n\t{\n\t\tA: 2\n\n\t\tB: 1\n\t}\n\tmain\n\t{\n\t\tDescription: \"d\"\n\t\tMode Name: \"X\"\n\t}\n}\n";

#[test]
fn settings_ignore_key_order_and_blank_lines() {
    assert_eq!(normalize(SETTINGS_A), normalize(SETTINGS_B));
}

#[test]
fn settings_values_still_matter() {
    assert_ne!(
        normalize(SETTINGS_A),
        normalize(&SETTINGS_B.replace("A: 2", "A: 3"))
    );
}

#[test]
fn main_block_is_ignored_only_on_request() {
    let other = SETTINGS_A.replace("\"X\"", "\"Y\"");
    assert_ne!(normalize(SETTINGS_A), normalize(&other));
    let opts = Options::default().ignore_main();
    assert_eq!(
        normalize_with(SETTINGS_A, &opts),
        normalize_with(&other, &opts)
    );
    assert!(!normalize_with(SETTINGS_A, &opts).contains("main"));
}

#[test]
fn indentation_and_line_endings_do_not_matter() {
    let tidy = actions("        Wait(1, Ignore Condition);\n");
    let messy = tidy.replace('\n', "\r\n").replace("    ", "\t");
    assert_eq!(normalize(&tidy), normalize(&format!("\u{feff}{messy}")));
}

#[test]
fn normalizing_twice_equals_once() {
    let text = format!(
        "{SETTINGS_B}variables {{\n    global:\n        0: x\n}}\n{}",
        actions(
            "        \"c\"\n        If(a);\n            Set Global Variable(x, 0.50);\n            Big Message(All Players(All Teams), Custom String(\"Hi\", Null, Null, Null));\n        Else;\n            While(True);\n"
        )
    );
    let once = normalize(&text);
    assert_eq!(normalize(&once), once);
}
