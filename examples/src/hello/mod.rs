//! A small mod that shows the pieces: variables, a shared rule, a rule type
//! with debug options, a plain-data rule and a disabled rule.

mod greeting;
mod score;

use ow_workshop::{Global, Globals, Hero, Lobby, Mode, PlayerVar, Players, Rule, Workshop, utils};

#[derive(Globals)]
pub struct HelloGlobals {
    #[debug(label = "Round", hud = Top)]
    pub round: Global,
}

#[derive(Players)]
pub struct HelloPlayers {
    #[debug(label = "Score", sort_order = 1)]
    pub score: PlayerVar,
    pub greeted: PlayerVar,
}

#[derive(Default)]
pub struct Hello;

impl Workshop for Hello {
    const NAME: &'static str = "Hello";
    const DESCRIPTION: &'static str = "Made with ow-workshop";

    type Globals = HelloGlobals;
    type Players = HelloPlayers;

    fn lobby() -> Lobby {
        Lobby::new()
            .set("Max Team 1 Players", 5)
            .set("Max Team 2 Players", 5)
    }

    fn modes() -> Vec<Mode> {
        vec![Mode::new("Team Deathmatch").enabled_maps(["Workshop Island"])]
    }

    fn heroes() -> Vec<Hero> {
        Vec::new()
    }

    fn rules(&self, g: &HelloGlobals, p: &HelloPlayers) -> Vec<Box<dyn Rule>> {
        let mut rules: Vec<Box<dyn Rule>> = vec![Box::new(utils::DisableDamageAndHealing)];
        rules.extend(greeting::rules(p));
        rules.extend(score::rules(g, p));
        rules
    }
}
