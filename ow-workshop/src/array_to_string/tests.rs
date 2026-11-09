use super::*;
use crate::opt::testutil::sh;

fn numbers() -> Expr<Array<Num>> {
    Expr::from([1, 2].map(Expr::<Num>::from))
}

const ITEM: &str = "(((Len(@) == 1) || ((@ == []) && (@ != Null))) ? \"[{0}]\"(@) : \
                    (Len(@) ? \"[{0}, …+{1}]\"(@, (Len(@) - 1)) : @))";

#[test]
fn expands_to_the_formula_of_the_issue() {
    let describe = "Concat(Concat((Len(@) || ((@ == []) && (@ != Null))), \
                    ((!Len(@) && (@ != [])) ? 3 : (Len(@) * 3))), @)";
    let result = "(FirstOf(@) ? \"[{0}{1}]\"(Replace(\"{0}, {1}, …\u{1}\"(@[2], @[3]), \
                  Substring(\"0, 0, …\u{1}\"(), (-2 + @[1]), (10 - @[1])), []), \
                  ((@[1] > 6) ? \"+{0}\"(((@[1] / 3) - 2)) : [])) : Split(@[2], []))";
    assert_eq!(
        sh(array_to_string(numbers(), 2).ir()),
        format!("MappedArray(MappedArray([MappedArray([1, 2], {ITEM})], {describe}), {result})")
    );
}

#[test]
fn max_len_is_capped_and_at_least_one() {
    let long = sh(array_to_string(numbers(), 5000).ir());
    assert!(
        long.contains(&format!("{}…\u{1}", "0, ".repeat(1000))),
        "1000 slots"
    );
    assert!(!long.contains(&"0, ".repeat(1001)));
    assert!(long.contains("(@[1] > 3000)"));

    let one = sh(array_to_string(numbers(), 0).ir());
    assert!(one.contains("\"0, …\u{1}\"()"));
    assert!(one.contains("(@[1] > 3)"));
}

#[test]
fn the_source_array_may_be_any_array() {
    let heroes: Expr<Array<crate::ty::Player>> = crate::generated::api::all_players();
    let text = sh(array_to_string(heroes, 3).ir());
    assert!(
        text.starts_with("MappedArray(MappedArray([MappedArray(GetPlayers(Team.ALL), "),
        "{text}"
    );
}
