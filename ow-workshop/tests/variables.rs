//! Variable declarations, index allocation and the variable actions.

use ow_workshop::{
    Action, BasicRule, Chase, ChaseRateReeval, ChaseTimeReeval, Event, Global, Globals, Hero,
    Lobby, Mode, ModifyOp, PlayerVar, Players, Profile, Rule, Value, ValueId, Workshop, script,
};

#[allow(non_snake_case)]
#[derive(Globals)]
struct OverchessGlobals {
    sellingActive: Global,
    sellingRan: Global,
    roundEnded: Global,
    sellRemaining: Global,
    roundsCompleted: Global,
    roundWinner: Global,
    lastScore1: Global,
    lastScore2: Global,
    roundPlayer: Global,
    roundIndex: Global,
    #[var(index = 127)]
    __holygrail__: Global,
}

#[allow(non_snake_case)]
#[derive(Players)]
struct OverchessPlayers {
    pointOffset: PlayerVar,
    coins: PlayerVar,
    receivedStartingCoins: PlayerVar,
    members: PlayerVar,
    turnIndex: PlayerVar,
    chooser: PlayerVar,
    messageIndex: PlayerVar,
    options: PlayerVar,
    optionIndex: PlayerVar,
    optionPosition: PlayerVar,
    facingAngle: PlayerVar,
    rowCenter: PlayerVar,
    botSlots: PlayerVar,
    priceTextId: PlayerVar,
    isShopBot: PlayerVar,
    optionBots: PlayerVar,
    optionPositions: PlayerVar,
    purchased: PlayerVar,
    isBuyer: PlayerVar,
    targetBot: PlayerVar,
    targetIndex: PlayerVar,
    targetCost: PlayerVar,
    placedCount: PlayerVar,
    placePosition: PlayerVar,
    placedPositions: PlayerVar,
    markerEffect: PlayerVar,
    confirmTextId: PlayerVar,
    cleanupIndex: PlayerVar,
    placedSlots: PlayerVar,
    isSeller: PlayerVar,
    sellBots: PlayerVar,
    sellSlots: PlayerVar,
    keptHeroes: PlayerVar,
    purchasedLevels: PlayerVar,
    keptLevels: PlayerVar,
    mergeIndex: PlayerVar,
    mateIndex: PlayerVar,
    mergedLevel: PlayerVar,
    mergedHero: PlayerVar,
    botLevel: PlayerVar,
    optionLevels: PlayerVar,
    levelOrder: PlayerVar,
    level2Count: PlayerVar,
    targetLevel: PlayerVar,
}

struct Overchess;

impl Workshop for Overchess {
    const NAME: &'static str = "Overchess";
    const DESCRIPTION: &'static str = "";
    type Globals = OverchessGlobals;
    type Players = OverchessPlayers;

    fn lobby() -> Lobby {
        Lobby::new()
    }
    fn modes() -> Vec<Mode> {
        Vec::new()
    }
    fn heroes() -> Vec<Hero> {
        Vec::new()
    }
    fn rules(&self, _: &OverchessGlobals, _: &OverchessPlayers) -> Vec<Box<dyn Rule>> {
        Vec::new()
    }
}

fn variables_block(script: &str) -> &str {
    let start = script.find("variables {").expect("variables block");
    let end = start + script[start..].find("\n}\n").expect("end of block") + 3;
    &script[start..end]
}

#[test]
fn reproduces_the_overchess_variables_block() {
    let reference = include_str!("../../examples/samples/overchess.owmod");
    let generated = script(&Overchess, Profile::Release);
    assert_eq!(variables_block(&generated), variables_block(reference));
}

#[derive(Globals)]
struct Gaps {
    first: Global,
    #[var(index = 0)]
    pinned_zero: Global,
    #[var(index = 2)]
    pinned_two: Global,
    second: Global,
    third: Global,
    #[var(name = "RENAMED")]
    fourth: Global,
}

#[test]
fn unpinned_variables_fill_the_lowest_free_indices() {
    let gaps = Gaps::declare();
    let indices: Vec<(&str, u8)> = [
        &gaps.first,
        &gaps.pinned_zero,
        &gaps.pinned_two,
        &gaps.second,
        &gaps.third,
        &gaps.fourth,
    ]
    .iter()
    .map(|var| (var.name, var.index))
    .collect();
    assert_eq!(
        indices,
        [
            ("first", 1),
            ("pinned_zero", 0),
            ("pinned_two", 2),
            ("second", 3),
            ("third", 4),
            ("RENAMED", 5),
        ]
    );
}

#[allow(non_snake_case)]
#[derive(Globals)]
struct G {
    arr: Global,
    pos: Global,
    roundsCompleted: Global,
}

