use ow_workshop::{Action, BasicRule, CompareOp, Event, Rule, Value};

pub fn rules(g: &super::HelloGlobals, p: &super::HelloPlayers) -> Vec<Box<dyn Rule>> {
    let start_round = BasicRule {
        actions: vec![Action::SetGlobal {
            var: g.round.clone(),
            value: 1.into(),
        }],
        ..BasicRule::new("Start first round", Event::OngoingGlobal)
    };
    let reset_score = BasicRule {
        enabled: false,
        actions: vec![Action::when(
            p.score.of(Value::event_player()).compare(CompareOp::Gt, 0),
            [Action::SetPlayerVar {
                player: Value::event_player(),
                var: p.score.clone(),
                value: 0.into(),
            }],
        )],
        ..BasicRule::new("Testing: reset score", Event::each_player())
    };
    vec![Box::new(start_round), Box::new(reset_score)]
}
