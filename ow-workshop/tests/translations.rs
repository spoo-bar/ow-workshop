//! Translated strings: `t!`, `tr(..)` and the catalog they fill in.

use ow_workshop::TranslationLang::{En, Fr, Ja};
use ow_workshop::prelude::*;
use ow_workshop::{Catalog, Entry};

#[derive(GlobalVars)]
struct Globals {
    title: Global<Str>,
    money: Global<Num>,
}

struct Mode {
    translations: Option<Translations>,
    body: fn(&mut Block, &Globals),
}

impl Workshop for Mode {
    fn settings(&self) -> ow_workshop::Settings {
        ow_workshop::Settings::unchecked()
    }

    type Globals = Globals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &Globals, (): &()) {
        ws.rule("Show", OngoingGlobal, |r, _| {
            r.actions(|b| (self.body)(b, g));
        });
    }

    fn translations(&self) -> Option<Translations> {
        self.translations.clone()
    }
}

fn compile(translations: Translations, body: fn(&mut Block, &Globals)) -> CompileOutput {
    Mode {
        translations: Some(translations),
        body,
    }
    .compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
    .expect("compiles")
}

fn error_codes(
    translations: Option<Translations>,
    body: fn(&mut Block, &Globals),
) -> Vec<&'static str> {
    let err = Mode { translations, body }
        .compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
        .expect_err("fails");
    err.0.iter().map(Diagnostic::code).collect()
}

/// The `Big Message` line of the output.
fn message(code: &str) -> &str {
    code.lines()
        .find(|l| l.contains("Big Message"))
        .expect("a message")
        .trim()
}

fn french() -> Catalog {
    let mut c = Catalog::default();
    c.merge_po(
        Fr,
        "msgid \"You have {0} money\"\nmsgstr \"Vous avez {0} argent\"\n",
    )
    .expect("parses");
    c
}

const SEP: &str = "\u{EC48}";
const INDEX: &str = "Absolute Value(Index Of Array Value(Global.__overpyTranslationHelper__, String Split(Color(White), Empty Array)))";

#[test]
fn short_string_with_arguments() {
    let t = Translations::new([En, Fr]).with_catalog(french());
    let out = compile(t, |b, g| {
        b.big_message(t!("You have {} money", g.money));
    });
    assert_eq!(
        message(&out.code),
        format!(
            "Big Message(All Players(All Teams), Value In Array(String Split(Custom String(\"ＴＬＥｒｒ{SEP}You have {{0}} money{SEP}Vous avez {{0}} argent\", Global.money), Global.__overpyTranslationHelper__), {INDEX}));"
        )
    );
    assert!(out.code.contains(&format!(
        "Set Global Variable(__overpyTranslationHelper__, String Split(Custom String(\"{SEP}0White0Blanc\"), First Of(Null)));"
    )));
}

#[test]
fn long_string_uses_replace_mode() {
    let out = compile(Translations::new([En, Fr]), |b, _| {
        b.big_message(t!("{} {} {} {}", 1, 2, 3, 4));
    });
    let line = message(&out.code);
    assert!(
        line.contains(
            "(0.00, 1.00, 0.00) (0.00, -1.00, 0.00) (1.00, 0.00, 0.00) (-1.00, 0.00, 0.00)"
        ),
        "{line}"
    );
    assert!(
        line.ends_with(", 1), Update Every Frame(Down), 2), Update Every Frame(Left), 3), Update Every Frame(Right), 4));"),
        "{line}"
    );
    assert!(line.starts_with("Big Message(All Players(All Teams), String Replace(String Replace("));
}

#[test]
fn stored_then_displayed() {
    let out = compile(Translations::new([En, Fr]), |b, g| {
        b.set(g.title, t!("Hello"));
        b.big_message(tr(g.title));
    });
    assert!(out.code.contains(&format!(
        "Set Global Variable(title, String Split(Custom String(\"ＴＬＥｒｒ{SEP}Hello{SEP}Hello\"), Global.__overpyTranslationHelper__));"
    )), "{}", out.code);
    assert_eq!(
        message(&out.code),
        format!("Big Message(All Players(All Teams), Value In Array(Global.title, {INDEX}));")
    );
}

#[test]
fn stored_long_string_is_mapped() {
    let out = compile(Translations::new([En, Fr]), |b, g| {
        b.set(g.title, t!("{} {} {} {}", 1, 2, 3, 4));
    });
    assert!(
        out.code
            .contains("Set Global Variable(title, Mapped Array(String Split("),
        "{}",
        out.code
    );
    assert!(out.code.contains("String Replace(String Replace(String Replace(String Replace(Current Array Element, Update Every Frame(Up), 1)"), "{}", out.code);
}

