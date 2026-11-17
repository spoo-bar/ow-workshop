//! The "Commit Duration: Playtime" subroutine and the `subroutines` block of the Limited Ammo script.

use ow_workshop::{
    Action, BasicSubroutine, CompareOp, Global, Globals, Hero, Lobby, Mode, ModifyOp, PlayerVar,
    Players, Profile, Rule, Subroutine, Value, ValueId, Workshop, script,
};
use ow_workshop_golden::{normalize, reference_file};

#[derive(Globals)]
struct G {
    #[var(index = 8, name = "stats_DurationPlaytime")]
    duration_playtime: Global,
    #[var(index = 21, name = "g_TimestampsPlaytime")]
    timestamps_playtime: Global,
}

#[derive(Players)]
struct P {
    #[var(index = 0, name = "SLOT_INDEX")]
    slot_index: PlayerVar,
    #[var(index = 13, name = "heroIndex")]
    hero_index: PlayerVar,
}

const NAMES: [&str; 10] = [
    "resetPlayerState",
    "commitDurationPlaytime",
    "commitDurationAlive",
    "commitDurationOutOfAmmo",
    "updateHeroIndex",
    "cacheAmmoData",
    "refillReserveAmmo",
    "givePlayerAmmo",
    "destroyAmmoPacks",
    "createAmmoPacks",
];

fn commit_duration_playtime(g: &G, p: &P) -> BasicSubroutine {
    let me = Value::event_player;
    let started = Value::call(
        ValueId::ValueInArray,
        [g.timestamps_playtime.get(), p.slot_index.of(me())],
    );
    let elapsed = Value::call(
        ValueId::Subtract,
        [Value::of(ValueId::GetTotalTimeElapsed), started.clone()],
    );
    let hero = p.hero_index.of(me());
    BasicSubroutine {
        actions: vec![Action::when(
            started.compare(CompareOp::Gt, 0),
            [
                Action::when(
                    hero.clone().compare(CompareOp::Ne, -1),
                    [g.duration_playtime.modify_at(hero, ModifyOp::Add, elapsed)],
                ),
                g.timestamps_playtime.set_at(p.slot_index.of(me()), 0),
            ],
        )],
        ..BasicSubroutine::new("commitDurationPlaytime")
            .titled("Subroutine: Commit Duration: Playtime")
    }
}

struct LimitedAmmo;

impl Workshop for LimitedAmmo {
    const NAME: &'static str = "Limited Ammo";
    const DESCRIPTION: &'static str = "Excerpt";
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
    fn subroutines(&self, g: &G, p: &P) -> Vec<Box<dyn Subroutine>> {
        NAMES
            .iter()
            .map(|&name| -> Box<dyn Subroutine> {
                if name == "commitDurationPlaytime" {
                    Box::new(commit_duration_playtime(g, p))
                } else {
                    Box::new(BasicSubroutine::new(name))
                }
            })
            .collect()
    }
    fn rules(&self, _: &G, _: &P) -> Vec<Box<dyn Rule>> {
        Vec::new()
    }
}

const SUBROUTINES: &str = "subroutines {
    0: resetPlayerState
    1: commitDurationPlaytime
    2: commitDurationAlive
    3: commitDurationOutOfAmmo
    4: updateHeroIndex
    5: cacheAmmoData
    6: refillReserveAmmo
    7: givePlayerAmmo
    8: destroyAmmoPacks
    9: createAmmoPacks
}
";

const RULE: &str = r#"rule ("Subroutine: Commit Duration: Playtime") {
    event {
        Subroutine;
        commitDurationPlaytime;
    }
    actions {
        If(Compare(Value In Array(Global.g_TimestampsPlaytime, (Event Player).SLOT_INDEX), >, 0));
            If(Compare((Event Player).heroIndex, !=, -1));
                Modify Global Variable At Index(stats_DurationPlaytime, (Event Player).heroIndex, Add, Subtract(Total Time Elapsed, Value In Array(Global.g_TimestampsPlaytime, (Event Player).SLOT_INDEX)));
            End;
            Set Global Variable At Index(g_TimestampsPlaytime, (Event Player).SLOT_INDEX, 0);
        End;
    }
}
"#;

#[test]
fn reproduces_the_subroutine_rule_and_block() {
    let generated = normalize(&script(&LimitedAmmo, Profile::Release));
    for excerpt in [SUBROUTINES, RULE] {
        assert!(
            generated.contains(&normalize(excerpt)),
            "missing:\n{excerpt}\nin:\n{generated}"
        );
    }
}

/// The excerpts above, checked against the full reference when `OW_REFERENCE_DIR` is set.
#[test]
fn excerpts_match_the_reference() {
    let Some(reference) = reference_file("Limited Ammo/LimitedAmmo.txt") else {
        return;
    };
    let reference = normalize(&reference);
    for excerpt in [SUBROUTINES, RULE] {
        assert!(reference.contains(&normalize(excerpt)), "{excerpt}");
    }
}
