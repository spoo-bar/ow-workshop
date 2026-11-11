//! Text tags and textures, end to end.

mod common;

use ow_workshop::prelude::*;

struct Tagged;

impl Workshop for Tagged {
    const NAME: &'static str = "Test";
    const DESCRIPTION: &'static str = "Test mode";

    fn settings(&self) -> ow_workshop::Settings {
        ow_workshop::Settings::unchecked()
    }

    type Globals = ();
    type Players = ();

    fn build(&self, ws: &mut Builder, (): &(), (): &()) {
        ws.rule("Tags", OngoingGlobal, |r, _| {
            r.actions(|b| {
                b.big_message(ws_fmt!("<fgFF0000FF>Red</fg>"));
                b.big_message(ws_fmt!("<tx2DD21>"));
            });
        });
    }

    fn options(&self) -> CompileOptions {
        let mut options = CompileOptions::default();
        options.tags = true;
        options
    }
}

struct Textured;

impl Workshop for Textured {
    const NAME: &'static str = "Test";
    const DESCRIPTION: &'static str = "Test mode";

    fn settings(&self) -> ow_workshop::Settings {
        ow_workshop::Settings::unchecked()
    }

    type Globals = ();
    type Players = ();

    fn build(&self, ws: &mut Builder, (): &(), (): &()) {
        ws.rule("Texture", OngoingGlobal, |r, _| {
            r.actions(|b| b.big_message(ws_fmt!("{} Ultimate", Texture::Ultimate)));
        });
    }
}

#[test]
fn tags_get_the_holygrail_and_the_setup_rule() {
    let code = Tagged
        .compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
        .expect("compiles")
        .code;
    assert!(
        code.contains(r#"Custom String("{0}fgFF0000FF>Red{0}/fg>", Global.__holygrail__)"#),
        "{code}"
    );
    assert!(
        code.contains(r#"Custom String("{0}txC0000000002DD21>", Global.__holygrail__)"#),
        "{code}"
    );
    assert!(code.contains("setup code"), "{code}");
    assert!(code.contains("Create Dummy Bot(All Heroes"), "{code}");
    assert!(code.contains("Destroy All Dummy Bots;"), "{code}");
}

#[test]
fn textures_enable_tags_automatically() {
    let code = Textured
        .compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
        .expect("compiles")
        .code;
    assert!(code.contains("__holygrail__"), "{code}");
    assert!(code.contains("txc"), "{code}");
    assert!(code.contains("setup code"), "{code}");
}

#[test]
fn the_setup_rule_matches_the_listing() {
    let code = Tagged
        .compile_with_debug(Lang::EnUs, Some(DebugMode::Off))
        .expect("compiles")
        .code;
    let start = code.find("rule (\"<").expect("setup rule");
    let end = code[start..].find("\nrule (\"Tags\")").expect("user rule") + start;
    let underscores = "_".repeat(126);
    let (soft, hole, pit) = ('\u{AD}', '\u{303C}', '\u{0840}');
    let expected = format!(
        r#"rule ("<{soft}tx> / <{soft}fg> setup code") {{
    event {{
        Ongoing - Global;
    }}
    actions {{
        Create Dummy Bot(All Heroes, If-Then-Else(Number Of Slots(Team 1), Team 1, If-Then-Else(Number Of Slots(Team 2), Team 2, True)), False, Null, Null);
        Start Forcing Dummy Bot Name(Last Created Entity, Custom String("{underscores}{hole}"));
        Set Global Variable(__holygrail__, String Split(First Of(Last Created Entity), Empty Array));
        Start Forcing Dummy Bot Name(Last Created Entity, Custom String("{underscores}{pit}"));
        Set Global Variable(__holygrail__, String Slice(String Replace(Custom String("{underscores}{hole}"), Global.__holygrail__, First Of(Last Created Entity)), 126, True));
        Destroy All Dummy Bots;
    }}
}}"#
    );
    common::assert_workshop_code_eq(&code[start..end], &expected);
}
