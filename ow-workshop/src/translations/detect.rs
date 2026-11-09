//! The value whose string form tells the viewer's language, and the global that lists its names.

use ow_workshop_data::{Lang, Map, ValueId};

use super::{TranslationLang, Translations};
use crate::ir::{
    ConstRef, Diagnostics, Expr, ExprKind, Program, Severity, StrExpr, StrToken, VarDecl,
};

/// Internal global holding the split of the language names.
pub(crate) const HELPER: &str = "__overpyTranslationHelper__";

/// Private-use character that separates translations; forbidden inside translated strings.
pub(crate) const SEPARATOR: char = '\u{EC48}';

/// A value that stringifies differently in each configured language.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Detection {
    /// `Color(White)`: stringifies without `Update Every Frame`.
    WhiteColor,
    /// `Update Every Frame(Map(Ilios Ruins))`: tells `zh_cn` from `zh_tw`.
    IliosRuins,
    /// `Update Every Frame(Map(Practice Range))`: tells `es_es` from `es_mx` and `zh_cn` from `zh_tw`.
    PracticeRange,
}

impl Detection {
    /// The constant's name as the game shows it in `lang`.
    pub(crate) fn name(self, lang: Lang) -> &'static str {
        match self {
            Self::WhiteColor => ow_workshop_data::Color::White.name(lang),
            Self::IliosRuins => Map::IliosRuins.name(lang),
            Self::PracticeRange => Map::PracticeRange.name(lang),
        }
    }

    /// The workshop expression; the string lowering reads it.
    #[allow(dead_code)]
    pub(crate) fn expr(self) -> Expr {
        let konst = |enum_name, key| Expr::new(ExprKind::Const(ConstRef { enum_name, key }), None);
        let live = |map: Map| {
            Expr::new(
                ExprKind::Value {
                    id: ValueId::UpdateEveryFrame,
                    args: vec![konst("Map", map.key())],
                },
                None,
            )
        };
        match self {
            Self::WhiteColor => konst("Color", "WHITE"),
            Self::IliosRuins => live(Map::IliosRuins),
            Self::PracticeRange => live(Map::PracticeRange),
        }
    }
}

/// The cheapest constant that tells every language in `langs` apart.
pub(crate) fn choose(langs: &[TranslationLang]) -> Result<Detection, &'static str> {
    use TranslationLang::{Es, EsEs, EsMx, Zh, ZhCn, ZhTw};
    let has = |lang| langs.contains(&lang);
    if has(Zh) && has(EsMx) && has(EsEs) {
        return Err("`zh`, `es_mx` and `es_es` together cannot be told apart by any constant");
    }
    let split_zh = has(ZhCn) && has(ZhTw);
    let split_es = has(EsMx) && has(EsEs);
    Ok(if !split_zh && !split_es {
        Detection::WhiteColor
    } else if has(Es) {
        Detection::IliosRuins
    } else {
        Detection::PracticeRange
    })
}

/// `String Split(Custom String("<SEP>0<name_0>0<name_1>0…"), First Of(Null))`.
///
/// `First Of(Null)` stringifies to `"0"`, so the result is `[SEP, name_0, name_1, …]`.
pub(crate) fn helper_init(langs: &[TranslationLang], detection: Detection) -> Expr {
    let mut text = SEPARATOR.to_string();
    for lang in langs {
        text.push('0');
        text.push_str(detection.name(lang.game_lang()));
    }
    let ex = |kind| Expr::new(kind, None);
    let string = ex(ExprKind::Str(StrExpr::Custom {
        tokens: vec![StrToken::Text(text)],
        args: vec![],
    }));
    let zero = ex(ExprKind::Value {
        id: ValueId::FirstOf,
        args: vec![ex(ExprKind::Null)],
    });
    ex(ExprKind::Value {
        id: ValueId::Split,
        args: vec![string, zero],
    })
}

/// Declares the helper global, initialized in the global init rule, when translations are on.
pub(crate) fn declare(program: &mut Program) -> Result<(), Diagnostics> {
    let Some(Translations { langs, .. }) = program.translations.as_ref() else {
        return Ok(());
    };
    if langs.is_empty() {
        return Ok(());
    }
    let detection = choose(langs).map_err(|message| {
        let mut diags = Diagnostics::default();
        diags.push(
            Severity::Error,
            "translation-languages",
            message.to_owned(),
            None,
        );
        diags
    })?;
    let init = helper_init(langs, detection);
    program.globals.declare(VarDecl {
        name: HELPER.to_owned(),
        index: None,
        init: Some(init),
        internal: true,
        flags: crate::ir::VarFlags::default(),
        location: None,
    });
    Ok(())
}
