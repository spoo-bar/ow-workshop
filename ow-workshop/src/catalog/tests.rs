use super::*;
use crate::translations::TranslationLang::{Fr, Ja};

fn occ(n: u32) -> String {
    format!("file main, line {n}, rule \"r\"")
}

fn catalog() -> Catalog {
    let mut c = Catalog::default();
    let e = c.entry_mut("Hello", None);
    e.occurrences.push(occ(1));
    e.translations.insert(Fr, "Bonjour".to_owned());
    let e = c.entry_mut("Left", Some("direction"));
    e.occurrences.push(occ(2));
    c
}

#[test]
fn header_is_written() {
    let po = Catalog::default().to_po(Fr, false);
    assert_eq!(
        po,
        "msgid \"\"\nmsgstr \"\"\n\"Content-Type: text/plain; charset=UTF-8\\n\"\n\"Language: fr\\n\"\n\"X-Generator: ow-workshop\\n\"\n"
    );
}

#[test]
fn entries_are_written_with_occurrences_and_context() {
    let po = catalog().to_po(Fr, false);
    assert!(
        po.contains("\n#. file main, line 1, rule \"r\"\nmsgid \"Hello\"\nmsgstr \"Bonjour\"\n")
    );
    assert!(po.contains(
        "\n#. file main, line 2, rule \"r\"\nmsgctxt \"direction\"\nmsgid \"Left\"\nmsgstr \"\"\n"
    ));
}

#[test]
fn round_trip_is_stable() {
    let mut c = catalog();
    c.entry_mut("Bye", None).comments.push("note".to_owned());
    let first = c.to_po(Fr, true);
    let mut again = Catalog::default();
    again.merge_po(Fr, &first).expect("parses");
    // Occurrences are rebuilt by the compiler, so give them back.
    for e in &mut again.entries {
        if let Some(old) = c.get(&e.msgid, e.msgctxt.as_deref()) {
            e.occurrences.clone_from(&old.occurrences);
        }
    }
    assert_eq!(again.to_po(Fr, true), first);
    assert_eq!(again.translation(Fr, "Hello", None), Some("Bonjour"));
}

#[test]
fn multi_line_strings_round_trip() {
    let po = "msgid \"\"\nmsgstr \"\"\n\"Language: fr\\n\"\n\nmsgid \"\"\n\"one\\n\"\n\"two\"\nmsgstr \"\"\n\"un\\n\"\n\"deux\"\n";
    let mut c = Catalog::default();
    c.merge_po(Fr, po).expect("parses");
    assert_eq!(c.translation(Fr, "one\ntwo", None), Some("un\ndeux"));
    c.entry_mut("one\ntwo", None).occurrences.push(occ(1));
    let out = c.to_po(Fr, false);
    assert!(out.contains("msgid \"\"\n\"one\\n\"\n\"two\"\nmsgstr \"\"\n\"un\\n\"\n\"deux\"\n"));
}

#[test]
fn line_count_mismatch_is_an_error() {
    let po = "msgid \"a\\nb\"\nmsgstr \"c\"\n";
    let err = Catalog::default().merge_po(Fr, po).expect_err("mismatch");
    assert_eq!(err.0[0].code, "po-newlines");
}

#[test]
fn wrong_language_header_is_an_error() {
    let po = "msgid \"\"\nmsgstr \"\"\n\"Language: ja\\n\"\n";
    let err = Catalog::default().merge_po(Fr, po).expect_err("wrong lang");
    assert_eq!(err.0[0].code, "po-language");
    Catalog::default().merge_po(Ja, po).expect("matches");
}

#[test]
fn language_header_is_case_and_dash_insensitive() {
    let po = "msgid \"\"\nmsgstr \"\"\n\"Language: ES-mx\\n\"\n";
    Catalog::default()
        .merge_po(TranslationLang::EsMx, po)
        .expect("matches");
}

#[test]
fn empty_msgstr_is_untranslated() {
    let mut c = Catalog::default();
    c.merge_po(Fr, "msgid \"a\"\nmsgstr \"\"\n")
        .expect("parses");
    assert_eq!(c.translation(Fr, "a", None), None);
    assert!(c.get("a", None).is_some());
}

#[test]
fn comments_flags_and_context_are_read() {
    let po = "# keep me\n#. extracted\n#: ref\n#, hasMultipleOccurrencesNotification\nmsgctxt \"ctx\"\nmsgid \"a\"\nmsgstr \"b\"\n";
    let mut c = Catalog::default();
    c.merge_po(Fr, po).expect("parses");
    let e = c.get("a", Some("ctx")).expect("entry");
    assert_eq!(e.comments, ["keep me"]);
    assert!(e.flag_multi);
    assert_eq!(e.translations[&Fr], "b");
}

#[test]
fn merging_a_second_language_adds_to_the_entry() {
    let mut c = Catalog::default();
    c.merge_po(Fr, "msgid \"a\"\nmsgstr \"b\"\n").expect("fr");
    c.merge_po(Ja, "msgid \"a\"\nmsgstr \"c\"\n").expect("ja");
    assert_eq!(c.entries.len(), 1);
    assert_eq!(c.translation(Ja, "a", None), Some("c"));
    assert_eq!(c.translation(Fr, "a", None), Some("b"));
}

