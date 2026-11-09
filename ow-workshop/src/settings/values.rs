//! Setting values and the ordered map that holds them.

/// The value of one custom game setting.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum SettingValue {
    /// A yes/no, on/off or enabled/disabled setting.
    Bool(bool),
    /// A plain number.
    Number(f64),
    /// A number printed with `%`.
    Percent(f64),
    /// A string.
    Str(String),
    /// The key of one of a combo setting's choices.
    Choice(String),
}

impl From<bool> for SettingValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<f64> for SettingValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<i32> for SettingValue {
    fn from(value: i32) -> Self {
        Self::Number(f64::from(value))
    }
}

impl From<u32> for SettingValue {
    fn from(value: u32) -> Self {
        Self::Number(f64::from(value))
    }
}

impl From<&str> for SettingValue {
    fn from(value: &str) -> Self {
        Self::Str(value.to_owned())
    }
}

impl From<String> for SettingValue {
    fn from(value: String) -> Self {
        Self::Str(value)
    }
}

/// Setting values by key, in the order they were set.
///
/// Keys the schema doesn't know are kept as given.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Values(Vec<(String, SettingValue)>);

impl Values {
    /// An empty map.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets `key`, replacing an earlier value but keeping its place.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<SettingValue>) {
        let (key, value) = (key.into(), value.into());
        match self.0.iter_mut().find(|(k, _)| *k == key) {
            Some((_, slot)) => *slot = value,
            None => self.0.push((key, value)),
        }
    }

    /// [`Values::set`], returning the map.
    #[must_use]
    pub fn with(mut self, key: impl Into<String>, value: impl Into<SettingValue>) -> Self {
        self.set(key, value);
        self
    }

    /// The value of `key`.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&SettingValue> {
        self.0.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    /// Removes `key`, returning its value.
    pub fn remove(&mut self, key: &str) -> Option<SettingValue> {
        let index = self.0.iter().position(|(k, _)| k == key)?;
        Some(self.0.remove(index).1)
    }

    /// The entries, in order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &SettingValue)> {
        self.0.iter().map(|(k, v)| (k.as_str(), v))
    }

    /// How many values are set.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether nothing is set.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
