use ow_workshop_data::Lang;

use super::TranslationLang::{self, *};
use super::detect::{Detection, HELPER, SEPARATOR, choose, helper_init};
use super::*;
use crate::ir::{Diagnostics, ExprKind, Program};
use crate::lower::testutil::show;

fn pick(langs: &[TranslationLang]) -> Result<Detection, &'static str> {
    choose(langs)
}

#[test]
fn constant_choice_table() {
    let cases: &[(&[TranslationLang], Result<Detection, ()>)] = &[
        (&[En], Ok(Detection::WhiteColor)),
        (&[En, Fr, De, Ja], Ok(Detection::WhiteColor)),
        (&[En, Es], Ok(Detection::WhiteColor)),
        (&[En, Zh, EsMx], Ok(Detection::WhiteColor)),
        (&[En, Zh, EsEs], Ok(Detection::WhiteColor)),
        (&[En, ZhCn], Ok(Detection::WhiteColor)),
        (&[En, EsMx], Ok(Detection::WhiteColor)),
        (&[En, ZhCn, ZhTw], Ok(Detection::PracticeRange)),
        (&[En, EsMx, EsEs], Ok(Detection::PracticeRange)),
        (&[En, ZhCn, ZhTw, EsMx, EsEs], Ok(Detection::PracticeRange)),
        (&[En, ZhCn, ZhTw, Es], Ok(Detection::IliosRuins)),
        (&[En, EsMx, EsEs, Es], Ok(Detection::IliosRuins)),
        (&[En, Zh, EsMx, EsEs], Err(())),
        (&[Zh, EsMx, EsEs, ZhCn, ZhTw, Es], Err(())),
    ];
    for (langs, want) in cases {
        assert_eq!(pick(langs).map_err(|_| ()), *want, "{langs:?}");
    }
}

#[test]
fn chosen_constant_tells_every_configured_language_apart() {
    for mask in 0_u32..(1 << TranslationLang::ALL.len()) {
        let langs: Vec<_> = TranslationLang::ALL
            .into_iter()
            .enumerate()
            .filter(|(i, _)| mask >> i & 1 == 1)
            .map(|(_, lang)| lang)
            .collect();
        let Ok(detection) = pick(&langs) else {
            continue;
        };
        // The spec's table does not cover a plain `es`/`zh` next to a regional code of the other
        // variant (they share a game language with one of its two codes), so those are skipped.
        let has = |l| langs.contains(&l);
        if (has(Es) && (has(EsEs) || has(EsMx))) || (has(Zh) && (has(ZhCn) || has(ZhTw))) {
            continue;
        }
        let mut seen: Vec<(Lang, &str)> = Vec::new();
        for lang in langs.iter().map(|l| l.game_lang()) {
            let name = detection.name(lang);
            // Two codes for one game language (`zh` and `zh_cn`) share a name by design.
            let clash = seen.iter().find(|(l, n)| *n == name && *l != lang);
            assert!(
                clash.is_none(),
                "{langs:?} clash on {name:?} via {detection:?}"
            );
            seen.push((lang, name));
        }
    }
}

#[test]
fn names_never_contain_the_split_characters() {
    for detection in [
        Detection::WhiteColor,
        Detection::IliosRuins,
        Detection::PracticeRange,
    ] {
        for lang in Lang::ALL {
            let name = detection.name(lang);
            assert!(!name.contains('0') && !name.contains(SEPARATOR), "{name:?}");
        }
    }
}

#[test]
fn game_languages() {
    let expected = [
        (En, Lang::EnUs),
        (Es, Lang::EsMx),
        (Fr, Lang::FrFr),
        (Ja, Lang::JaJp),
        (Pt, Lang::PtBr),
        (Th, Lang::ThTh),
        (Tr, Lang::TrTr),
        (Zh, Lang::ZhCn),
        (De, Lang::DeDe),
        (It, Lang::ItIt),
        (Ko, Lang::KoKr),
        (Pl, Lang::PlPl),
        (Ru, Lang::RuRu),
        (EsEs, Lang::EsEs),
        (EsMx, Lang::EsMx),
        (ZhCn, Lang::ZhCn),
        (ZhTw, Lang::ZhTw),
    ];
    assert_eq!(expected.len(), TranslationLang::ALL.len());
    for (lang, game) in expected {
        assert_eq!(lang.game_lang(), game, "{lang:?}");
    }
}

#[test]
fn codes_are_unique_lowercase() {
    let codes: Vec<_> = TranslationLang::ALL
        .into_iter()
        .map(TranslationLang::code)
        .collect();
    let mut unique = codes.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), codes.len());
    assert!(
        codes
            .iter()
            .all(|c| c.chars().all(|c| c.is_ascii_lowercase() || c == '_'))
    );
}

#[test]
fn detection_expressions() {
    assert_eq!(show(&Detection::WhiteColor.expr()), "Color.WHITE");
    assert_eq!(
        show(&Detection::IliosRuins.expr()),
        "UpdateEveryFrame(Map.iliosRuins)"
    );
    assert_eq!(
        show(&Detection::PracticeRange.expr()),
        "UpdateEveryFrame(Map.practiceRange)"
    );
}

#[test]
fn helper_initializer_lists_names_in_language_order() {
    let init = helper_init(&[En, Fr], Detection::WhiteColor);
    assert_eq!(
        show(&init),
        "Split(\"\u{EC48}0White0Blanc\"(), FirstOf([Null]))".replace("[Null]", "Null")
    );
}

#[test]
fn helper_initializer_uses_the_game_language_names() {
    let init = helper_init(&[Es, EsEs, Zh], Detection::PracticeRange);
    let ExprKind::Value { args, .. } = init.kind() else {
        panic!("not a call");
    };
    assert_eq!(
        show(&args[0]),
        "\"\u{EC48}0Campo de pruebas0Práctica de combate0训练靶场\"()"
    );
}

fn declared(translations: Option<Translations>) -> Result<Program, Diagnostics> {
    let mut program = Program {
        translations,
        ..Program::default()
    };
    declare(&mut program)?;
    Ok(program)
}

#[test]
fn no_helper_without_translations() {
    let globals = |t| declared(t).expect("ok").globals.entries.len();
    assert_eq!(globals(None), 0);
    assert_eq!(globals(Some(Translations::default())), 0);
}

#[test]
fn helper_is_an_internal_initialized_global() {
    let program = declared(Some(Translations::new([En, Fr]))).expect("declares");
    let [helper] = program.globals.entries.as_slice() else {
        panic!("one global");
    };
    assert_eq!(helper.name, HELPER);
    assert!(helper.internal);
    assert!(helper.init.is_some());
}

#[test]
fn ambiguous_languages_are_an_error() {
    let err = declared(Some(Translations::new([En, Zh, EsMx, EsEs]))).expect_err("ambiguous");
    assert_eq!(err.0[0].code(), "translation-languages");
}

#[test]
fn builder_methods_set_fields() {
    let t = Translations::new([Fr, En])
        .with_player_var(PlayerVarMode::NoTlErr)
        .keep_unused(true)
        .source_lines(true);
    assert_eq!(t.source(), Some(Fr));
    assert_eq!(t.player_var, Some(PlayerVarMode::NoTlErr));
    assert!(t.keep_unused && t.source_lines);
}
