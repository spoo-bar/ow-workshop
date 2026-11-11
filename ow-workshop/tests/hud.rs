//! HUD helpers compile end to end.

use ow_workshop::prelude::*;

#[derive(GlobalVars)]
struct Globals {
    a: Global<Num>,
}

struct Mode {
    tags: bool,
}

impl Workshop for Mode {
    const NAME: &'static str = "Test";
    const DESCRIPTION: &'static str = "Test mode";

    fn settings(&self) -> ow_workshop::Settings {
        ow_workshop::Settings::unchecked()
    }

    type Globals = Globals;
    type Players = ();

    fn build(&self, ws: &mut Builder, g: &Globals, (): &()) {
        ws.rule("Show", OngoingGlobal, |r, _| {
            r.actions(|b| {
                b.hud_header("Score");
                b.print("Ready");
                debug!(b, g.a.get() + 2);
            });
        });
    }

    fn options(&self) -> CompileOptions {
        let mut options = CompileOptions::default();
        options.tags = self.tags;
        options
    }
}

/// `print` and `debug!` are debug-only, so this is a debug build without the variable panels.
fn compile(tags: bool) -> String {
    let mut options = DebugOptions::default();
    options.hud = false;
    options.player_hud = false;
    Mode { tags }
        .compile_with_debug(Lang::EnUs, Some(DebugMode::On(options)))
        .expect("compiles")
        .code
}

#[test]
fn hud_texts_compile_with_their_arguments() {
    let code = compile(false);
    assert!(
        code.contains(
            "Create HUD Text(All Players(All Teams), Custom String(\"Score\"), Null, Null, Left, 0, Color(White), Null, Null, Visible To Sort Order String and Color, Default Visibility);"
        ),
        "{code}"
    );
    assert!(
        code.contains(
            "-9999, Color(Orange), Null, Null, Visible To and String, Default Visibility);"
        ),
        "{code}"
    );
}

#[test]
fn debug_shows_the_expression_and_its_value() {
    let code = compile(false);
    assert!(code.contains("Add(Global.a, 2)"), "{code}");
    assert_eq!(code.matches("Create HUD Text(").count(), 3, "{code}");
}

#[test]
fn debug_colors_the_text_when_tags_are_on() {
    let code = compile(true);
    assert!(code.contains("__holygrail__"), "{code}");
    assert!(code.to_lowercase().contains("fgd0ff00ff"), "{code}");
}
