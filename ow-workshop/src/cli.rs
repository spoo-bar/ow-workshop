//! Pure helpers behind [`main!`](crate::main).

use ow_workshop_data::Lang;

use crate::options::{DebugMode, DebugOptions};

/// Reads `--lang <code>` or `--lang=<code>`; English (`en-US`) by default.
#[doc(hidden)]
pub fn lang_from_args(args: impl IntoIterator<Item = String>) -> Result<Lang, String> {
    parse(args, false).map(|(lang, _)| lang)
}

/// [`lang_from_args`] that also reads `--debug` and `--no-debug`, which force the debug mode on
/// (with its default options) or off; `None` leaves the mod's own setting.
#[doc(hidden)]
pub fn args_from(
    args: impl IntoIterator<Item = String>,
) -> Result<(Lang, Option<DebugMode>), String> {
    parse(args, true)
}

fn parse(
    args: impl IntoIterator<Item = String>,
    debug_flags: bool,
) -> Result<(Lang, Option<DebugMode>), String> {
    let mut lang = Lang::EnUs;
    let mut debug = None;
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        let code = if arg == "--lang" {
            args.next()
                .ok_or_else(|| "`--lang` needs a language code".to_owned())?
        } else if let Some(code) = arg.strip_prefix("--lang=") {
            code.to_owned()
        } else if debug_flags && arg == "--debug" {
            debug = Some(DebugMode::On(DebugOptions::default()));
            continue;
        } else if debug_flags && arg == "--no-debug" {
            debug = Some(DebugMode::Off);
            continue;
        } else {
            return Err(format!("unknown argument `{arg}`"));
        };
        lang = code.parse().map_err(|e| format!("{e}"))?;
    }
    Ok((lang, debug))
}

/// Defines a native `main` for a [`Workshop`](crate::Workshop): prints the workshop code to
/// stdout and diagnostics to stderr, and exits with code 1 on errors. `--lang <code>` picks the
/// language (default `en-US`); `--debug` and `--no-debug` force debug output on or off, whatever
/// the build profile says (see [`DebugMode`](crate::DebugMode)). Expands to nothing on `wasm32`.
///
/// ```no_run
/// use ow_workshop::prelude::*;
/// use ow_workshop::{Gamemode, Map};
///
/// struct MyMode;
///
/// impl Workshop for MyMode {
///     fn mode_name(&self) -> &'static str {
///         "My Mode"
///     }
///
///     fn description(&self) -> &'static str {
///         "Made with ow-workshop"
///     }
///
///     type Globals = ();
///     type Players = ();
///
///     fn build(&self, ws: &mut Builder, _: &(), _: &()) {
///         ws.rule("Init", OngoingGlobal, |_, _| {});
///     }
///
///     fn settings(&self) -> Settings {
///         let maps = MapSelection::Enabled(vec![Map::WorkshopChamber.into()]);
///         Settings::default()
///             .with_mode(
///                 ModeKey::Mode(Gamemode::TeamDeathmatch),
///                 ModeSettings::default().maps(maps),
///             )
///     }
/// }
///
/// ow_workshop::main!(MyMode);
/// ```
#[macro_export]
macro_rules! main {
    ($mode:expr) => {
        #[cfg(not(target_arch = "wasm32"))]
        fn main() {
            use $crate::WorkshopExt as _;
            let (lang, debug) = match $crate::args_from(::std::env::args().skip(1)) {
                ::core::result::Result::Ok(parsed) => parsed,
                ::core::result::Result::Err(message) => {
                    ::std::eprintln!("error: {message}");
                    ::std::process::exit(2);
                }
            };
            match ($mode).compile_with_debug(lang, debug) {
                ::core::result::Result::Ok(output) => {
                    if let ::core::option::Option::Some(log) = &output.debug_log {
                        ::std::eprintln!("{log}");
                    }
                    for warning in &output.warnings {
                        ::std::eprintln!("{warning}");
                    }
                    ::std::print!("{}", output.code);
                }
                ::core::result::Result::Err(diagnostics) => {
                    for diagnostic in &diagnostics.0 {
                        ::std::eprintln!("{diagnostic}");
                    }
                    ::std::process::exit(1);
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{Diagnostic, Diagnostics, ErrorKind, Kind, Location, Severity, Warning};

    #[test]
    fn diagnostics_print_like_compilers() {
        let located = Diagnostic {
            severity: Severity::Warning,
            kind: Kind::Warning(Warning::BreakOutsideLoop),
            message: "break outside a loop".to_owned(),
            location: Some(Location {
                file: "src/main.rs",
                line: 3,
                column: 9,
            }),
        };
        assert_eq!(
            located.to_string(),
            "src/main.rs:3:9: warning[break-outside-loop]: break outside a loop"
        );
        let plain = Diagnostic {
            severity: Severity::Error,
            kind: Kind::Error(ErrorKind::EmitFailed),
            message: "m".to_owned(),
            location: None,
        };
        assert_eq!(plain.to_string(), "error[emit-failed]: m");
        assert_eq!(
            Diagnostics(vec![located, plain])
                .to_string()
                .lines()
                .count(),
            2
        );
    }

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| (*s).to_owned()).collect()
    }

    #[test]
    fn english_by_default() {
        assert_eq!(lang_from_args(args(&[])), Ok(Lang::EnUs));
    }

    #[test]
    fn lang_flag_forms() {
        assert_eq!(lang_from_args(args(&["--lang", "de-DE"])), Ok(Lang::DeDe));
        assert_eq!(lang_from_args(args(&["--lang=ja-JP"])), Ok(Lang::JaJp));
    }

    #[test]
    fn debug_flags_force_the_mode() {
        let parse = |list: &[&str]| args_from(args(list)).expect("parses");
        assert_eq!(parse(&[]), (Lang::EnUs, None));
        assert_eq!(
            parse(&["--debug"]).1,
            Some(DebugMode::On(DebugOptions::default()))
        );
        assert_eq!(
            parse(&["--no-debug", "--lang=de-DE"]),
            (Lang::DeDe, Some(DebugMode::Off))
        );
        assert!(lang_from_args(args(&["--debug"])).is_err());
    }

    #[test]
    fn bad_input_is_rejected() {
        assert!(lang_from_args(args(&["--lang"])).is_err());
        assert!(lang_from_args(args(&["--lang", "xx-XX"])).is_err());
        assert!(lang_from_args(args(&["--verbose"])).is_err());
    }
}
