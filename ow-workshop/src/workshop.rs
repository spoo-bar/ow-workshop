use crate::{Extension, Globals, Hero, Lobby, Mode, Players, Rule, Subroutine};

/// A workshop mod. Implement it once per mod.
pub trait Workshop {
    /// The mode name shown in the custom game browser.
    const NAME: &'static str;
    /// The mode description.
    const DESCRIPTION: &'static str;

    /// The mod's global variables. Use `()` for none.
    type Globals: Globals;
    /// The mod's player variables. Use `()` for none.
    type Players: Players;

    /// Lobby settings.
    fn lobby() -> Lobby;
    /// Game modes and their settings.
    fn modes() -> Vec<Mode>;
    /// Hero settings.
    fn heroes() -> Vec<Hero>;
    /// Workshop extensions the mod needs.
    fn extensions() -> Vec<Extension> {
        Vec::new()
    }

    /// The mod's subroutines.
    fn subroutines(&self, _g: &Self::Globals, _p: &Self::Players) -> Vec<Box<dyn Subroutine>> {
        Vec::new()
    }
    /// The mod's rules, in script order.
    fn rules(&self, g: &Self::Globals, p: &Self::Players) -> Vec<Box<dyn Rule>>;
}
