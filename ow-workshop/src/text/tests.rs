use super::*;
use crate::generated::api::match_time;
use crate::opt::testutil::sh;
use crate::ty::Button;

fn show<T>(e: &Expr<T>) -> String {
    sh(e.ir())
}

#[test]
fn time_is_split_into_hours_minutes_and_seconds() {
    assert_eq!(
        show(&time_to_string(match_time())),
        "\"{0}:{1}:{2}\"(Round((GetMatchTime() / 3600), Rounding.__roundDown__), \
         Substring((((GetMatchTime() % 3600) / 60) + 100), 1, 2), \
         Substring(((GetMatchTime() % 60) + 100), 1, 9999))"
    );
}

#[test]
fn time_of_a_literal_has_no_runtime_inputs() {
    let text = show(&time_to_string(75));
    assert!(
        text.contains("(75 / 3600)") && text.contains("(75 % 60)"),
        "{text}"
    );
}

#[test]
fn button_binding_is_bracketed_unless_it_is_a_modifier() {
    assert_eq!(
        show(&button_to_string(Button::Jump)),
        "MappedArray(InputBindingString(Button.JUMP), \
         Split(\"{0}(0.00, 1.00, 0.00)[{0}](0.00, 1.00, 0.00)[SHIFT](0.00, 1.00, 0.00)[CTRL](0.00, 1.00, 0.00)[ALT]\"(@), \
         FirstOf(VectorUp()))[(((StrLen(\"\\{0}{0}{0}{0}{0}{0}{0}\"(@)) % 7) == 1) && \
         Abs(Index(Split(\"\u{EC47}0\u{EC47}0LSHIFT0LCONTROL0LALT\"(), FirstOf(Null)), @)))])"
    );
}
