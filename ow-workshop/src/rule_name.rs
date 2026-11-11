//! The template that turns a rule's name into the name shown in the game.

/// What a rule name template sees.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleMeta {
    /// The rule's own name.
    pub name: String,
    /// Whether the rule is a delimiter.
    pub is_delimiter: bool,
    /// Source file of the `rule` call without its extension, e.g. `src/effects`.
    pub module_path: String,
}

/// The default template: the rule's own name.
#[must_use]
pub fn default_rule_name(meta: &RuleMeta) -> String {
    meta.name.clone()
}

/// The module path of a source location: its file without the extension.
pub(crate) fn module_path(file: &str) -> String {
    file.strip_suffix(".rs").unwrap_or(file).to_owned()
}

#[cfg(test)]
mod tests {
    use crate::builder::Builder;
    use crate::ir::Program;
    use crate::prelude::*;

    fn names(ws: Builder) -> Vec<String> {
        ws.finish(Program::default())
            .0
            .rules
            .into_iter()
            .map(|r| r.name)
            .collect()
    }

    #[test]
    fn default_template_keeps_the_name() {
        let mut ws = Builder::new();
        ws.rule("Free", OngoingGlobal, |_, _| {});
        ws.rule("", OngoingGlobal, |_, _| {});
        assert_eq!(names(ws), ["Free", ""]);
    }

    #[test]
    fn custom_template_sees_the_metadata() {
        fn template(meta: &RuleMeta) -> String {
            format!(
                "{}|{}|{}",
                meta.name,
                meta.is_delimiter,
                meta.module_path.rsplit('/').next().unwrap_or_default()
            )
        }
        let mut ws = Builder::with_template(template);
        ws.rule("r", OngoingGlobal, |_, _| {});
        assert_eq!(names(ws), ["r|false|rule_name"]);
    }

    #[test]
    fn options_carry_the_template() {
        struct Custom;
        impl Workshop for Custom {
            const NAME: &'static str = "Test";
            const DESCRIPTION: &'static str = "Test mode";

            fn settings(&self) -> crate::settings::Settings {
                crate::settings::Settings::unchecked()
            }

            type Globals = ();
            type Players = ();
            fn rules<'a>(&'a self, (): &'a (), (): &'a ()) -> Vec<RuleDef<'a>> {
                vec![rule("r", OngoingGlobal, |r, _| {
                    r.actions(|b| b.big_message("x"));
                })]
            }
            fn options(&self) -> CompileOptions {
                CompileOptions {
                    rule_name_template: |m| m.name.to_uppercase(),
                    ..CompileOptions::default()
                }
            }
        }
        let code = Custom.compile(Lang::EnUs).expect("compiles").code;
        assert!(code.contains("\"R\""), "{code}");
    }
}
