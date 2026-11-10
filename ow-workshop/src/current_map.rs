//! `Current Map`.

use ow_workshop_data::ValueId;

use crate::expr::Expr;
use crate::support;
use crate::ty::Map;

/// `Current Map`.
///
/// The game reports some maps (Colosseo, Esperança, Samoa, Throne of Anubis) wrongly, so the
/// compiler rewrites comparisons and other uses of this value to tell them apart.
#[track_caller]
#[must_use]
pub fn current_map() -> Expr<Map> {
    support::value(ValueId::GetCurrentMap, Vec::new())
}
