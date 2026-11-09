//! Workshop settings: values the lobby settings let a player edit.
//!
//! Every function returns a small builder that converts into an [`Expr`] of the right type, so it
//! can be used wherever a value is expected. The category, name and combo options are plain
//! strings; the compiler rewrites the characters the workshop cannot paste.
//!
//! ```
//! use ow_workshop::prelude::*;
//!
//! let friendly_fire = workshop_setting::toggle("Gameplay", "Friendly fire", false).sort(0);
//! let waves = workshop_setting::int("Gameplay", "Waves", 10).range(1..=50);
//! let speed = workshop_setting::real("Gameplay", "Speed", 1.0).range(0.5..=3.0);
//! let level = workshop_setting::combo("Gameplay", "Difficulty", ["Easy", "Normal", "Hard"], 0);
//! # let _ = (friendly_fire, waves, speed, level);
//! ```

use std::ops::RangeInclusive;

use ow_workshop_data::{Hero, ValueId};

use crate::expr::Expr;
use crate::ir::{self, ExprKind, Location, StrExpr, StrToken};
use crate::support;
use crate::ty::{Bool, IntoExpr, Num};

/// The range a number setting has unless told otherwise.
const LIMIT: f64 = 999_999_999_999.0;

/// A string argument, kept exactly as written; the compiler sanitizes it.
fn text(s: &str) -> ir::Expr {
    ir::Expr::new(
        ExprKind::Str(StrExpr::Custom {
            tokens: vec![StrToken::Text(s.to_owned())],
            args: Vec::new(),
        }),
        None,
    )
}

/// The call of `id`, located where the setting was written.
fn call(id: ValueId, loc: Location, args: Vec<ir::Expr>) -> ir::Expr {
    ir::Expr::new(ExprKind::Value { id, args }, Some(loc))
}

/// The settings every kind shares.
#[derive(Clone, Debug)]
struct Common {
    category: String,
    name: String,
    sort: f64,
    loc: Location,
}

impl Common {
    #[track_caller]
    fn new(category: &str, name: &str) -> Self {
        Self {
            category: category.to_owned(),
            name: name.to_owned(),
            sort: 0.0,
            loc: Location::caller(),
        }
    }

    /// The category, the name, and what the kind adds, then the sort order.
    fn args(&self, id: ValueId, middle: Vec<ir::Expr>) -> ir::Expr {
        let mut args = vec![text(&self.category), text(&self.name)];
        args.extend(middle);
        args.push(support::num(self.sort));
        call(id, self.loc, args)
    }
}

macro_rules! builder_common {
    ($name:ident, $ty:ty) => {
        impl $name {
            /// Where the setting is listed among the others of its category; 0 by default.
            #[must_use]
            pub fn sort(mut self, order: i32) -> Self {
                self.common.sort = f64::from(order);
                self
            }
        }

        impl From<$name> for Expr<$ty> {
            fn from(setting: $name) -> Self {
                Expr::from_ir(setting.build())
            }
        }

        impl IntoExpr<$ty> for $name {
            fn into_expr(self) -> Expr<$ty> {
                self.into()
            }
        }
    };
}

/// A toggle (`Workshop Setting Toggle`).
#[derive(Clone, Debug)]
pub struct Toggle {
    common: Common,
    default: bool,
}

/// A toggle.
#[track_caller]
#[must_use]
pub fn toggle(category: &str, name: &str, default: bool) -> Toggle {
    Toggle {
        common: Common::new(category, name),
        default,
    }
}

impl Toggle {
    fn build(&self) -> ir::Expr {
        self.common.args(
            ValueId::CreateWorkshopSettingBool,
            vec![support::boolean(self.default)],
        )
    }
}

builder_common!(Toggle, Bool);

/// The limits of a number setting.
#[derive(Clone, Copy, Debug)]
struct Limits {
    min: f64,
    max: f64,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            min: -LIMIT,
            max: LIMIT,
        }
    }
}

