use super::{hex, RGB};
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
/// CGA is a 16-color palette developed by IBM and used in early computing.
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
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15,
}

impl CGA {
    /// quantize a RGB triplet to the closest CGA color and error.

    pub fn quantize(RGB(r0, g0, b0): RGB<f64>) -> (RGB<f64>, RGB<f64>) {
        let mut min_abs_err = std::f64::INFINITY;
        let mut closest = RGB::default();
        let mut min_err = RGB::default();

        // dev note: just iterating through all sixteen colors is a pretty naive approach. I think algorithimically there's a better way,
        // but since there's only sixteen posibilities iterating through is very fast.
        for RGB(r1, g1, b1) in CGA::COLORS.iter().cloned().map(RGB::<f64>::from) {
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

#[test]
fn test_quantize() {
    let cyan: RGB<f64> = RGB::from(CGA::Cyan);
    let offset = RGB(1.2, 2.2, -3.1);
    let nearly_cyan = cyan.clone() + offset.clone();

    let (got_quantized, RGB(dr, dg, db)) = CGA::quantize(nearly_cyan);
    assert_eq!(got_quantized, cyan);

    let want_abs_err = 1.2 + 2.2 + 3.1;
    let got_abs_err = dr.abs() + dg.abs() + db.abs();
    assert!(f64::abs(got_abs_err - want_abs_err) < 1e-9); // need to account for floating point error
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UnknownCGAColorError;

impl std::fmt::Display for UnknownCGAColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "unknown CGA color")
    }
}

impl std::error::Error for UnknownCGAColorError {}

impl CGA {
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
    pub fn to_hex(self) -> u32 {
        match self {
            CGA::Black => hex::BLACK,
            CGA::Blue => hex::BLUE,
            CGA::Green => hex::GREEN,
            CGA::Cyan => hex::CYAN,
            CGA::Red => hex::RED,
            CGA::Magenta => hex::MAGENTA,
            CGA::Brown => hex::BROWN,
            CGA::LightGray => hex::LIGHT_GRAY,
            CGA::Gray => hex::GRAY,
            CGA::LightBlue => hex::LIGHT_BLUE,
            CGA::LightGreen => hex::LIGHT_GREEN,
            CGA::LightCyan => hex::LIGHT_CYAN,
            CGA::LightRed => hex::LIGHT_RED,
            CGA::LightMagenta => hex::LIGHT_MAGENTA,
            CGA::Yellow => hex::YELLOW,
            CGA::White => hex::WHITE,
        }
    }

    pub fn try_from_hex(hex: u32) -> Option<Self> {
        Some(match hex {
            hex::BLACK => CGA::Black,
            hex::BLUE => CGA::Blue,
            hex::GREEN => CGA::Green,
            hex::CYAN => CGA::Cyan,
            hex::RED => CGA::Red,
            hex::MAGENTA => CGA::Magenta,
            hex::BROWN => CGA::Brown,
            hex::LIGHT_GRAY => CGA::LightGray,
            hex::GRAY => CGA::Gray,
            hex::LIGHT_BLUE => CGA::LightBlue,
            hex::LIGHT_GREEN => CGA::LightGreen,
            hex::LIGHT_CYAN => CGA::LightCyan,
            hex::LIGHT_RED => CGA::LightRed,
            hex::LIGHT_MAGENTA => CGA::LightMagenta,
            hex::YELLOW => CGA::Yellow,
            hex::WHITE => CGA::White,
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
