use super::{write_censored_string, write_string};

fn plain(text: &str) -> String {
    let mut out = String::new();
    write_string(&mut out, text).unwrap();
    out
}

fn censored(text: &str) -> String {
    let mut out = String::new();
    write_censored_string(&mut out, text).unwrap();
    out
}

#[test]
fn escapes_quotes_backslashes_and_line_breaks() {
    assert_eq!(plain("say \"hi\"\n"), r#""say \"hi\"\n""#);
    assert_eq!(plain("a\\b\r"), r#""a\\b\r""#);
}

#[test]
fn leaves_everything_else_alone() {
    assert_eq!(plain("tab\t é {0} Admin"), "\"tab\t é {0} Admin\"");
}

#[test]
fn plain_strings_are_not_censored() {
    assert_eq!(plain("Admin panel"), "\"Admin panel\"");
}

#[test]
fn censored_strings_are_escaped_too() {
    assert_eq!(censored("a \"b\"\n"), r#""a \"b\"\n""#);
}

#[test]
fn censored_strings_get_a_soft_hyphen() {
    assert_eq!(censored("Admin panel"), "\"A\u{AD}dmin panel\"");
}
