use super::RGB;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
/// The CGA color palette. This treats [CGA::Yellow] as a true yellow rather than the traditional light brown.
pub enum CGA {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    Gray = 8,
    LightBlue = 9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    LightMagenta = 0xD,
    Yellow = 0xE,
    White = 0xF,
}

impl CGA {
    /// an iterator through all the variations of the enum
    pub fn iter() -> impl Iterator<Item = Self> {
        Self::COLORS.iter().cloned()
    }

    /// quantize a RGB triplet to the closest CGA color and error.
    /// ```
    /// # use dither::prelude::*;
    /// let nearly_red = RGB(f64::from(0xAA-3), 0., 0.);
    /// let (got, got_err) = CGA::quantize(nearly_red);
    /// assert_eq!(got, RGB::from(CGA::Red));
    /// assert_eq!(got_err, RGB(-3., 0., 0.));
    /// ```
    pub fn quantize(RGB(r0, g0, b0): RGB<f64>) -> (RGB<f64>, RGB<f64>) {
        // dev note: this is naive implementation and the back of my mind says I can do better
        let mut min_abs_err = std::f64::INFINITY;
        let mut closest = RGB::default();
        let mut min_err = RGB::default();

        for RGB(r1, g1, b1) in CGA::iter().map(RGB::from) {
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

// constants
impl CGA {
    /// array containing all of the enum variations
    pub const COLORS: [CGA; 16] = [
        CGA::Black,
        CGA::Blue,
        CGA::Green,
        CGA::Cyan,
        CGA::Red,
        CGA::Magenta,
        CGA::Brown,
        CGA::LightCyan,
        CGA::Gray,
        CGA::LightBlue,
        CGA::LightGreen,
        CGA::LightCyan,
        CGA::LightRed,
        CGA::LightMagenta,
        CGA::Yellow,
        CGA::White,
    ];
    /// hexidecimal representation of [CGA::Black]
    pub const BLACK: u32 = 0x00_00_00;
    /// hexidecimal representation of [CGA::Blue]
    pub const BLUE: u32 = 0x00_00_AA;
    /// hexidecimal representation of [CGA::Green]
    pub const GREEN: u32 = 0x00_AA_00;
    /// hexidecimal representation of [CGA::Cyan]
    pub const CYAN: u32 = 0x00_AA_AA;
    /// hexidecimal representation of [CGA::Red]
    pub const RED: u32 = 0xAA_00_00;
    /// hexidecimal representation of [CGA::Magenta]
    pub const MAGENTA: u32 = 0xAA_00_AA;
    /// hexidecimal representation of [CGA::Brown]
    pub const BROWN: u32 = 0xAA_55_00;
    /// hexidecimal representation of [CGA::LightGray]
    pub const LIGHT_GRAY: u32 = 0xAA_AA_AA;
    /// hexidecimal representation of [CGA::Gray]
    pub const GRAY: u32 = 0x55_55_55;
    /// hexidecimal representation of [CGA::LightBlue]
    pub const LIGHT_BLUE: u32 = 0x55_55_FF;
    /// hexidecimal representation of [CGA::LightGreen]
    pub const LIGHT_GREEN: u32 = 0x55_FF_55;
    /// hexidecimal representation of [CGA::LightCyan]
    pub const LIGHT_CYAN: u32 = 0x55_FF_FF;
    /// hexidecimal representation of [CGA::LightRed]
    pub const LIGHT_RED: u32 = 0xFF_55_55;
    /// hexidecimal representation of [CGA::LightMagenta]
    pub const LIGHT_MAGENTA: u32 = 0xFF_55_FF;
    /// hexidecimal representation of [CGA::Yellow]
    pub const YELLOW: u32 = 0xFF_FF_55;
    /// hexidecimal representation of [CGA::White]
    pub const WHITE: u32 = 0xFF_FF_FF;
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// An error parsing a CGA color.
pub struct UnknownCGAColorError;

impl std::fmt::Display for UnknownCGAColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let names = CGA::iter().map(|cga| cga.into()).collect::<Vec<&str>>();
        write!(f, "unknown CGA color. options are: {:#?},", names)
    }
}

impl std::error::Error for UnknownCGAColorError {}

impl CGA {
    /// convert to the equivalent hexideicmal code.
    /// ``` rust
    /// # use dither::prelude::*;
    /// assert_eq!(CGA::Red.to_hex(), CGA::RED)
    /// ```
    pub fn to_hex(self) -> u32 {
        match self {
            CGA::Black => CGA::BLACK,
            CGA::Blue => CGA::BLUE,
            CGA::Green => CGA::GREEN,
            CGA::Cyan => CGA::CYAN,
            CGA::Red => CGA::RED,
            CGA::Magenta => CGA::MAGENTA,
            CGA::Brown => CGA::BROWN,
            CGA::LightGray => CGA::LIGHT_GRAY,
            CGA::Gray => CGA::GRAY,
            CGA::LightBlue => CGA::LIGHT_BLUE,
            CGA::LightGreen => CGA::LIGHT_GREEN,
            CGA::LightCyan => CGA::LIGHT_CYAN,
            CGA::LightRed => CGA::LIGHT_RED,
            CGA::LightMagenta => CGA::LIGHT_MAGENTA,
            CGA::Yellow => CGA::YELLOW,
            CGA::White => CGA::WHITE,
        }
    }
    /// convert a color specified as a hex code (i.e, 0xFF0000) to the appropriate CGA color, if it exists
    /// ```
    /// # use dither::prelude::*;
    /// assert_eq!(CGA::try_from_hex(CGA::RED), Some(CGA::Red));
    /// assert_eq!(CGA::try_from_hex(0x12_34_56), None);
    /// ```
    pub fn try_from_hex(hex: u32) -> Option<Self> {
        Some(match hex {
            CGA::BLACK => CGA::Black,
            CGA::BLUE => CGA::Blue,
            CGA::GREEN => CGA::Green,
            CGA::CYAN => CGA::Cyan,
            CGA::RED => CGA::Red,
            CGA::MAGENTA => CGA::Magenta,
            CGA::BROWN => CGA::Brown,
            CGA::LIGHT_GRAY => CGA::LightGray,
            CGA::GRAY => CGA::Gray,
            CGA::LIGHT_BLUE => CGA::LightBlue,
            CGA::LIGHT_GREEN => CGA::LightGreen,
            CGA::LIGHT_CYAN => CGA::LightCyan,
            CGA::LIGHT_RED => CGA::LightRed,
            CGA::LIGHT_MAGENTA => CGA::LightMagenta,
            CGA::YELLOW => CGA::Yellow,
            CGA::WHITE => CGA::White,
            _ => return None,
        })
    }
}

impl std::str::FromStr for CGA {
    type Err = UnknownCGAColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_uppercase().as_ref() {
            "BLACK" => CGA::Black,
            "BLUE" => CGA::Blue,
            "GREEN" => CGA::Green,
            "CYAN" => CGA::Cyan,
            "RED" => CGA::Red,
            "MAGENTA" => CGA::Magenta,
            "BROWN" => CGA::Brown,
            "LIGHT_GRAY" => CGA::LightGray,
            "GRAY" => CGA::Gray,
            "LIGHT_BLUE" => CGA::LightBlue,
            "LIGHT_GREEN" => CGA::LightGreen,
            "LIGHT_CYAN" => CGA::LightCyan,
            "LIGHT_RED" => CGA::LightRed,
            "LIGHT_MAGENTA" => CGA::LightMagenta,
            "YELLOW" => CGA::Yellow,
            "WHITE" => CGA::White,
            _ => return Err(UnknownCGAColorError),
        })
    }
}

impl std::fmt::Display for CGA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s: &'static str = (*self).into();
        write!(f, "{}", s)
    }
}

impl Into<&'static str> for CGA {
    fn into(self) -> &'static str {
        match self {
            CGA::Black => "BLACK",
            CGA::Blue => "BLUE",
            CGA::Green => "GREEN",
            CGA::Cyan => "CYAN",
            CGA::Red => "RED",
            CGA::Magenta => "MAGENTA",
            CGA::Brown => "BROWN",
            CGA::LightGray => "LIGHT_GRAY",
            CGA::Gray => "GRAY",
            CGA::LightBlue => "LIGHT_BLUE",
            CGA::LightGreen => "LIGHT_GREEN",
            CGA::LightCyan => "LIGHT_CYAN",
            CGA::LightRed => "LIGHT_RED",
            CGA::LightMagenta => "LIGHT_MAGENTA",
            CGA::Yellow => "YELLOW",
            CGA::White => "WHITE",
        }
    }
}
