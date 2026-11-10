use ow_workshop_data::ValueId as V;

use crate::ir::CmpOp::{Eq, Ne};
use crate::ir::{Expr, ExprKind};
use crate::lower::testutil::{boolean, konst, null, num};
use crate::opt::testutil::*;

fn a(items: &[f64]) -> Expr {
    arr(items.iter().map(|n| num(*n)).collect())
}

fn elem() -> Expr {
    Expr::new(ExprKind::ArrayElem, None)
}

fn idx() -> Expr {
    Expr::new(ExprKind::ArrayIndex, None)
}

fn team() -> Expr {
    konst("Team", "ALL")
}

fn players() -> Expr {
    val(V::GetPlayers, vec![team()])
}

fn s(e: &Expr) -> String {
    speed(e)
}

// ---- Value In Array ------------------------------------------------------------------------

#[test]
fn indexing_a_literal_array_picks_the_element() {
    assert_eq!(s(&index(a(&[10.0, 20.0, 30.0]), num(1.0))), "20");
    assert_eq!(
        s(&index(a(&[10.0, 20.0, 30.0]), num(1.6))),
        "30",
        "the index is rounded"
    );
    assert_eq!(s(&index(arr(vec![g(0), g(1)]), num(1.0))), "g1");
}

#[test]
fn out_of_bounds_is_null() {
    assert_eq!(s(&index(a(&[1.0]), num(1.0))), "Null");
    assert_eq!(s(&index(a(&[1.0]), num(-1.0))), "Null");
    assert_eq!(s(&index(g(0), num(-1.0))), "Null");
    assert_eq!(s(&index(arr(vec![]), num(0.0))), "Null");
}

#[test]
fn an_empty_array_keeps_its_index_in_strict_mode() {
    assert_eq!(strict(&index(arr(vec![]), num(0.0))), "[][0]");
    assert_eq!(strict(&index(arr(vec![]), num(-1.0))), "[][-1]");
    assert_eq!(strict(&index(a(&[1.0]), num(3.0))), "Null");
}

#[test]
fn index_zero_is_first_of() {
    assert_eq!(s(&index(g(0), num(0.0))), "FirstOf(g0)");
    assert_eq!(s(&index(g(0), num(1.0))), "g0[1]");
}

#[test]
fn indexing_a_number_warns() {
    let e = index(num(4.0), g(0));
    assert_eq!(s(&e), "4[g0]");
    assert_eq!(warnings(&e), ["array-index-number"]);
}

#[test]
fn index_of_in_an_empty_array() {
    assert_eq!(s(&val(V::Index, vec![arr(vec![]), g(0)])), "-1");
}

#[test]
fn index_of_with_literals_is_decided() {
    let find = |items: &[f64], v: f64| s(&val(V::Index, vec![a(items), num(v)]));
    assert_eq!(find(&[5.0, 6.0, 7.0], 7.0), "2");
    assert_eq!(find(&[5.0, 6.0, 7.0], 5.0), "0");
    assert_eq!(find(&[5.0, 6.0, 7.0], 9.0), "-1");
}

#[test]
fn index_of_stops_at_the_first_match() {
    // `g0` comes first and may equal 7, so the answer is not known; the tail after 7 is dead.
    let e = val(
        V::Index,
        vec![arr(vec![g(0), num(7.0), num(8.0)]), num(7.0)],
    );
    assert_eq!(s(&e), "Index([g0, 7], 7)");
    let unknown = val(V::Index, vec![arr(vec![num(1.0), g(0), num(3.0)]), g(1)]);
    assert_eq!(s(&unknown), "Index([1, g0, 3], g1)");
    let same = val(V::Index, vec![arr(vec![num(1.0), g(1), num(3.0)]), g(1)]);
    assert_eq!(s(&same), "Index([1, g1], g1)");
}

#[test]
fn contains_with_an_equal_element_is_true() {
    assert_eq!(
        s(&val(V::ArrayContains, vec![arr(vec![g(0), g(1)]), g(1)])),
        "true"
    );
    assert_eq!(
        s(&val(V::ArrayContains, vec![a(&[1.0, 2.0, 3.0]), num(3.0)])),
        "true"
    );
}

#[test]
fn contains_drops_literals_that_cannot_match() {
    let contains = |items: Vec<Expr>, v: Expr| s(&val(V::ArrayContains, vec![arr(items), v]));
    assert_eq!(contains(vec![num(1.0), num(2.0)], num(9.0)), "false");
    assert_eq!(contains(vec![], g(0)), "false");
    assert_eq!(
        contains(vec![num(1.0), g(1), num(2.0), g(2)], num(9.0)),
        "ArrayContains([g1, g2], 9)"
    );
}

#[test]
fn contains_with_one_element_left_is_a_comparison() {
    let e = val(V::ArrayContains, vec![arr(vec![num(1.0), g(1)]), num(9.0)]);
    assert_eq!(s(&e), "(9 == g1)");
    let single = val(V::ArrayContains, vec![arr(vec![g(1)]), g(0)]);
    assert_eq!(s(&single), "(g0 == g1)");
    let none = val(V::ArrayContains, vec![arr(vec![g(1), g(2)]), g(0)]);
    assert_eq!(s(&none), "ArrayContains([g1, g2], g0)");
}

