//! The script text a mod produces under each profile.

use ow_workshop::{
    Action, BasicRule, BasicSubroutine, Chase, ChaseRateReeval, ChaseTimeReeval, CompareOp, Event,
    Global, Globals, Hero, Lobby, Mode, ModifyOp, PlayerVar, Players, Profile, Rule, Subroutine,
    Value, ValueId, Workshop, script, utils,
};

#[derive(Globals)]
struct G {
    #[debug(label = "Round", hud = Top)]
    round: Global,
    #[var(index = 2)]
    pinned: Global,
    arr: Global,
    #[var(index = 127)]
    __holygrail__: Global,
}

#[derive(Players)]
struct P {
    #[var(name = "SCORE")]
    score: PlayerVar,
    coins: PlayerVar,
    slots: PlayerVar,
}

struct Mod;

impl Workshop for Mod {
    const NAME: &'static str = "Test \"mod\"";
    const DESCRIPTION: &'static str = "Line one\nLine two";
    type Globals = G;
    type Players = P;

    fn lobby() -> Lobby {
        Lobby::new()
    }
    fn modes() -> Vec<Mode> {
        vec![Mode::new("Clash").disabled()]
    }
    fn heroes() -> Vec<Hero> {
        vec![Hero::general().hero("Jetpack Cat", "Frenetic Flight Unlimited Fuel", "On")]
    }

    fn subroutines(&self, g: &G, _: &P) -> Vec<Box<dyn Subroutine>> {
        vec![Box::new(BasicSubroutine {
            name: "nextRound".to_owned(),
            actions: vec![Action::SetGlobal {
                var: g.round.clone(),
                value: 2.5.into(),
            }],
            debug: None,
        })]
    }

    fn rules(&self, g: &G, p: &P) -> Vec<Box<dyn Rule>> {
        let score = p.score.of(Value::event_player());
        let branch = Action::If {
            cond: score.compare(CompareOp::Ge, 3),
            then: vec![Action::CallSubroutine("nextRound".to_owned()).commented("advance")],
            otherwise: vec![Action::SetGlobal {
                var: g.round.clone(),
                value: (&g.round).into(),
            }],
        };
        let me = Value::event_player;
        let last = Value::of(ValueId::GetLastCreatedEntity);
        let variables = vec![
            g.round.modify(ModifyOp::Add, 1),
            g.arr.modify_at(1, ModifyOp::RemoveFromArrayByValue, 7),
            g.arr.set_at(2, 5),
            p.coins.modify(me(), ModifyOp::Subtract, p.score.of(me())),
            p.coins.modify_at(me(), p.score.of(me()), ModifyOp::Add, 1),
            p.slots.modify(
                me(),
                ModifyOp::AppendToArray,
                Value::call(ValueId::GetSlot, [last]),
            ),
            p.slots.set_at(me(), p.score.of(me()), -1),
            p.coins.set(p.score.of(g.pinned.get()), 0),
            g.round.chase(10, Chase::at_rate(3)),
            g.round.chase(
                10,
                Chase::OverTime {
                    duration: 2.into(),
                    reevaluation: ChaseTimeReeval::None,
                },
            ),
            p.coins.chase(
                me(),
                100,
                Chase::AtRate {
                    rate: 5.into(),
                    reevaluation: ChaseRateReeval::None,
                },
            ),
            p.coins.chase(me(), 100, Chase::over_time(1.5)),
            g.round.stop_chasing(),
            p.coins.stop_chasing(me()),
        ];
        vec![
            Box::new(utils::DisableDamageAndHealing),
            Box::new(BasicRule {
                actions: vec![branch.clone()],
                ..BasicRule::new("Branch", Event::OngoingGlobal)
            }),
            Box::new(BasicRule {
                enabled: false,
                actions: vec![branch],
                ..BasicRule::new("Off", Event::OngoingGlobal)
            }),
            Box::new(BasicRule {
                actions: variables,
                ..BasicRule::new("Variables", Event::OngoingGlobal)
            }),
        ]
    }
}

