//! handling of color modes & [RGB].

mod rgb;

pub type Palette = [RGB<u8>];
pub use self::rgb::RGB;

use std::borrow::Cow;
use std::path::Path;
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Eq)]
/// Mode is the color mode the program runs in. Corresponds to [Opt][crate::Opt] `--color`
pub enum Mode {
    /// A single known [RGB] color.
    /// -  `--color="RED"`
    SingleColor(RGB<u8>),

    /// Color dithering to the user-specified bit depth.
    /// - `--color="color"`
    Color,
    /// Grayscale dithering to the user-specified bit depth.
    /// - `-color="bw"`(default)
    BlackAndWhite,
    Palette {
        palette: Cow<'static, Palette>,
        name: Cow<'static, str>,
    },
}

/// parse a palette, specified as 6-digit hexidecimal RGB values (w/ optional 0x prefix) separated by newlines.
/// lines consisting entirely of whitespace or starting with `//` are ignored.
/// /// don't forget to include at least two colors (probably including one of WHITE (0xffffff) or BLACK(0xffffff))
/// ```
/// # use dither::color::parse_palette;
/// # use dither::prelude::*;
/// # use std::collections::HashSet;
/// let input = "
/// // BLACK
/// 0x000000
/// // RED
/// 0xFF0000
/// // GREEN
/// 00ff00
/// ";
/// let want_colors: HashSet<_> = vec![RGB(0, 0, 0), RGB(0xff, 0x00, 0x00), RGB(0x00, 0xff, 0x00)].into_iter().collect();
/// assert_eq!(want_colors,  parse_palette(input).unwrap());
/// ```
pub fn parse_palette<T: std::iter::FromIterator<RGB<u8>>>(s: &str) -> Result<T, Error> {
    let filtered: Vec<&str> = s
        .lines()
        .map(str::trim)
        .filter(|line| {
            !(line.is_empty() || line.starts_with("//") || line.chars().all(char::is_whitespace))
        })
        .collect();
    if filtered.len() <= 2 {
        Err(Error::PaletteTooSmall)
    } else {
        filtered.into_iter().map(RGB::<u8>::from_str).collect()
    }
}
impl Mode {
    pub const CGA_PALETTE: Self = Mode::Palette {
        palette: Cow::Borrowed(cga::ALL),
        name: Cow::Borrowed("CGA"),
    };
}

pub mod cga {
    pub const ALL: &super::Palette = &[
        BLACK,
        BLUE,
        GREEN,
        CYAN,
        RED,
        MAGENTA,
        BROWN,
        LIGHT_GRAY,
        GRAY,
        LIGHT_BLUE,
        LIGHT_GREEN,
        LIGHT_CYAN,
        LIGHT_RED,
        LIGHT_MAGENTA,
        YELLOW,
        WHITE,
    ];
    use crate::prelude::RGB;
    pub const BLACK: RGB<u8> = RGB(0x00, 0x00, 0x00);
    /// the 24-bit rgb representation of [CGA::Blue]
    pub const BLUE: RGB<u8> = RGB(0x00, 0x00, 0xAA);
    /// the 24-bit rgb representation of [CGA::Green]
    pub const GREEN: RGB<u8> = RGB(0x00, 0xAA, 0x00);
    /// the 24-bit rgb representation of [CGA::Cyan]
    pub const CYAN: RGB<u8> = RGB(0x00, 0xAA, 0xAA);
    /// the 24-bit rgb representation of [CGA::Red]
    pub const RED: RGB<u8> = RGB(0xAA, 0x00, 0x00);
    /// the 24-bit rgb representation of [CGA::Magenta]
    pub const MAGENTA: RGB<u8> = RGB(0xAA, 0x00, 0xAA);
    /// the 24-bit rgb representation of [CGA::Brown]
    pub const BROWN: RGB<u8> = RGB(0xAA, 0x55, 0x00);
    /// the 24-bit rgb representation of [CGA::LightGray]
    pub const LIGHT_GRAY: RGB<u8> = RGB(0xAA, 0xAA, 0xAA);
    /// the 24-bit rgb representation of [CGA::Gray]
    pub const GRAY: RGB<u8> = RGB(0x55, 0x55, 0x55);
    /// the 24-bit rgb representation of [CGA::LightBlue]
    pub const LIGHT_BLUE: RGB<u8> = RGB(0x55, 0x55, 0xFF);
    /// the 24-bit rgb representation of [CGA::LightGreen]
    pub const LIGHT_GREEN: RGB<u8> = RGB(0x55, 0xFF, 0x55);
    /// the 24-bit rgb representation of [CGA::LightCyan]
    pub const LIGHT_CYAN: RGB<u8> = RGB(0x55, 0xFF, 0xFF);
    /// the 24-bit rgb representation of [CGA::LightRed]
    pub const LIGHT_RED: RGB<u8> = RGB(0xFF, 0x55, 0x55);
    /// the 24-bit rgb representation of [CGA::LightMagenta]
    pub const LIGHT_MAGENTA: RGB<u8> = RGB(0xFF, 0x55, 0xFF);
    /// the 24-bit rgb representation of [CGA::Yellow]
    pub const YELLOW: RGB<u8> = RGB(0xFF, 0xFF, 0x55);
    /// the 24-bit rgb representation of [CGA::White]
    pub const WHITE: RGB<u8> = RGB(0xFF, 0xFF, 0xFF);
}

impl Default for Mode {
    fn default() -> Self {
        Mode::BlackAndWhite
    }
}