#[test]
fn player_variable_mode() {
    let t = Translations::new([En, Fr]).with_player_var(PlayerVarMode::Default);
    let out = compile(t, |b, g| {
        b.big_message(t!("You have {} money", g.money));
        b.big_message(tr_spectator(t!("Hello")));
    });
    let lines: Vec<_> = out
        .code
        .lines()
        .filter(|l| l.contains("Big Message"))
        .collect();
    assert!(
        lines[0].ends_with("(Local Player).__languageIndex__));"),
        "{}",
        lines[0]
    );
    assert!(lines[1].contains(INDEX), "{}", lines[1]);
    assert!(
        out.code
            .contains("Set Player Variable(Event Player, __languageIndex__, 1.1);")
    );
}

#[test]
fn no_tl_err_mode_has_no_error_entry_and_counts_from_zero() {
    let t = Translations::new([En, Fr]).with_player_var(PlayerVarMode::NoTlErr);
    let out = compile(t, |b, _| {
        b.big_message(tr_spectator(t!("Hello")));
    });
    let line = message(&out.code);
    assert!(
        line.contains(&format!("Custom String(\"Hello{SEP}Hello\")")),
        "{line}"
    );
    assert!(
        line.ends_with(&format!("Subtract({INDEX}, 1)));")),
        "{line}"
    );
    assert!(out.code.contains("__languageIndex__, 0.1);"));
}

#[test]
fn tr_raw_keeps_the_array_in_a_display_action() {
    let out = compile(Translations::new([En, Fr]), |b, g| {
        b.set(g.title, tr_raw(t!("Hello")).erase().cast::<Str>());
    });
    assert!(
        out.code
            .contains("Set Global Variable(title, String Split(")
    );
}

#[test]
fn translated_strings_must_be_displayed_or_stored() {
    let codes = error_codes(Some(Translations::new([En, Fr])), |b, g| {
        b.set(g.title, ws_fmt!("{}", t!("Hello")));
    });
    assert_eq!(codes, ["translation-placement"]);
    let codes = error_codes(Some(Translations::new([En, Fr])), |b, g| {
        b.wait(t!("Hello").erase().cast::<Num>());
        let _ = g;
    });
    assert_eq!(codes, ["translation-placement"]);
}

#[test]
fn tr_needs_a_reevaluated_argument() {
    let codes = error_codes(Some(Translations::new([En, Fr])), |b, g| {
        b.set(g.title, tr(g.title));
    });
    assert_eq!(codes, ["translation-placement"]);
}

#[test]
fn translations_must_be_configured() {
    let codes = error_codes(None, |b, _| {
        b.big_message(t!("Hello"));
    });
    assert_eq!(codes, ["translations-not-configured"]);
}

#[test]
fn the_separator_is_reserved() {
    let codes = error_codes(Some(Translations::new([En, Fr])), |b, _| {
        b.big_message(t!("a\u{EC48}b"));
    });
    assert_eq!(codes, ["translation-separator"]);
}

#[test]
fn placeholders_must_have_arguments() {
    let codes = error_codes(Some(Translations::new([En, Fr])), |b, _| {
        b.big_message(t!("{0} and {1}", 1));
    });
    assert_eq!(codes, ["translation-placeholder"]);
}

#[test]
fn the_catalog_records_occurrences() {
    let t = Translations::new([En, Fr]);
    let out = compile(t, |b, g| {
        b.big_message(t!("You have {} money", g.money));
        b.big_message(t!("You have {} money", g.money));
        b.big_message(t_ctx!("direction", "Left"));
    });
    let catalog = out.catalog.expect("a catalog");
    let ids: Vec<_> = catalog.entries.iter().map(|e| e.msgid.as_str()).collect();
    assert_eq!(ids, ["You have {0} money", "Left"]);
    let money = &catalog.entries[0];
    assert_eq!(money.occurrences.len(), 2);
    assert!(money.occurrences[0].starts_with("file "), "{money:?}");
    assert!(
        money.occurrences[0].ends_with(", rule \"Show\""),
        "{money:?}"
    );
    assert!(!money.occurrences[0].contains("line"), "{money:?}");
    assert_eq!(catalog.entries[1].msgctxt.as_deref(), Some("direction"));
}

#[test]
fn source_lines_are_opt_in() {
    let t = Translations::new([En, Fr]).source_lines(true);
    let out = compile(t, |b, _| {
        b.big_message(t!("Hello"));
    });
    let entry = &out.catalog.expect("catalog").entries[0];
    assert!(entry.occurrences[0].contains(", line "), "{entry:?}");
}

#[test]
fn unused_entries_survive_without_occurrences() {
    let mut old = french();
    old.entries.push(Entry::new("Old", None));
    let out = compile(Translations::new([En, Fr]).with_catalog(old), |b, g| {
        b.big_message(t!("You have {} money", g.money));
    });
    let catalog = out.catalog.expect("catalog");
    assert_eq!(catalog.entries[0].msgid, "You have {0} money");
    assert_eq!(catalog.entries[0].translations[&Fr], "Vous avez {0} argent");
    assert_eq!(catalog.entries[1].msgid, "Old");
    assert_eq!(catalog.entries[1].occurrences.len(), 0);
    assert!(!catalog.to_po(Fr, false).contains("Old"));
    assert!(catalog.to_po(Fr, true).contains("Unused translation"));
}

