//! Math and layout constants.

/// Archimedes' constant.
pub const PI: f64 = std::f64::consts::PI;
/// Euler's number.
pub const E: f64 = std::f64::consts::E;
/// The largest number the workshop keeps; the game clamps larger values to it.
pub const INFINITY: f64 = 999_999_999_999.0;
/// The smallest step of a workshop number.
pub const EPSILON: f64 = 0.000_000_119_209_3;
/// Converts a sphere effect's radius to its visual horizontal radius.
pub const SPHERE_HORIZONTAL_RADIUS_MULT: f64 = 0.984_724;
/// Converts a sphere effect's radius to its visual vertical radius.
pub const SPHERE_VERTICAL_RADIUS_MULT: f64 = 0.998_959;
/// Converts a ring effect's radius to its inner visual radius.
pub const INNER_RING_RADIUS_MULT: f64 = 0.9415;
/// Converts a ring effect's radius to its outer visual radius.
pub const OUTER_RING_RADIUS_MULT: f64 = 0.94965;
/// Converts a ring explosion's radius to its visual radius.
pub const RING_EXPLOSION_RADIUS_MULT: f64 = 0.48;
/// 170 em spaces: pads HUD text to a fixed width.
pub const LOTS_OF_SPACES: &str = "\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}\u{2003}";
/// 125 newlines: pushes HUD text up the screen.
pub const LOTS_OF_NEWLINES: &str = "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_match_the_workshop() {
        assert_eq!(PI, std::f64::consts::PI);
        assert_eq!(E, std::f64::consts::E);
        assert_eq!(INFINITY, 999_999_999_999.0);
        assert_eq!(LOTS_OF_SPACES.chars().count(), 170);
        assert!(LOTS_OF_SPACES.chars().all(|c| c == '\u{2003}'));
        assert_eq!(LOTS_OF_NEWLINES, "\n".repeat(125));
    }
}