#[test]
fn syntax_errors_are_reported() {
    for po in [
        "bogus\n",
        "\"orphan\"\n",
        "msgid \"a\nmsgstr \"b\"\n",
        "msgid \"a\\q\"\n",
    ] {
        let err = Catalog::default().merge_po(Fr, po).expect_err(po);
        assert_eq!(err.0[0].code, "po-syntax", "{po}");
    }
}

#[test]
fn escapes_round_trip() {
    let mut c = Catalog::default();
    let e = c.entry_mut("say \"hi\"\t\\", None);
    e.occurrences.push(occ(1));
    e.translations.insert(Fr, "dis « \"salut\" »".to_owned());
    let po = c.to_po(Fr, false);
    let mut again = Catalog::default();
    again.merge_po(Fr, &po).expect("parses");
    assert_eq!(again.entries[0].msgid, "say \"hi\"\t\\");
    assert_eq!(again.entries[0].translations[&Fr], "dis « \"salut\" »");
}

#[test]
fn one_occurrence_is_a_single_comment_line() {
    let po = catalog().to_po(Fr, false);
    assert!(!po.contains("occurrences:"));
    assert!(!po.contains(" - file"));
}

#[test]
fn several_occurrences_are_listed() {
    let mut c = Catalog::default();
    let e = c.entry_mut("a", None);
    e.occurrences = vec![occ(1), occ(2)];
    e.translations.insert(Fr, "b".to_owned());
    let po = c.to_po(Fr, false);
    assert!(po.contains(
        "#. 2 occurrences:\n#.  - file main, line 1, rule \"r\"\n#.  - file main, line 2, rule \"r\"\n"
    ));
    assert!(!po.contains("TODO"));
}

#[test]
fn untranslated_multiple_occurrences_get_a_todo_and_flag() {
    let mut c = Catalog::default();
    c.entry_mut("a", None).occurrences = vec![occ(1), occ(2)];
    let po = c.to_po(Fr, false);
    assert!(po.contains("# TODO: check if all occurrences have the same translation\n"));
    assert!(po.contains("#, hasMultipleOccurrencesNotification\n"));
    // Reading it back keeps the note and the flag, and does not duplicate them.
    let mut again = Catalog::default();
    again.merge_po(Fr, &po).expect("parses");
    again.entries[0].occurrences = vec![occ(1), occ(2)];
    let second = again.to_po(Fr, false);
    assert_eq!(second.matches("TODO").count(), 1);
    assert_eq!(
        second.matches("hasMultipleOccurrencesNotification").count(),
        1
    );
}

#[test]
fn context_suppresses_the_todo() {
    let mut c = Catalog::default();
    c.entry_mut("a", Some("ctx")).occurrences = vec![occ(1), occ(2)];
    assert!(!c.to_po(Fr, false).contains("TODO"));
}

#[test]
fn unused_entries_are_dropped_or_marked() {
    let mut c = catalog();
    c.entry_mut("Old", None)
        .translations
        .insert(Fr, "Vieux".to_owned());
    assert!(!c.to_po(Fr, false).contains("Old"));
    let kept = c.to_po(Fr, true);
    assert!(kept.contains("#. Unused translation\nmsgid \"Old\"\nmsgstr \"Vieux\"\n"));
    // Used entries come first even though "Old" was added after them.
    assert!(kept.find("Hello") < kept.find("Old"));
}

#[test]
fn used_entries_sort_before_unused_ones() {
    let mut c = Catalog::default();
    c.entry_mut("unused", None);
    c.entry_mut("used", None).occurrences.push(occ(1));
    let po = c.to_po(Fr, true);
    assert!(po.find("\"used\"") < po.find("\"unused\""));
}

mod text {
    use crate::catalog::text::{number_placeholders, strip_whitespace};

    #[test]
    fn whitespace_is_stripped_and_restored() {
        let s = strip_whitespace("  \u{AD}Hello \n\t world\u{3164} \n");
        assert_eq!(s.key, "Hello\nworld");
        assert_eq!(
            s.apply("Bonjour\nmonde"),
            "  \u{AD}Bonjour \n\t monde\u{3164} \n"
        );
    }

    #[test]
    fn plain_text_is_untouched() {
        let s = strip_whitespace("Hello");
        assert_eq!(s.key, "Hello");
        assert_eq!(s.apply("Salut"), "Salut");
    }

    #[test]
    fn a_translation_with_other_line_count_is_wrapped_whole() {
        let s = strip_whitespace(" a\nb ");
        assert_eq!(s.apply("x"), " x ");
    }

    #[test]
    fn empty_placeholders_are_numbered() {
        assert_eq!(
            number_placeholders("{} has {} of {1}").as_deref(),
            Ok("{0} has {1} of {1}")
        );
        assert_eq!(number_placeholders("{x} {").as_deref(), Ok("{x} {"));
    }

    #[test]
    fn at_most_sixteen_placeholders() {
        assert!(number_placeholders(&"{}".repeat(16)).is_ok());
        assert_eq!(number_placeholders(&"{}".repeat(17)), Err(17));
    }
}
