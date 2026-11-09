//! Colors that exist only in the editor's palette, as `Custom Color` values.

use crate::expr::Expr;
use crate::generated::api::{RgbArgs, rgb_with};
use crate::ty::Color;

macro_rules! tooling_colors {
    ($($name:ident = [$r:literal, $g:literal, $b:literal, $a:literal]),* $(,)?) => {$(
        #[doc = concat!("`Custom Color(", stringify!($r), ", ", stringify!($g), ", ", stringify!($b), ", ", stringify!($a), ")`.")]
        #[track_caller]
        #[must_use]
        pub fn $name() -> Expr<Color> {
            rgb_with($r, $g, $b, RgbArgs { alpha: $a.into() })
        }
    )*};
}

tooling_colors! {
    light_red = [255, 112, 122, 255],
    light_purple = [210, 127, 243, 255],
    light_violet = [203, 135, 255, 255],
    light_gray = [168, 168, 168, 255],
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opt::testutil::sh;

    #[test]
    fn expand_to_custom_colors() {
        assert_eq!(sh(light_red().ir()), "Rgb(255, 112, 122, 255)");
        assert_eq!(sh(light_purple().ir()), "Rgb(210, 127, 243, 255)");
        assert_eq!(sh(light_violet().ir()), "Rgb(203, 135, 255, 255)");
        assert_eq!(sh(light_gray().ir()), "Rgb(168, 168, 168, 255)");
    }
}