// ---- building arrays -----------------------------------------------------------------------

#[test]
fn append_extends_a_literal_array() {
    assert_eq!(s(&val(V::Concat, vec![a(&[1.0]), num(2.0)])), "[1, 2]");
    assert_eq!(
        s(&val(V::Concat, vec![a(&[1.0]), a(&[2.0, 3.0])])),
        "[1, 2, 3]"
    );
    assert_eq!(
        s(&val(V::Concat, vec![a(&[1.0]), g(0)])),
        "Concat([1], g0)",
        "an unknown value might be an array"
    );
}

#[test]
fn remove_filters_a_literal_array() {
    assert_eq!(
        s(&val(V::Exclude, vec![a(&[1.0, 2.0, 1.0]), num(1.0)])),
        "[2]"
    );
    assert_eq!(
        s(&val(V::Exclude, vec![a(&[1.0, 2.0, 3.0]), a(&[1.0, 3.0])])),
        "[2]"
    );
    assert_eq!(
        s(&val(V::Exclude, vec![a(&[1.0, 2.0]), num(9.0)])),
        "[1, 2]"
    );
}

#[test]
fn remove_keeps_the_call_unless_everything_is_literal() {
    let e = val(V::Exclude, vec![arr(vec![g(0), num(1.0), g(1)]), num(1.0)]);
    assert_eq!(s(&e), "Exclude([g0, g1], 1)");
    let unknown = val(V::Exclude, vec![arr(vec![g(0), num(1.0)]), g(1)]);
    assert_eq!(s(&unknown), "Exclude([g0, 1], g1)");
    let same = val(V::Exclude, vec![arr(vec![g(0), g(1)]), arr(vec![g(1)])]);
    assert_eq!(s(&same), "Exclude([g0], [g1])");
}

#[test]
fn slices_fold() {
    let slice = |start: f64, count: f64| {
        s(&val(
            V::Slice,
            vec![a(&[1.0, 2.0, 3.0, 4.0]), num(start), num(count)],
        ))
    };
    assert_eq!(slice(1.0, 2.0), "[2, 3]");
    assert_eq!(slice(2.0, 10.0), "[3, 4]");
    assert_eq!(slice(0.6, 1.0), "[2]", "rounded");
    assert_eq!(
        slice(-1.0, 3.0),
        "[1, 2]",
        "a negative start shrinks the count"
    );
    assert_eq!(slice(0.0, 0.0), "[]");
    assert_eq!(slice(0.0, -2.0), "[]");
    assert_eq!(slice(9.0, 2.0), "[]");
}

#[test]
fn last_and_count_of_literal_arrays() {
    assert_eq!(s(&val(V::Last, vec![arr(vec![g(0), g(1)])])), "g1");
    assert_eq!(s(&val(V::Len, vec![a(&[1.0, 2.0, 3.0])])), "3");
    assert_eq!(s(&val(V::Len, vec![arr(vec![])])), "0");
    assert_eq!(s(&val(V::Len, vec![g(0)])), "Len(g0)");
}

#[test]
fn count_of_players_uses_the_dedicated_value() {
    let t = || team();
    let cases = [
        (
            val(V::GetPlayers, vec![t()]),
            "GetNumberOfPlayers(Team.ALL)",
        ),
        (
            val(V::GetLivingPlayers, vec![t()]),
            "GetNumberOfLivingPlayers(Team.ALL)",
        ),
        (
            val(V::GetDeadPlayers, vec![t()]),
            "GetNumberOfDeadPlayers(Team.ALL)",
        ),
        (
            val(V::GetPlayersOnObjective, vec![t()]),
            "GetNumberOfPlayersOnObjective(Team.ALL)",
        ),
        (
            val(V::GetPlayersOnHero, vec![konst("Hero", "ANA"), t()]),
            "GetNumberOfHeroes(Hero.ANA, Team.ALL)",
        ),
    ];
    for (players, expected) in cases {
        assert_eq!(s(&val(V::Len, vec![players])), expected);
    }
}

// ---- lambdas -------------------------------------------------------------------------------

#[test]
fn filter_without_lambda_vars_becomes_a_conditional() {
    let e = val(V::FilteredArray, vec![g(0), g(1)]);
    assert_eq!(s(&e), "(g1 ? g0 : [])");
    assert_eq!(size(&e), "FilteredArray(g0, g1)", "not in size mode");
}

#[test]
fn filter_not_equal_removes_the_value() {
    let ne = |l, r| cmp(Ne, l, r);
    assert_eq!(
        s(&val(V::FilteredArray, vec![g(0), ne(elem(), g(1))])),
        "Exclude(g0, g1)"
    );
    assert_eq!(
        s(&val(V::FilteredArray, vec![g(0), ne(g(1), elem())])),
        "Exclude(g0, g1)"
    );
    let uses_lambda = ne(elem(), idx());
    assert_eq!(
        s(&val(V::FilteredArray, vec![g(0), uses_lambda])),
        "FilteredArray(g0, (@ != #))"
    );
}