macro_rules! number_setting {
    ($(#[$doc:meta])* $name:ident, $id:ident, $num:ty) => {
        $(#[$doc])*
        #[derive(Clone, Debug)]
        pub struct $name {
            common: Common,
            default: f64,
            limits: Limits,
        }

        impl $name {
            /// Allows only values in `range`.
            #[must_use]
            pub fn range(mut self, range: RangeInclusive<$num>) -> Self {
                self.limits = Limits {
                    min: range.start().to_f64(),
                    max: range.end().to_f64(),
                };
                self
            }

            /// Allows only values from 0 up.
            #[must_use]
            pub fn unsigned(mut self) -> Self {
                self.limits.min = 0.0;
                self
            }

            /// Allows only values up to 0.
            #[must_use]
            pub fn signed(mut self) -> Self {
                self.limits.max = 0.0;
                self
            }

            fn build(&self) -> ir::Expr {
                self.common.args(
                    ValueId::$id,
                    vec![
                        support::num(self.default),
                        support::num(self.limits.min),
                        support::num(self.limits.max),
                    ],
                )
            }
        }

        builder_common!($name, Num);
    };
}

/// A number that converts to `f64`, so integer and real ranges share one macro.
trait ToF64 {
    fn to_f64(&self) -> f64;
}

impl ToF64 for i32 {
    fn to_f64(&self) -> f64 {
        f64::from(*self)
    }
}

impl ToF64 for f64 {
    fn to_f64(&self) -> f64 {
        *self
    }
}

number_setting!(
    /// An integer (`Workshop Setting Integer`).
    Integer,
    CreateWorkshopSettingInt,
    i32
);
number_setting!(
    /// A real number (`Workshop Setting Real`).
    Real,
    CreateWorkshopSettingFloat,
    f64
);

/// An integer.
#[track_caller]
#[must_use]
pub fn int(category: &str, name: &str, default: i32) -> Integer {
    Integer {
        common: Common::new(category, name),
        default: f64::from(default),
        limits: Limits::default(),
    }
}

/// A real number.
#[track_caller]
#[must_use]
pub fn real(category: &str, name: &str, default: f64) -> Real {
    Real {
        common: Common::new(category, name),
        default,
        limits: Limits::default(),
    }
}

/// A hero (`Workshop Setting Hero`).
#[derive(Clone, Debug)]
pub struct HeroSetting {
    common: Common,
    default: Hero,
}

/// A hero.
#[track_caller]
#[must_use]
pub fn hero(category: &str, name: &str, default: Hero) -> HeroSetting {
    HeroSetting {
        common: Common::new(category, name),
        default,
    }
}

impl HeroSetting {
    fn build(&self) -> ir::Expr {
        self.common.args(
            ValueId::CreateWorkshopSettingHero,
            vec![support::constant("Hero", self.default.key())],
        )
    }
}

builder_common!(HeroSetting, Hero);

/// A combo box of options (`Workshop Setting Combo`).
#[derive(Clone, Debug)]
pub struct Combo {
    common: Common,
    options: Vec<String>,
    default: usize,
}

/// A combo box; `default` is the index of the option that is selected at first.
///
/// With a `default` other than 0 the first option cannot be selected in the lobby settings, which
/// the compiler warns about.
#[track_caller]
#[must_use]
pub fn combo<S: AsRef<str>>(
    category: &str,
    name: &str,
    options: impl IntoIterator<Item = S>,
    default: usize,
) -> Combo {
    Combo {
        common: Common::new(category, name),
        options: options.into_iter().map(|o| o.as_ref().to_owned()).collect(),
        default,
    }
}

impl Combo {
    fn build(&self) -> ir::Expr {
        let options = self.options.iter().map(|o| text(o)).collect();
        // The default is a small index.
        #[allow(clippy::cast_precision_loss)]
        let default = self.default as f64;
        self.common.args(
            ValueId::CreateWorkshopSettingEnum,
            vec![
                support::num(default),
                ir::Expr::new(ExprKind::Array(options), None),
            ],
        )
    }
}

builder_common!(Combo, Num);

#[cfg(test)]
mod tests;
