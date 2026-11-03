use super::*;

fn vars(decls: Vec<VarDecl>) -> Vars {
    Vars { entries: decls }
}

fn explicit(name: &str, index: u8) -> VarDecl {
    VarDecl {
        index: Some(index),
        ..VarDecl::new(name)
    }
}

fn internal(name: &str) -> VarDecl {
    VarDecl {
        internal: true,
        ..VarDecl::new(name)
    }
}

fn codes(d: &Diagnostics) -> Vec<&'static str> {
    d.0.iter().map(|d| d.code).collect()
}

#[test]
fn explicit_and_automatic_indices() {
    let mut v = vars(vec![
        VarDecl::new("first"),
        explicit("pinned", 1),
        VarDecl::new("second"),
        VarDecl::new("third"),
    ]);
    let alloc = v.allocate().unwrap();
    assert_eq!(alloc.indices, [0, 1, 2, 3]);
    assert_eq!(v.entries[2].index, Some(2));
}

#[test]
fn internal_variables_take_the_highest_free_index() {
    let mut v = vars(vec![
        internal("__holygrail__"),
        VarDecl::new("user"),
        internal("__languageIndex__"),
        explicit("top", 127),
    ]);
    assert_eq!(v.allocate().unwrap().indices, [126, 0, 125, 127]);
}

#[test]
fn default_names_keep_their_natural_index() {
    let mut v = vars(vec![
        VarDecl::new("mine"),
        VarDecl::new("B"),
        VarDecl::new("AA"),
        VarDecl::new("DX"),
    ]);
    // `mine` allocates after the defaults claimed 1, 26 and 127.
    assert_eq!(v.allocate().unwrap().indices, [0, 1, 26, 127]);
}

#[test]
fn default_name_with_taken_natural_index_falls_back() {
    let mut v = vars(vec![explicit("x", 1), VarDecl::new("B")]);
    assert_eq!(v.allocate().unwrap().indices, [1, 0]);
}

#[test]
fn default_subroutine_names() {
    let mut s = Subs {
        entries: vec![
            SubDecl::new("mine"),
            SubDecl::new("Sub0"),
            SubDecl::new("Sub5"),
        ],
    };
    assert_eq!(s.allocate().unwrap().indices, [1, 0, 5]);
    assert_eq!(default_sub_index("Sub007"), None);
    assert_eq!(default_sub_index("Sub128"), None);
}

#[test]
fn duplicate_name() {
    let mut v = vars(vec![VarDecl::new("a"), VarDecl::new("a")]);
    assert_eq!(codes(&v.allocate().unwrap_err()), ["duplicate-name"]);
}

#[test]
fn invalid_name() {
    let mut v = vars(vec![
        VarDecl::new("1abc"),
        VarDecl::new("a-b"),
        VarDecl::new(""),
    ]);
    assert_eq!(codes(&v.allocate().unwrap_err()), ["invalid-name"; 3]);
}

#[test]
fn duplicate_index() {
    let mut v = vars(vec![explicit("a", 4), explicit("b", 4)]);
    assert_eq!(codes(&v.allocate().unwrap_err()), ["duplicate-index"]);
}

#[test]
fn index_out_of_range() {
    let mut v = vars(vec![explicit("a", 128)]);
    assert_eq!(codes(&v.allocate().unwrap_err()), ["index-out-of-range"]);
}

#[test]
fn more_than_128() {
    let mut v = vars((0..129).map(|i| VarDecl::new(format!("v{i}"))).collect());
    assert_eq!(codes(&v.allocate().unwrap_err()), ["too-many"]);
    let mut full = vars((0..128).map(|i| VarDecl::new(format!("v{i}"))).collect());
    assert_eq!(full.allocate().unwrap().indices.len(), 128);
}

#[test]
fn censored_names_warn_but_allocate() {
    let mut v = vars(vec![VarDecl::new("MyFuck"), VarDecl::new("fine")]);
    let alloc = v.allocate().unwrap();
    assert_eq!(alloc.indices, [0, 1]);
    assert_eq!(codes(&alloc.warnings), ["censored-var-name"]);
}

#[test]
fn failed_allocation_leaves_entries_untouched() {
    let mut v = vars(vec![VarDecl::new("a"), VarDecl::new("a")]);
    let before = v.clone();
    v.allocate().unwrap_err();
    assert_eq!(v, before);
}

#[test]
fn call_graph() {
    let mut subs = Subs::default();
    let first = subs.declare(SubDecl::new("first"));
    let second = subs.declare(SubDecl::new("second"));
    let third = subs.declare(SubDecl::new("third"));
    subs.entries[0].calls = vec![second];
    subs.entries[1].calls = vec![third, second];
    let graph = subs.call_graph();
    assert_eq!(graph.callees(first), [second]);
    assert_eq!(graph.reachable(first), BTreeSet::from([second, third]));
    assert!(graph.is_recursive(second));
    assert!(!graph.is_recursive(first));
    assert!(!graph.is_recursive(third));
}