#[test]
fn filtering_all_players_uses_the_dedicated_values() {
    let filter = |cond| s(&val(V::FilteredArray, vec![players(), cond]));
    assert_eq!(
        filter(val(V::IsOnObjective, vec![elem()])),
        "GetPlayersOnObjective(Team.ALL)"
    );
    assert_eq!(
        filter(not(val(V::IsOnObjective, vec![elem()]))),
        "GetPlayersNotOnObjective(Team.ALL)"
    );
    assert_eq!(
        filter(val(V::IsAlive, vec![elem()])),
        "GetLivingPlayers(Team.ALL)"
    );
    assert_eq!(
        filter(val(V::IsDead, vec![elem()])),
        "GetDeadPlayers(Team.ALL)"
    );
    assert_eq!(
        filter(cmp(Eq, val(V::GetHero, vec![elem()]), konst("Hero", "ANA"))),
        "GetPlayersOnHero(Hero.ANA, Team.ALL)"
    );
    assert_eq!(
        filter(not(val(V::IsAlive, vec![elem()]))),
        "GetDeadPlayers(Team.ALL)"
    );
}

#[test]
fn mapping_to_the_element_is_the_array() {
    assert_eq!(s(&val(V::MappedArray, vec![g(0), elem()])), "g0");
    assert_eq!(
        s(&val(V::MappedArray, vec![g(0), idx()])),
        "MappedArray(g0, #)"
    );
}

#[test]
fn is_true_for_all() {
    assert_eq!(s(&val(V::All, vec![arr(vec![]), elem()])), "true");
    assert_eq!(s(&val(V::All, vec![g(0), tru()])), "true");
    assert_eq!(s(&val(V::All, vec![g(0), fals()])), "!Len(g0)");
    assert_eq!(s(&val(V::All, vec![g(0), elem()])), "All(g0, @)");
}

#[test]
fn is_true_for_any() {
    assert_eq!(s(&val(V::Any, vec![arr(vec![]), elem()])), "false");
    assert_eq!(s(&val(V::Any, vec![g(0), fals()])), "false");
    assert_eq!(s(&val(V::Any, vec![g(0), tru()])), "(Len(g0) > 0)");
    assert_eq!(s(&val(V::Any, vec![g(0), elem()])), "Any(g0, @)");
}

#[test]
fn sorting_by_the_negated_index_reverses() {
    let key = mul(num(-1.0), idx());
    assert_eq!(
        s(&val(V::SortedArray, vec![a(&[1.0, 2.0, 3.0]), key])),
        "[3, 2, 1]"
    );
    let other = val(V::SortedArray, vec![a(&[1.0, 2.0]), elem()]);
    assert_eq!(s(&other), "SortedArray([1, 2], @)");
}

#[test]
fn the_dict_fixture_lookup_resolves() {
    // `{ANA: 1, PHARAH: 2}[PHARAH]` lowers to a value array indexed by the key's position.
    let pharah = konst("Hero", "PHARAH");
    let keys = arr(vec![konst("Hero", "ANA"), pharah.clone()]);
    let lookup = index(a(&[1.0, 2.0]), val(V::Index, vec![keys, pharah]));
    assert_eq!(s(&lookup), "2");
    // `Null` is not a literal, so the key's position is not known statically.
    let nulls = index(
        a(&[1.0, 2.0]),
        val(V::Index, vec![arr(vec![null()]), null()]),
    );
    assert_eq!(s(&nulls), "[1, 2][Index([Null], Null)]");
}

#[test]
fn nothing_changes_with_the_optimizer_off() {
    assert_eq!(off(&val(V::Len, vec![a(&[1.0])])), "Len([1])");
}

#[test]
fn all_and_any_of_a_mapped_array_test_the_source_directly() {
    let mapped = val(V::MappedArray, vec![g(0), cmp(Eq, elem(), num(2.0))]);
    assert_eq!(
        s(&val(V::All, vec![mapped.clone(), elem()])),
        "All(g0, (@ == 2))"
    );
    assert_eq!(s(&val(V::Any, vec![mapped, elem()])), "Any(g0, (@ == 2))");
}

#[test]
fn all_false_counts_players_with_the_dedicated_value() {
    let e = val(V::All, vec![players(), boolean(false)]);
    assert_eq!(s(&e), "!GetNumberOfPlayers(Team.ALL)");
}

#[test]
fn excluding_a_literal_nothing_can_equal_drops_it_from_the_list() {
    let e = val(
        V::Exclude,
        vec![a(&[1.0, 2.0, 3.0]), arr(vec![num(1.0), g(0)])],
    );
    assert_eq!(s(&e), "Exclude([2, 3], [g0])");
}