#[test]
fn ambiguous_languages_stop_the_compile() {
    use ow_workshop::TranslationLang::{EsEs, EsMx, Zh};
    let codes = error_codes(Some(Translations::new([En, Zh, EsMx, EsEs])), |_, _| {});
    assert_eq!(codes, ["translation-languages"]);
    let _ = Ja;
}

#[test]
fn leading_and_trailing_whitespace_is_kept_around_translations() {
    let mut catalog = Catalog::default();
    catalog
        .merge_po(Fr, "msgid \"Hi\"\nmsgstr \"Salut\"\n")
        .expect("parses");
    let out = compile(Translations::new([En, Fr]).with_catalog(catalog), |b, _| {
        b.big_message(t!("  Hi  "));
    });
    assert!(
        message(&out.code).contains(&format!("\"ＴＬＥｒｒ{SEP}  Hi  {SEP}  Salut  \"")),
        "{}",
        message(&out.code)
    );
}

fn detection_rule(mode: PlayerVarMode, optimize: OptLevel) -> String {
    struct Detecting(PlayerVarMode, OptLevel);
    impl Workshop for Detecting {
        fn settings(&self) -> ow_workshop::Settings {
            ow_workshop::Settings::unchecked()
        }

        type Globals = ();
        type Players = ();

        fn build(&self, ws: &mut Builder, (): &(), (): &()) {
            ws.rule("Show", OngoingGlobal, |r, _| {
                r.actions(|b| b.big_message(t!("Hello")));
            });
        }

        fn options(&self) -> CompileOptions {
            let mut options = CompileOptions::default();
            options.optimize = self.1;
            options
        }

        fn translations(&self) -> Option<Translations> {
            Some(Translations::new([En, Fr]).with_player_var(self.0))
        }
    }
    let code = Detecting(mode, optimize)
        .compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
        .expect("compiles")
        .code;
    let start = code
        .find("rule (\"Translation setup - detect player language\")")
        .expect("the rule");
    let rest = &code[start..];
    rest[..rest.find("\n}\n").expect("end") + 3].to_owned()
}

fn expected_rule(start: &str, last: &str) -> String {
    format!(
        r#"rule ("Translation setup - detect player language") {{
    event {{
        Ongoing - Each Player;
        All;
        All;
    }}
    conditions {{
        Has Spawned(Event Player) == True;
        Is Dummy Bot(Event Player) == False;
        (Event Player).__languageIndex__ == {start};
    }}
    actions {{
        Modify Player Variable(Event Player, __languageIndex__, Append To Array, Facing Direction Of(Event Player));
        Start Facing(Event Player, Direction From Angles(Multiply(10, Index Of Array Value(String Split(Custom String("{SEP}0White0Blanc"), First Of(Null)), String Split(Color(White), Empty Array))), 5), 999999999999, To World, Direction and Turn Rate);
        Wait Until(And(Not(Modulo(Round To Integer(Multiply(Horizontal Facing Angle Of(Event Player), 100), To Nearest), 1000)), Compare(Absolute Value(Subtract(Vertical Facing Angle Of(Event Player), 5)), <, 0.01)), 15);
        Set Player Variable At Index(Event Player, __languageIndex__, 0, Max(1, Multiply(Compare(Absolute Value(Subtract(Vertical Facing Angle Of(Event Player), 5)), <, 0.01), Round To Integer(Divide(Horizontal Facing Angle Of(Event Player), 10), To Nearest))));
        Stop Facing(Event Player);
        Set Facing(Event Player, Last Of((Event Player).__languageIndex__), To World);
        Set Player Variable(Event Player, __languageIndex__, {last});
    }}
}}
"#
    )
}

#[test]
fn detection_rule_in_default_mode() {
    assert_eq!(
        detection_rule(PlayerVarMode::Default, OptLevel::Off),
        expected_rule("1.1", "Value In Array((Event Player).__languageIndex__, 0)")
    );
}

#[test]
fn detection_rule_without_tl_err() {
    assert_eq!(
        detection_rule(PlayerVarMode::NoTlErr, OptLevel::Off),
        expected_rule(
            "0.1",
            "Subtract(Value In Array((Event Player).__languageIndex__, 0), 1)"
        )
    );
}

#[test]
fn detection_rule_survives_the_optimizer() {
    let rule = detection_rule(PlayerVarMode::Default, OptLevel::Speed);
    assert!(
        rule.contains("Start Facing(Event Player, Direction From Angles("),
        "{rule}"
    );
    assert!(rule.contains("__languageIndex__ == 1.1;"), "{rule}");
}

#[test]
fn no_detection_rule_without_the_player_variable() {
    let out = compile(Translations::new([En, Fr]), |b, _| {
        b.big_message(t!("Hello"));
    });
    assert!(!out.code.contains("Translation setup"));
}
