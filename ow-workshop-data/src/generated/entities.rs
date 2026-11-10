//! Named characters usable in workshop strings.
/// Named character constants, e.g. `chars::BLACK_SQUARE`.
pub mod chars {
    ///The `asterism` entity.
    pub const ASTERISM: char = '⁂';
    ///The `black_circle` entity.
    pub const BLACK_CIRCLE: char = '●';
    ///The `black_clubs` entity.
    pub const BLACK_CLUBS: char = '♣';
    ///The `black_diamond` entity.
    pub const BLACK_DIAMOND: char = '◆';
    ///The `black_heart` entity.
    pub const BLACK_HEART: char = '♥';
    ///The `black_phone` entity.
    pub const BLACK_PHONE: char = '☎';
    ///The `black_spades` entity.
    pub const BLACK_SPADES: char = '♠';
    ///The `black_square` entity.
    pub const BLACK_SQUARE: char = '■';
    ///The `black_star` entity.
    pub const BLACK_STAR: char = '★';
    ///The `bottom_left_arrow` entity.
    pub const BOTTOM_LEFT_ARROW: char = '↙';
    ///The `bottom_right_arrow` entity.
    pub const BOTTOM_RIGHT_ARROW: char = '↘';
    ///The `box_with_x` entity.
    pub const BOX_WITH_X: char = '〿';
    ///The `copyright` entity.
    pub const COPYRIGHT: char = '©';
    ///The `diamond_in_diamond` entity.
    pub const DIAMOND_IN_DIAMOND: char = '◈';
    ///The `down_arrow` entity.
    pub const DOWN_ARROW: char = '↓';
    ///The `down_black_triangle` entity.
    pub const DOWN_BLACK_TRIANGLE: char = '▼';
    ///The `down_white_triangle` entity.
    pub const DOWN_WHITE_TRIANGLE: char = '▽';
    ///The `eighth_note` entity.
    pub const EIGHTH_NOTE: char = '♪';
    ///The `female` entity.
    pub const FEMALE: char = '♀';
    ///The `flat_note` entity.
    pub const FLAT_NOTE: char = '♭';
    ///The `fullwidth_block` entity.
    pub const FULLWIDTH_BLOCK: char = '▒';
    ///The `fullwidth_space` entity.
    pub const FULLWIDTH_SPACE: char = '\u{3000}';
    ///The `horizontal_arrow` entity.
    pub const HORIZONTAL_ARROW: char = '↔';
    ///The `horizontal_bar` entity.
    pub const HORIZONTAL_BAR: char = '―';
    ///The `horizontal_double_arrow` entity.
    pub const HORIZONTAL_DOUBLE_ARROW: char = '⇔';
    ///The `industrial_symbol` entity.
    pub const INDUSTRIAL_SYMBOL: char = '〄';
    ///The `infinity` entity.
    pub const INFINITY: char = '∞';
    ///The `java` entity.
    pub const JAVA: char = '♨';
    ///The `left_arrow` entity.
    pub const LEFT_ARROW: char = '←';
    ///The `left_black_triangle` entity.
    pub const LEFT_BLACK_TRIANGLE: char = '◀';
    ///The `left_hand` entity.
    pub const LEFT_HAND: char = '☜';
    ///The `left_white_triangle` entity.
    pub const LEFT_WHITE_TRIANGLE: char = '◁';
    ///The `macron` entity.
    pub const MACRON: char = '¯';
    ///The `male` entity.
    pub const MALE: char = '♂';
    ///The `middle_dot` entity.
    pub const MIDDLE_DOT: char = '·';
    ///The `postal_face` entity.
    pub const POSTAL_FACE: char = '〠';
    ///The `quarter_note` entity.
    pub const QUARTER_NOTE: char = '♩';
    ///The `reference_mark` entity.
    pub const REFERENCE_MARK: char = '※';
    ///The `registered` entity.
    pub const REGISTERED: char = '®';
    ///The `right_arrow` entity.
    pub const RIGHT_ARROW: char = '→';
    ///The `right_black_triangle` entity.
    pub const RIGHT_BLACK_TRIANGLE: char = '▶';
    ///The `right_double_arrow` entity.
    pub const RIGHT_DOUBLE_ARROW: char = '⇒';
    ///The `right_hand` entity.
    pub const RIGHT_HAND: char = '☞';
    ///The `right_white_triangle` entity.
    pub const RIGHT_WHITE_TRIANGLE: char = '▷';
    ///The `sixteenth_double_note` entity.
    pub const SIXTEENTH_DOUBLE_NOTE: char = '♬';
    ///The `square_diagonal_lines` entity.
    pub const SQUARE_DIAGONAL_LINES: char = '▩';
    ///The `square_grid_lines` entity.
    pub const SQUARE_GRID_LINES: char = '▦';
    ///The `square_horizontal_lines` entity.
    pub const SQUARE_HORIZONTAL_LINES: char = '▤';
    ///The `square_in_square` entity.
    pub const SQUARE_IN_SQUARE: char = '▣';
    ///The `square_top_left_bottom_right_lines` entity.
    pub const SQUARE_TOP_LEFT_BOTTOM_RIGHT_LINES: char = '▧';
    ///The `square_top_right_bottom_left_lines` entity.
    pub const SQUARE_TOP_RIGHT_BOTTOM_LEFT_LINES: char = '▨';
    ///The `square_vertical_lines` entity.
    pub const SQUARE_VERTICAL_LINES: char = '▥';
    ///The `top_left_arrow` entity.
    pub const TOP_LEFT_ARROW: char = '↖';
    ///The `top_right_arrow` entity.
    pub const TOP_RIGHT_ARROW: char = '↗';
    ///The `up_arrow` entity.
    pub const UP_ARROW: char = '↑';
    ///The `up_black_triangle` entity.
    pub const UP_BLACK_TRIANGLE: char = '▲';
    ///The `up_white_triangle` entity.
    pub const UP_WHITE_TRIANGLE: char = '△';
    ///The `vertical_arrow` entity.
    pub const VERTICAL_ARROW: char = '↕';
    ///The `white_circle` entity.
    pub const WHITE_CIRCLE: char = '○';
    ///The `white_clubs` entity.
    pub const WHITE_CLUBS: char = '♧';
    ///The `white_diamond` entity.
    pub const WHITE_DIAMOND: char = '◇';
    ///The `white_heart` entity.
    pub const WHITE_HEART: char = '♡';
    ///The `white_phone` entity.
    pub const WHITE_PHONE: char = '☏';
    ///The `white_spades` entity.
    pub const WHITE_SPADES: char = '♤';
    ///The `white_square` entity.
    pub const WHITE_SQUARE: char = '□';
    ///The `white_star` entity.
    pub const WHITE_STAR: char = '☆';
    ///The `zero_width_space` entity.
    pub const ZERO_WIDTH_SPACE: char = '\u{ad}';
    /// `U+00AD`, invisible; breaks words the game censors.
    pub const SOFT_HYPHEN: char = '\u{AD}';
}
/// The character of a string entity such as `black_square`.
pub fn entity(name: &str) -> Option<char> {
    match name {
        "asterism" => Some('⁂'),
        "black_circle" => Some('●'),
        "black_clubs" => Some('♣'),
        "black_diamond" => Some('◆'),
        "black_heart" => Some('♥'),
        "black_phone" => Some('☎'),
        "black_spades" => Some('♠'),
        "black_square" => Some('■'),
        "black_star" => Some('★'),
        "bottom_left_arrow" => Some('↙'),
        "bottom_right_arrow" => Some('↘'),
        "box_with_x" => Some('〿'),
        "copyright" => Some('©'),
        "diamond_in_diamond" => Some('◈'),
        "down_arrow" => Some('↓'),
        "down_black_triangle" => Some('▼'),
        "down_white_triangle" => Some('▽'),
        "eighth_note" => Some('♪'),
        "female" => Some('♀'),
        "flat_note" => Some('♭'),
        "fullwidth_block" => Some('▒'),
        "fullwidth_space" => Some('\u{3000}'),
        "horizontal_arrow" => Some('↔'),
        "horizontal_bar" => Some('―'),
        "horizontal_double_arrow" => Some('⇔'),
        "industrial_symbol" => Some('〄'),
        "infinity" => Some('∞'),
        "java" => Some('♨'),
        "left_arrow" => Some('←'),
        "left_black_triangle" => Some('◀'),
        "left_hand" => Some('☜'),
        "left_white_triangle" => Some('◁'),
        "macron" => Some('¯'),
        "male" => Some('♂'),
        "middle_dot" => Some('·'),
        "postal_face" => Some('〠'),
        "quarter_note" => Some('♩'),
        "reference_mark" => Some('※'),
        "registered" => Some('®'),
        "right_arrow" => Some('→'),
        "right_black_triangle" => Some('▶'),
        "right_double_arrow" => Some('⇒'),
        "right_hand" => Some('☞'),
        "right_white_triangle" => Some('▷'),
        "sixteenth_double_note" => Some('♬'),
        "square_diagonal_lines" => Some('▩'),
        "square_grid_lines" => Some('▦'),
        "square_horizontal_lines" => Some('▤'),
        "square_in_square" => Some('▣'),
        "square_top_left_bottom_right_lines" => Some('▧'),
        "square_top_right_bottom_left_lines" => Some('▨'),
        "square_vertical_lines" => Some('▥'),
        "top_left_arrow" => Some('↖'),
        "top_right_arrow" => Some('↗'),
        "up_arrow" => Some('↑'),
        "up_black_triangle" => Some('▲'),
        "up_white_triangle" => Some('△'),
        "vertical_arrow" => Some('↕'),
        "white_circle" => Some('○'),
        "white_clubs" => Some('♧'),
        "white_diamond" => Some('◇'),
        "white_heart" => Some('♡'),
        "white_phone" => Some('☏'),
        "white_spades" => Some('♤'),
        "white_square" => Some('□'),
        "white_star" => Some('☆'),
        "zero_width_space" => Some('\u{ad}'),
        _ => None,
    }
}
