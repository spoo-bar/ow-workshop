//! Rule definitions: what a mod's [`Workshop::rules`](crate::Workshop::rules) returns.

use crate::builder::Builder;
use crate::event::{Ev, EventType};
use crate::ir::Location;
use crate::rule::Rule;

/// A rule that has been described but not yet compiled. Make one with [`rule`] or
/// [`subroutine`](crate::subroutine::subroutine); the order of a mod's rules does not matter.
pub struct RuleDef<'a> {
    build: Box<dyn FnOnce(&mut Builder) + 'a>,
}

impl<'a> RuleDef<'a> {
    pub(crate) fn new(build: impl FnOnce(&mut Builder) + 'a) -> Self {
        Self {
            build: Box::new(build),
        }
    }

    pub(crate) fn run(self, ws: &mut Builder) {
        (self.build)(ws);
    }
}

impl std::fmt::Debug for RuleDef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RuleDef")
    }
}

/// Defines a rule listening to `event`.
#[track_caller]
pub fn rule<'a, E: EventType + 'a>(
    name: impl Into<String> + 'a,
    event: E,
    build: impl FnOnce(&mut Rule<E>, Ev<E>) + 'a,
) -> RuleDef<'a> {
    let loc = Location::caller();
    RuleDef::new(move |ws| ws.record_rule(loc, &name.into(), event, build))
}
