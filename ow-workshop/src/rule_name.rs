//! Rule name prefixes and the template that combines them with rule names.

/// What a rule name template sees.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleMeta {
    /// The rule's own name.
    pub name: String,
    /// The enclosing group's prefix; empty outside groups.
    pub prefix: String,
    /// Whether the rule is a delimiter.
    pub is_delimiter: bool,
    /// Source file of the `rule` call without its extension, e.g. `src/effects`.
    pub module_path: String,
}

/// The default template: `[prefix] name`, or just the name without a prefix or for delimiters.
#[must_use]
pub fn default_rule_name(meta: &RuleMeta) -> String {
    if meta.prefix.is_empty() || meta.name.is_empty() || meta.is_delimiter {
        meta.name.clone()
    } else {
        format!("[{}] {}", meta.prefix, meta.name)
    }
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
    fn default_template_prefixes_grouped_rules() {
        let mut ws = Builder::new();
        ws.rule("Free", OngoingGlobal, |_, _| {});
        ws.group("Effects", |ws| {
            ws.rule("Spawn particles", OngoingGlobal, |_, _| {});
        });
        ws.rule("After", OngoingGlobal, |_, _| {});
        assert_eq!(names(ws), ["Free", "[Effects] Spawn particles", "After"]);
    }

    #[test]
    fn inner_group_replaces_the_outer_prefix() {
        let mut ws = Builder::new();
        ws.group("Outer", |ws| {
            ws.rule("a", OngoingGlobal, |_, _| {});
            ws.group("Inner", |ws| {
                ws.rule("b", OngoingGlobal, |_, _| {});
            });
            ws.rule("c", OngoingGlobal, |_, _| {});
        });
        assert_eq!(names(ws), ["[Outer] a", "[Inner] b", "[Outer] c"]);
    }

    #[test]
    fn delimiters_and_empty_names_are_not_prefixed() {
        let mut ws = Builder::new();
        ws.group("G", |ws| {
            ws.rule("---", OngoingGlobal, |r, _| r.delimiter());
            ws.rule("", OngoingGlobal, |_, _| {});
        });
        assert_eq!(names(ws), ["---", ""]);
    }

    #[test]
    fn subroutines_get_the_prefix_too() {
        let mut ws = Builder::new();
        ws.group("Waves", |ws| {
            ws.subroutine("spawn", |_, _| {});
        });
        assert_eq!(names(ws), ["[Waves] Subroutine spawn"]);
    }

    #[test]
    fn custom_template_sees_the_metadata() {
        fn template(meta: &RuleMeta) -> String {
            format!(
                "{}|{}|{}|{}",
                meta.prefix,
                meta.name,
                meta.is_delimiter,
                meta.module_path.rsplit('/').next().unwrap_or_default()
            )
        }
        let mut ws = Builder::with_template(template);
        ws.group("G", |ws| {
            ws.rule("r", OngoingGlobal, |_, _| {});
        });
        assert_eq!(names(ws), ["G|r|false|rule_name"]);
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
            fn build(&self, ws: &mut Builder, (): &(), (): &()) {
                ws.group("X", |ws| ws.rule("r", OngoingGlobal, |_, _| {}));
            }
            fn options(&self) -> CompileOptions {
                CompileOptions {
                    rule_name_template: |m| m.name.to_uppercase(),
                    ..CompileOptions::default()
                }
            }
        }
        let mut ws = Builder::with_template(Custom.options().rule_name_template);
        Custom.build(&mut ws, &(), &());
        assert_eq!(names(ws), ["R"]);
    }
}