const SHARED_RULE: &str = r#"rule ("Disable damage and healing") {
    event {
        Ongoing - Each Player;
        All;
        All;
    }
    conditions {
        Has Spawned(Event Player) == True;
    }
    actions {
        Set Damage Dealt(Event Player, 0);
        Set Damage Received(Event Player, 0);
        Set Healing Dealt(Event Player, 0);
        Set Healing Received(Event Player, 0);
    }
}

"#;

const RELEASE: &str = r#"settings
{
	main
	{
		Description: "Line one\nLine two"
		Mode Name: "Test \"mod\""
	}
	modes
	{
		disabled Clash
		{
		}
	}
	heroes
	{
		General
		{
			Jetpack Cat
			{
				Frenetic Flight Unlimited Fuel: On
			}
		}
	}
}
variables {
    global:
        0: round
        1: arr
        2: pinned
        127: __holygrail__
    player:
        0: SCORE
        1: coins
        2: slots
}
subroutines {
    0: nextRound
}
rule ("Disable damage and healing") {
    event {
        Ongoing - Each Player;
        All;
        All;
    }
    conditions {
        Has Spawned(Event Player) == True;
    }
    actions {
        Set Damage Dealt(Event Player, 0);
        Set Damage Received(Event Player, 0);
        Set Healing Dealt(Event Player, 0);
        Set Healing Received(Event Player, 0);
    }
}

rule ("Branch") {
    event {
        Ongoing - Global;
    }
    actions {
        If(Compare((Event Player).SCORE, >=, 3));
            Call Subroutine(nextRound);
        Else;
            Set Global Variable(round, Global.round);
        End;
    }
}

rule ("Variables") {
    event {
        Ongoing - Global;
    }
    actions {
        Modify Global Variable(round, Add, 1);
        Modify Global Variable At Index(arr, 1, Remove From Array By Value, 7);
        Set Global Variable At Index(arr, 2, 5);
        Modify Player Variable(Event Player, coins, Subtract, (Event Player).SCORE);
        Modify Player Variable At Index(Event Player, coins, (Event Player).SCORE, Add, 1);
        Modify Player Variable(Event Player, slots, Append To Array, Slot Of(Last Created Entity));
        Set Player Variable At Index(Event Player, slots, (Event Player).SCORE, -1);
        Set Player Variable((Global.pinned).SCORE, coins, 0);
        Chase Global Variable At Rate(round, 10, 3, Destination and Rate);
        Chase Global Variable Over Time(round, 10, 2, None);
        Chase Player Variable At Rate(Event Player, coins, 100, 5, None);
        Chase Player Variable Over Time(Event Player, coins, 100, 1.5, Destination and Duration);
        Stop Chasing Global Variable(round);
        Stop Chasing Player Variable(Event Player, coins);
    }
}

rule ("Subroutine: nextRound") {
    event {
        Subroutine;
        nextRound;
    }
    actions {
        Set Global Variable(round, 2.5);
    }
}

"#;

#[test]
fn shared_rule_matches_the_reference_text() {
    assert!(script(&Mod, Profile::Release).contains(SHARED_RULE));
}

#[test]
fn release_script_has_only_what_runs() {
    assert_eq!(script(&Mod, Profile::Release), RELEASE);
}

#[test]
fn debug_script_keeps_disabled_rules_comments_and_variable_panels() {
    let debug = script(&Mod, Profile::Debug);
    assert!(debug.contains("disabled rule (\"Off\") {"));
    assert!(debug.contains("            \"advance\"\n            Call Subroutine(nextRound);"));
    assert!(debug.contains(
        "Create HUD Text(All Players(All Teams), Null, Null, Custom String(\"Round: {0}\", Global.round), Top, 0,"
    ));
    assert!(!debug.contains("Debug: player variables"));
}
