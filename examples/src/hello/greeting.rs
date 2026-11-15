use ow_workshop::{
    Action, ActionId, Condition, Event, PlayerVar, Rule, Value, ValueId, WorkshopDebug,
};

pub fn rules(p: &super::HelloPlayers) -> Vec<Box<dyn Rule>> {
    vec![Box::new(Greet {
        greeted: p.greeted.clone(),
    })]
}

/// Greets each player once after they spawn.
#[derive(WorkshopDebug)]
#[debug(log_to_inspector)]
struct Greet {
    greeted: PlayerVar,
}

impl Rule for Greet {
    fn name(&self) -> String {
        "Greet".to_owned()
    }

    fn events(&self) -> Vec<Event> {
        vec![Event::each_player()]
    }

    fn conditions(&self) -> Vec<Condition> {
        let player = Value::event_player;
        vec![
            Value::call(ValueId::HasSpawned, [player()]).is_true(),
            self.greeted.of(player()).is_false(),
        ]
    }

    fn actions(&self) -> Vec<Action> {
        let player = Value::event_player;
        vec![
            Action::call(
                ActionId::SmallMessage,
                [player(), Value::string("Hello, {0}!", [player()])],
            )
            .commented("Shown once per player"),
            Action::SetPlayerVar {
                player: player(),
                var: self.greeted.clone(),
                value: true.into(),
            },
        ]
    }
}
