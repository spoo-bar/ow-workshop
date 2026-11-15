//! The script text a mod produces under each profile.

use ow_workshop::{
    Action, BasicRule, BasicSubroutine, CompareOp, Event, Global, Globals, Hero, Lobby, Mode,
    PlayerVar, Players, Profile, Rule, Subroutine, Value, Workshop, script, utils,
};

#[derive(Globals)]
struct G {
    #[debug(label = "Round", hud = Top)]
    round: Global,
}

#[derive(Players)]
struct P {
    #[var(name = "SCORE")]
    score: PlayerVar,
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
    player:
        0: SCORE
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