#[allow(non_snake_case)]
#[derive(Players)]
struct P {
    coins: PlayerVar,
    botSlots: PlayerVar,
    purchasedLevels: PlayerVar,
    mergeIndex: PlayerVar,
    targetIndex: PlayerVar,
    targetCost: PlayerVar,
    roundPlayer: PlayerVar,
}

struct M(std::cell::RefCell<Option<BasicRule>>);
impl Workshop for M {
    const NAME: &'static str = "t";
    const DESCRIPTION: &'static str = "";
    type Globals = G;
    type Players = P;
    fn lobby() -> Lobby {
        Lobby::new()
    }
    fn modes() -> Vec<Mode> {
        Vec::new()
    }
    fn heroes() -> Vec<Hero> {
        Vec::new()
    }
    fn rules(&self, _: &G, _: &P) -> Vec<Box<dyn Rule>> {
        vec![Box::new(self.0.borrow_mut().take().expect("rules once"))]
    }
}

fn lines(actions: Vec<Action>) -> String {
    let rule = BasicRule {
        actions,
        ..BasicRule::new("t", Event::OngoingGlobal)
    };
    let text = script(&M(std::cell::RefCell::new(Some(rule))), Profile::Release);
    let body = text.split("actions {\n").nth(1).expect("actions block");
    body.lines()
        .take_while(|line| *line != "    }")
        .map(str::trim)
        .collect::<Vec<_>>()
        .join("\n")
}

fn event_player() -> Value {
    Value::event_player()
}

fn entity() -> Value {
    Value::of(ValueId::GetLastCreatedEntity)
}

#[test]
fn set_and_modify_actions_match_reference_lines() {
    let (g, p) = (G::declare(), P::declare());
    let slot_of = Value::call(ValueId::GetSlot, [entity()]);
    let actions = vec![
        g.roundsCompleted.modify(ModifyOp::Add, 1),
        p.purchasedLevels.modify_at(
            event_player(),
            p.mergeIndex.of(event_player()),
            ModifyOp::Add,
            1,
        ),
        p.botSlots
            .modify(event_player(), ModifyOp::AppendToArray, slot_of),
        p.coins.modify(
            event_player(),
            ModifyOp::Subtract,
            p.targetCost.of(event_player()),
        ),
        p.coins
            .modify(p.roundPlayer.of(g.pos.get()), ModifyOp::Add, 1),
        p.botSlots
            .set_at(event_player(), p.targetIndex.of(event_player()), -1),
        g.arr.set_at(2, 5),
        g.arr.modify_at(1, ModifyOp::RemoveFromArrayByValue, 7),
        p.coins.set(event_player(), 0),
    ];
    assert_eq!(
        lines(actions),
        [
            "Modify Global Variable(roundsCompleted, Add, 1);",
            "Modify Player Variable At Index(Event Player, purchasedLevels, (Event Player).mergeIndex, Add, 1);",
            "Modify Player Variable(Event Player, botSlots, Append To Array, Slot Of(Last Created Entity));",
            "Modify Player Variable(Event Player, coins, Subtract, (Event Player).targetCost);",
            "Modify Player Variable((Global.pos).roundPlayer, coins, Add, 1);",
            "Set Player Variable At Index(Event Player, botSlots, (Event Player).targetIndex, -1);",
            "Set Global Variable At Index(arr, 2, 5);",
            "Modify Global Variable At Index(arr, 1, Remove From Array By Value, 7);",
            "Set Player Variable(Event Player, coins, 0);",
        ]
        .join("\n")
    );
}

#[test]
fn chase_actions_name_their_mode_and_reevaluation() {
    let (g, p) = (G::declare(), P::declare());
    let over_time = Chase::OverTime {
        duration: 2.into(),
        reevaluation: ChaseTimeReeval::None,
    };
    let at_rate = Chase::AtRate {
        rate: 5.into(),
        reevaluation: ChaseRateReeval::None,
    };
    let actions = vec![
        g.pos.chase(10, Chase::at_rate(3)),
        g.pos.chase(10, over_time.clone()),
        p.coins.chase(event_player(), 100, at_rate),
        p.coins.chase(event_player(), 100, Chase::over_time(1.5)),
        g.pos.stop_chasing(),
        p.coins.stop_chasing(event_player()),
    ];
    assert_eq!(
        lines(actions),
        [
            "Chase Global Variable At Rate(pos, 10, 3, Destination and Rate);",
            "Chase Global Variable Over Time(pos, 10, 2, None);",
            "Chase Player Variable At Rate(Event Player, coins, 100, 5, None);",
            "Chase Player Variable Over Time(Event Player, coins, 100, 1.5, Destination and Duration);",
            "Stop Chasing Global Variable(pos);",
            "Stop Chasing Player Variable(Event Player, coins);",
        ]
        .join("\n")
    );
}
