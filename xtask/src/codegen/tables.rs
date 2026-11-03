//! String entities, textures and font metrics.

use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};

use super::enums::{File, build, enum_def_with, grouped};
use crate::data::Data;

pub fn generate(data: &Data) -> Result<Vec<File>, String> {
    Ok(vec![
        ("entities", entities(data)?),
        ("fonts", fonts(data)?),
        ("textures", textures(data)?),
    ])
}

fn entities(data: &Data) -> Result<TokenStream, String> {
    let mut names = Vec::new();
    let mut chars = Vec::new();
    for (name, code) in &data.entities {
        names.push(name);
        chars.push(char::from_u32(*code).ok_or(format!("entity `{name}`: bad code point"))?);
    }
    Ok(quote! {
        //! Named characters usable in workshop strings.

        /// The character of a string entity such as `black_square`.
        pub fn entity(name: &str) -> Option<char> {
            match name {
                #( #names => Some(#chars), )*
                _ => None,
            }
        }
    })
}

fn textures(data: &Data) -> Result<TokenStream, String> {
    let vars = build(data.textures.keys().map(|k| (k.as_str(), None)))
        .map_err(|e| format!("textures: {e}"))?;
    let def = enum_def_with("Texture", "An inline texture.", &vars, None, true);
    // Textures with the same tag share a match arm.
    let groups = grouped(
        vars.iter()
            .zip(data.textures.values())
            .map(|(v, tag)| (&v.name, tag.as_str())),
    );
    let (names, tags): (Vec<_>, Vec<_>) = groups
        .into_iter()
        .map(|(names, tag)| (quote!( #( Self::#names )|* ), tag))
        .unzip();
    Ok(quote! {
        //! Inline textures.

        #def

        impl Texture {
            /// The tag text that renders the texture.
            pub fn tag(self) -> &'static str {
                match self {
                    #( #names => #tags, )*
                }
            }
        }
    })
}

/// A char literal; non-ASCII characters are escaped so normalization can't alter them.
fn char_lit(c: char) -> TokenStream {
    let text = if c.is_ascii_graphic() || c == ' ' {
        format!("{c:?}")
    } else {
        format!("'\\u{{{:x}}}'", u32::from(c))
    };
    text.parse().unwrap_or_default()
}

fn fonts(data: &Data) -> Result<TokenStream, String> {
    let font = &data.fonts;
    let default = Literal::u32_unsuffixed(font.default);
    // Runs of consecutive code points with the same width, as (first, last, width).
    let mut widths: Vec<(char, u32)> = font
        .widths
        .iter()
        .map(|(k, w)| Ok((k.chars().next().ok_or("empty font key")?, *w)))
        .collect::<Result<_, String>>()?;
    widths.sort_unstable();
    let mut runs: Vec<(char, char, u32)> = Vec::new();
    for (c, w) in widths {
        if w == font.default {
            continue;
        }
        match runs.last_mut() {
            Some((_, last, rw)) if *rw == w && u32::from(*last) + 1 == u32::from(c) => *last = c,
            _ => runs.push((c, c, w)),
        }
    }
    let (starts, ends, run_widths): (Vec<_>, Vec<_>, Vec<_>) = runs
        .iter()
        .map(|(s, e, w)| (char_lit(*s), char_lit(*e), Literal::u32_unsuffixed(*w)))
        .fold((vec![], vec![], vec![]), |mut acc, (s, e, w)| {
            acc.0.push(s);
            acc.1.push(e);
            acc.2.push(w);
            acc
        });
    let count = runs.len();

    let mut spaces: Vec<(u32, char)> = font
        .spaces
        .iter()
        .map(|(w, c)| {
            Ok((
                w.parse().map_err(|_| format!("bad space width `{w}`"))?,
                c.chars().next().ok_or("empty space")?,
            ))
        })
        .collect::<Result<_, String>>()?;
    spaces.sort_unstable();
    let (space_widths, space_chars): (Vec<_>, Vec<_>) = spaces
        .into_iter()
        .map(|(w, c)| (Literal::u32_unsuffixed(w), char_lit(c)))
        .unzip();
    let space_count = space_widths.len();

    let mut arms = Vec::new();
    for (c, m) in &font.lowercase {
        let c = c.chars().next().ok_or("empty lowercase key")?;
        let lower = m.lower.chars().next().ok_or("empty lowercase glyph")?;
        let (w, lw) = (
            Literal::u32_unsuffixed(m.width),
            Literal::u32_unsuffixed(m.lower_width),
        );
        let (x, lx) = (
            Literal::i32_unsuffixed(m.xmin),
            Literal::i32_unsuffixed(m.lower_xmin),
        );
        arms.push(quote! { #c => Glyph { lower: #lower, width: #w, lower_width: #lw, xmin: #x, lower_xmin: #lx }, });
    }
    let glyph = format_ident!("Glyph");
    Ok(quote! {
        //! Metrics of the Blizzard Global font.

        /// Width of characters without an entry.
        pub const DEFAULT_WIDTH: u32 = #default;

        /// Space characters as (width, character), sorted by width.
        pub static SPACES: [(u32, char); #space_count] = [ #( (#space_widths, #space_chars) ),* ];

        /// Metrics of an uppercase letter and its lowercase look-alike.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct #glyph {
            /// The lowercase look-alike character.
            pub lower: char,
            /// Width of the uppercase letter.
            pub width: u32,
            /// Width of the lowercase look-alike.
            pub lower_width: u32,
            /// Left bearing of the uppercase letter.
            pub xmin: i32,
            /// Left bearing of the lowercase look-alike.
            pub lower_xmin: i32,
        }

        /// The width of `c` in the font.
        pub fn char_width(c: char) -> u32 {
            // Runs of equal width as (first, last, width), sorted by first.
            static RUNS: [(char, char, u32); #count] = [ #( (#starts, #ends, #run_widths) ),* ];
            let after = RUNS.partition_point(|&(first, ..)| first <= c);
            after
                .checked_sub(1)
                .map(|i| RUNS[i])
                .filter(|&(_, last, _)| c <= last)
                .map_or(DEFAULT_WIDTH, |(.., width)| width)
        }

        /// Metrics for the cased-text helper, for the letters that have them.
        pub fn lowercase_metrics(c: char) -> Option<#glyph> {
            Some(match c {
                #( #arms )*
                _ => return None,
            })
        }
    })
}
