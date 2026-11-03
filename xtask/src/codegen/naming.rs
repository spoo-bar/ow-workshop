//! Rust identifiers for data keys.

/// Symbolic or numeric keys that don't convert mechanically.
const OVERRIDES: [(&str, &str); 8] = [
    ("1", "One"),
    ("2", "Two"),
    ("==", "Eq"),
    ("!=", "Ne"),
    ("<=", "Le"),
    (">=", "Ge"),
    ("<", "Lt"),
    (">", "Gt"),
];

/// `UPPER_SNAKE`, `camelCase` or `__camelCase__` to `PascalCase`.
pub fn pascal(key: &str) -> Result<String, String> {
    if let Some((_, name)) = OVERRIDES.iter().find(|(k, _)| *k == key) {
        return Ok((*name).to_owned());
    }
    let mut out = String::new();
    for part in key.split('_').filter(|p| !p.is_empty()) {
        let mut chars = part.chars();
        let Some(first) = chars.next() else { continue };
        out.extend(first.to_uppercase());
        let rest = chars.as_str();
        // `UPPER` words are lowercased; `camelCase` keeps its inner capitals.
        if part.chars().any(char::is_lowercase) {
            out.push_str(rest);
        } else {
            out.push_str(&rest.to_lowercase());
        }
    }
    let valid = out.chars().next().is_some_and(char::is_alphabetic)
        && out.chars().all(char::is_alphanumeric)
        && out != "Self";
    if valid {
        Ok(out)
    } else {
        Err(format!("`{key}` has no valid Rust variant name (`{out}`)"))
    }
}

/// Maps keys to variant names, rejecting collisions.
pub fn variants<'a>(keys: impl Iterator<Item = &'a str>) -> Result<Vec<(&'a str, String)>, String> {
    let mut out: Vec<(&str, String)> = Vec::new();
    for key in keys {
        let name = pascal(key)?;
        if let Some((other, _)) = out.iter().find(|(_, n)| *n == name) {
            return Err(format!("`{key}` and `{other}` both become `{name}`"));
        }
        out.push((key, name));
    }
    Ok(out)
}