#[derive(Debug, PartialEq, Eq)]
/// An error handling the `--color` input option.
pub enum Error {
    /// Error parsing the palette as a hexidecimal unsigned integer
    RGBParse,
    /// The custom palette only has one (or zero! colors)
    PaletteTooSmall,
    /// An unknown user option.
    UnknownOption(String),

    /// An error accessing a file
    BadFile { path: String, msg: String },
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Mode::Palette { name, .. } => write!(f, "custom_palette_{}", name),
            Mode::Color => write!(f, "color"),
            Mode::SingleColor(color) => write!(f, "single_color_{:x}", color),
            Mode::BlackAndWhite => write!(f, "bw"),
        }
    }
}

impl<'a> FromStr for Mode {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim().to_ascii_uppercase().as_ref() {
            "WHITE" | "BLACK" | "BW" => Mode::BlackAndWhite,
            "C" | "COLOR" => Mode::Color,

            "CGA" => Mode::CGA_PALETTE,

            "BLUE" => Mode::SingleColor(cga::BLUE),
            "GREEN" => Mode::SingleColor(cga::GREEN),
            "CYAN" => Mode::SingleColor(cga::CYAN),
            "RED" => Mode::SingleColor(cga::RED),
            "MAGENTA" => Mode::SingleColor(cga::MAGENTA),
            "BROWN" => Mode::SingleColor(cga::BROWN),
            "LIGHT_GRAY" => Mode::SingleColor(cga::LIGHT_GRAY),
            "GRAY" => Mode::SingleColor(cga::GRAY),
            "LIGHT_BLUE" => Mode::SingleColor(cga::LIGHT_BLUE),
            "LIGHT_GREEN" => Mode::SingleColor(cga::LIGHT_GREEN),
            "LIGHT_CYAN" => Mode::SingleColor(cga::LIGHT_CYAN),
            "LIGHT_RED" => Mode::SingleColor(cga::LIGHT_RED),
            "LIGHT_MAGENTA" => Mode::SingleColor(cga::LIGHT_MAGENTA),
            "YELLOW" => Mode::SingleColor(cga::YELLOW),
            path => match std::fs::read_to_string(path) {
                Ok(contents) => Mode::Palette {
                    palette: parse_palette(&contents)?,
                    name: Cow::Owned(s.to_string()),
                },
                Err(err) => {
                    return Err(Error::BadFile {
                        msg: format!("{}", err),
                        path: s.to_string(),
                    });
                }
            },
        })
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::UnknownOption(opt) => writeln!(f, "unknown color option {}", opt),
            Error::PaletteTooSmall => writeln!(
                f,
                "user-specified palette has 0 or 1 color; must have at least two"
            ),
            Error::RGBParse =>        write!(f, "could not parse to a RGB value: bad format. must be exactly six hexidecimal characters, with optional 0x prefix"),
            Error::BadFile{path, msg} => write!(f, "could not load color palette from file at path \"{}\": {}", path, msg),
        }
    }
}

#[test]
fn test_parse() {
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::{Path, PathBuf};
    const GARBAGE: &str = "ASDASLKJAS";

    let tt: Vec<(&str, Mode)> = vec![
        ("bw", Mode::BlackAndWhite),
        ("c", Mode::Color),
        ("color", Mode::Color),
        ("RED", Mode::SingleColor(cga::RED)),
        ("blue", Mode::SingleColor(cga::BLUE)),
        ("LigHT_CYAN", Mode::SingleColor(cga::LIGHT_CYAN)),
        ("cga", Mode::CGA_PALETTE),
    ];
    for (s, want) in tt {
        assert_eq!(s.parse::<Mode>().unwrap(), want);
    }
    assert!(GARBAGE.parse::<Mode>().is_err());

    let mut temppath = std::env::temp_dir();
    temppath.push("cga.plt");
    let mut file = File::create(&temppath).unwrap();
    write!(
        file,
        "
0x000000
0x0000AA
0x00AA00
0x00AAAA
0xAA00AA
0xAA0000
0xAA5500
0xAAAAAA
0x555555
0x5555FF
0x55FF55
0x55FFFF
0xFF5555
0xFF55FF
0xFFFF55
0xFFFFFF"
    )
    .unwrap();
    let want_palette: HashSet<RGB<u8>> = cga::ALL.iter().cloned().collect();
    if let Mode::Palette {
        palette: got_palette,
        ..
    } = temppath.to_string_lossy().parse::<Mode>().unwrap()
    {
        assert_eq!(want_palette, got_palette.iter().cloned().collect());
    } else {
        panic!("bad")
    }
}
/// create a quantization function from the specified palette, returning the pair
/// `(nearest_neighbor, dist_from_neighbor)`
pub fn quantize_palette(palette: &Palette) -> impl Fn(RGB<f64>) -> (RGB<f64>, RGB<f64>) {
    // the naive implementation is faster than using a k-d tree for small palettes;
    // see https://blog.krum.io/k-d-trees/

    let palette = palette.to_vec();
    move |RGB(r0, g0, b0)| {
        let mut min_abs_err = std::f64::INFINITY;
        let (mut nearest_neighbor, mut dist_from_neighbor) = (RGB(0., 0., 0.), RGB(0., 0., 0.));

        for RGB(r1, g1, b1) in palette.iter().cloned().map(RGB::<f64>::from) {
            let abs_err = f64::abs(r0 - r1) + f64::abs(g0 - g1) + f64::abs(b0 - b1);
            if abs_err < min_abs_err {
                dist_from_neighbor = RGB(r0 - r1, g0 - g1, b0 - b1);
                nearest_neighbor = RGB(r1, g1, b1);
                min_abs_err = abs_err;
            }
        }
        (nearest_neighbor, dist_from_neighbor)
    }
}
