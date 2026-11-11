//! A set of warnings, used to suppress them.

use crate::warning::Warning;

/// A small bitset over [`Warning`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct WarningSet(u64);

// One bit per warning.
const _: () = assert!(Warning::ALL.len() <= u64::BITS as usize);

impl WarningSet {
    /// The set with no warnings.
    pub const EMPTY: Self = Self(0);

    /// This set plus `warning`.
    #[must_use]
    pub const fn with(self, warning: Warning) -> Self {
        Self(self.0 | Self::bit(warning))
    }

    /// Adds `warning`.
    pub fn insert(&mut self, warning: Warning) {
        *self = self.with(warning);
    }

    /// Whether `warning` is in the set.
    #[must_use]
    pub const fn contains(self, warning: Warning) -> bool {
        self.0 & Self::bit(warning) != 0
    }

    /// The warnings in either set.
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Whether the set has no warnings.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    const fn bit(warning: Warning) -> u64 {
        1 << warning as u32
    }
}

impl From<Warning> for WarningSet {
    fn from(warning: Warning) -> Self {
        Self::EMPTY.with(warning)
    }
}

impl FromIterator<Warning> for WarningSet {
    fn from_iter<I: IntoIterator<Item = Warning>>(iter: I) -> Self {
        iter.into_iter().fold(Self::EMPTY, Self::with)
    }
}

impl Extend<Warning> for WarningSet {
    fn extend<I: IntoIterator<Item = Warning>>(&mut self, iter: I) {
        for warning in iter {
            self.insert(warning);
        }
    }
}
