use super::*;

fn tags_of(text: &str) -> Vec<String> {
    let all = items(&[StrToken::Text(text.to_owned())]);
    find_tags(&all)
        .into_iter()
        .map(|r| {
            all[r]
                .iter()
                .map(|i| if let Item::Ch(c) = i { *c } else { '?' })
                .collect()
        })
        .collect()
}

#[test]
fn recognizes_color_texture_and_closing_tags() {
    assert_eq!(
        tags_of("<fgFF0000FF>Red</fg> <FG #00ff00> <tx 12AB> <TX2DD21> </FG>"),
        [
            "<fgFF0000FF>",
            "</fg>",
            "<FG #00ff00>",
            "<tx 12AB>",
            "<TX2DD21>",
            "</FG>"
        ]
    );
}

#[test]
fn rejects_near_misses() {
    assert_eq!(tags_of("<fg>"), Vec::<String>::new());
    assert_eq!(tags_of("<fgZZ>"), Vec::<String>::new());
    assert_eq!(tags_of("<tx#12>"), Vec::<String>::new());
    assert_eq!(tags_of("<b>bold</b> < fg00>"), Vec::<String>::new());
}

#[test]
fn placeholders_count_as_hex_digits() {
    let tokens = [
        StrToken::Text("<fg".to_owned()),
        StrToken::Arg(0),
        StrToken::Text(">".to_owned()),
    ];
    let found = find_tags(&items(&tokens));
    assert_eq!(found.len(), 1);
    assert_eq!((found[0].start, found[0].end), (0, 5));
}

#[test]
fn segments_mark_tag_text() {
    let tokens = [StrToken::Text("a<fg00>b".to_owned())];
    assert_eq!(
        segments(&tokens, true),
        [vec![
            (false, "a".to_owned()),
            (true, "<fg00>".to_owned()),
            (false, "b".to_owned())
        ]]
    );
    assert_eq!(
        segments(&tokens, false),
        [vec![(false, "a<fg00>b".to_owned())]]
    );
}

#[test]
fn short_texture_ids_are_padded_to_14_digits() {
    assert_eq!(
        tx_fix("tx2DD21>"),
        TxFix::Fixed(8, "txC0000000002DD21>".to_owned())
    );
    assert_eq!(
        tx_fix("tx c12345> rest"),
        TxFix::Fixed(10, "txC00000000c12345>".to_owned())
    );
    assert_eq!(
        tx_fix("tx000>"),
        TxFix::Fixed(6, "txC00000000000000>".to_owned())
    );
}

#[test]
fn full_ids_are_left_alone_and_odd_ones_flagged() {
    assert_eq!(tx_fix("txC0000000002DD21>"), TxFix::None);
    assert_eq!(tx_fix("txC12345678>"), TxFix::Malformed);
    assert_eq!(tx_fix("txC123456789012345>"), TxFix::Malformed);
    assert_eq!(tx_fix("tx{}>"), TxFix::None);
    assert_eq!(tx_fix("fg00>"), TxFix::None);
}
