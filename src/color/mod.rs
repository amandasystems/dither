//! handling of color modes & [RGB].

mod rgb;

pub use self::rgb::RGB;
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
    KnownPalette {
        palette: &'static [RGB<u8>],
        name: &'static str,
    },
    CustomPalette(Vec<RGB<u8>>),
}

impl Mode {
    pub const CGA_PALETTE: Self = Mode::KnownPalette {
        palette: &[
            cga::BLACK,
            cga::BLUE,
            cga::GREEN,
            cga::CYAN,
            cga::RED,
            cga::MAGENTA,
            cga::BROWN,
            cga::LIGHT_GRAY,
            cga::GRAY,
            cga::LIGHT_BLUE,
            cga::LIGHT_GREEN,
            cga::LIGHT_CYAN,
            cga::LIGHT_RED,
            cga::LIGHT_MAGENTA,
            cga::YELLOW,
            cga::WHITE,
        ],
        name: "CGA",
    };
}

pub mod cga {
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
    /// An unknown or unimplemented option
    UnknownOption(String),
    /// An input color that's not in the range `0..=0xFF_FF_FF`
    BadPaletteColor(u32),
    /// Error parsing the palette as a hexidecimal unsigned integer
    CouldNotParsePalette(std::num::ParseIntError),
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Mode::KnownPalette { name, .. } => write!(f, "palette: {}", name),
            Mode::CustomPalette(palette) => write!(f, "custom palette: {:?}", palette),
            Mode::Color => write!(f, "color"),
            Mode::SingleColor(color) => write!(f, "single_color_{:x}", color),
            Mode::BlackAndWhite => write!(f, "bw"),
        }
    }
}

impl<'a> FromStr for Mode {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_uppercase().as_ref() {
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

            _ => return Err(Error::UnknownOption(s.to_string())),
        })
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::UnknownOption(o) => writeln!(
                f,
                "unknown color option 
            \"{}\"",
                o
            ),
            Error::BadPaletteColor(n) => writeln!(
                f,
                "palette colors must be between 0x00 and 0xffffff, but had 0x{:x}",
                n
            ),
            Error::CouldNotParsePalette(err) => {
                writeln!(f, "could not parse specified palette: {}", err)
            }
        }
    }
}
impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::CouldNotParsePalette(err)
    }
}
#[test]
fn test_parse() {
    const GARBAGE: &str = "alksdalksdsj";
    let tt: Vec<(&str, Result<Mode, Error>)> = vec![
        ("bw", Ok(Mode::BlackAndWhite)),
        ("c", Ok(Mode::Color)),
        ("color", Ok(Mode::Color)),
        ("RED", Ok(Mode::SingleColor(cga::RED))),
        ("blue", Ok(Mode::SingleColor(cga::BLUE))),
        ("LigHT_CYAN", Ok(Mode::SingleColor(cga::LIGHT_CYAN))),
        ("cga", Ok(Mode::CGA_PALETTE)),
        (GARBAGE, Err(Error::UnknownOption(GARBAGE.to_string()))),
        // (
        //     "0x1ffffff 0x123129",
        //     Err(Error::BadPaletteColor(0x1_ff_ff_ff)),
        // ),
    ];
    for (s, want) in tt {
        assert_eq!(s.parse::<Mode>(), want);
    }
}

pub fn quantize_palette(palette: &[RGB<u8>]) -> impl Fn(RGB<f64>) -> (RGB<f64>, RGB<f64>) {
    let palette = palette.to_vec();
    move |RGB(r0, g0, b0)| {
        // dev note: this is naive implementation and the back of my mind says I can do better
        let mut min_abs_err = std::f64::INFINITY;
        let mut closest: RGB<f64> = RGB::default();
        let mut min_err: RGB<f64> = RGB::default();

        for RGB(r1, g1, b1) in palette.iter().cloned().map(RGB::<f64>::from) {
            let abs_err = f64::abs(r0 - r1) + f64::abs(g0 - g1) + f64::abs(b0 - b1);
            if abs_err < min_abs_err {
                min_err = RGB(r0 - r1, g0 - g1, b0 - b1);
                closest = RGB(r1, g1, b1);
                min_abs_err = abs_err;
            }
        }
        (closest, min_err)
    }
}
