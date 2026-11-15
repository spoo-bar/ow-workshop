//! Rules and settings that many mods reuse.
//!
//! A shareable rule does not refer to any particular mod. If it needs a
//! variable, it holds the handle as a field.

use crate::{Action, ActionId, Condition, Event, Rule, Value, ValueId, WorkshopDebug};

/// Players deal and take no damage and no healing.
pub struct DisableDamageAndHealing;

impl WorkshopDebug for DisableDamageAndHealing {}

impl Rule for DisableDamageAndHealing {
    fn name(&self) -> String {
        "Disable damage and healing".to_owned()
    }

    fn events(&self) -> Vec<Event> {
        vec![Event::each_player()]
    }

    fn conditions(&self) -> Vec<Condition> {
        vec![Value::call(ValueId::HasSpawned, [Value::event_player()]).is_true()]
    }

    fn actions(&self) -> Vec<Action> {
        [
            ActionId::SetDamageDealt,
            ActionId::SetDamageReceived,
            ActionId::SetHealingDealt,
            ActionId::SetHealingReceived,
        ]
        .into_iter()
        .map(|id| Action::call(id, [Value::event_player(), 0.into()]))
        .collect()
    }
}
