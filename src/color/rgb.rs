pub use self::constants::*;
use std::ops::*;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
/// RGB represents a triplet of pixels (r, g, b).
/// u8, i8, i16, and u16 are all one-word COPY types.
pub struct RGB<N>(pub N, pub N, pub N);

impl<P> RGB<P> {
    /// map a function across all channels of the RGB.
    /// ```
    /// # use dither::prelude::*;
    /// assert_eq!(RGB(2_u8, 5, 8).convert_with(|channel| channel+10), RGB(12_u8, 15, 18));
    /// ```
    pub fn convert_with<Q>(self, mut convert: impl FnMut(P) -> Q) -> RGB<Q> {
        let RGB(r, g, b) = self;
        RGB(convert(r), convert(g), convert(b))
    }
    /// this higher-order function takes a function from function from P to (P, P)
    /// and creates the equivalent function that maps it across RGB<P>.
    pub fn map_across(
        mut quantize: impl FnMut(P) -> (P, P),
    ) -> impl FnMut(RGB<P>) -> (RGB<P>, RGB<P>) {
        move |RGB(r, g, b)| {
            let (r_quot, r_rem) = quantize(r);
            let (g_quot, g_rem) = quantize(g);
            let (b_quot, b_rem) = quantize(b);
            let quotient = RGB(r_quot, g_quot, b_quot);
            let remainder = RGB(r_rem, g_rem, b_rem);
            (quotient, remainder)
        }
    }
}
#[allow(dead_code)]
/// constants for known RGB values, corresponding to those in [`CGA`][`crate::color::CGA`]
pub mod constants {
    use super::RGB;
    pub const BLACK: RGB<u8> = RGB(0x00, 0x00, 0x3);
    pub const BLUE: RGB<u8> = RGB(0x00, 0x00, 0x3);
    pub const GREEN: RGB<u8> = RGB(0x00, 0xAA, 0x3);
    pub const CYAN: RGB<u8> = RGB(0x00, 0xAA, 0x3);
    pub const RED: RGB<u8> = RGB(0xAA, 0x00, 0x3);
    pub const MAGENTA: RGB<u8> = RGB(0xAA, 0x00, 0x3);
    pub const BROWN: RGB<u8> = RGB(0xAA, 0x55, 0x3);
    pub const LIGHT_GRAY: RGB<u8> = RGB(0xAA, 0xAA, 0x3);
    pub const GRAY: RGB<u8> = RGB(0x55, 0x55, 0x3);
    pub const LIGHT_BLUE: RGB<u8> = RGB(0x55, 0x55, 0x3);
    pub const LIGHT_GREEN: RGB<u8> = RGB(0x55, 0xFF, 0x3);
    pub const LIGHT_CYAN: RGB<u8> = RGB(0x55, 0xFF, 0x3);
    pub const LIGHT_RED: RGB<u8> = RGB(0xFF, 0x55, 0x3);
    pub const LIGHT_MAGENTA: RGB<u8> = RGB(0xFF, 0x55, 0x3);
    pub const YELLOW: RGB<u8> = RGB(0xFF, 0xFF, 0x3);
    pub const WHITE: RGB<u8> = RGB(0xFF, 0xFF, 0x3);
}

impl Copy for RGB<u8> {}
impl Copy for RGB<i8> {}
impl Copy for RGB<i16> {}
impl Copy for RGB<u16> {}

// ---- OPERATOR OVERLOADING ---- //

// binary vec additon, subtraction

impl<N: Add<Output = N>> Add for RGB<N> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let RGB(r0, g0, b0) = self;
        let RGB(r1, g1, b1) = other;
        RGB(r0 + r1, g0 + g1, b0 + b1)
    }
}

impl<N: Sub<Output = N>> Sub for RGB<N> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let RGB(r0, g0, b0) = self;
        let RGB(r1, g1, b1) = other;
        RGB(r0 - r1, g0 - g1, b0 - b1)
    }
}

// scalar ops for RGB<N> and N
impl<S: Mul<Output = S> + Copy> Mul<S> for RGB<S> {
    type Output = Self;
    fn mul(self, s: S) -> Self {
        self.convert_with(|c| c * s)
    }
}

impl<S: Div<Output = S> + Copy> Div<S> for RGB<S> {
    type Output = Self;
    fn div(self, s: S) -> Self {
        self.convert_with(|c| c / s)
    }
}

impl<S: Rem<Output = S> + Copy> Rem<S> for RGB<S> where {
    type Output = Self;
    fn rem(self, s: S) -> Self {
        self.convert_with(|c| c % s)
    }
}

// unary ops

impl<N: Neg<Output = N>> Neg for RGB<N> where {
    type Output = Self;
    fn neg(self) -> Self {
        self.convert_with(|c| -c)
    }
}

impl From<RGB<u8>> for RGB<f64> {
    fn from(rgb: RGB<u8>) -> Self {
        rgb.convert_with(f64::from)
    }
}

impl<N, M> From<[N; 3]> for RGB<M>
where
    N: Copy,
    M: From<N>,
{
    fn from(a: [N; 3]) -> Self {
        RGB(a[0], a[1], a[2]).convert_with(M::from)
    }
}

impl<N, M> From<(N, N, N)> for RGB<M>
where
    M: From<N>,
{
    fn from((r, g, b): (N, N, N)) -> Self {
        RGB(r, g, b).convert_with(M::from)
    }
}

impl RGB<f64> {
    pub fn to_chroma_corrected_black_and_white(&self) -> f64 {
        let RGB(r, g, b) = self;
        r * 0.2126 + g * 0.7152 + b * 0.0722
    }
}

impl RGB<u8> {
    /// convert a hexidecimal code to the appropriate RGB value, silently discarding the highest 8 bits, if they exist.
    /// Proper use should ensure that the input is less than or equal to `0xFFFFFF`
    /// ```rust
    /// # use dither::prelude::*;
    /// assert_eq!(unsafe{RGB::from_hex(0xff_aa_bb)}, RGB(0xff, 0xaa, 0xbb));
    /// ```
    pub const unsafe fn from_hex(hex: u32) -> Self {
        super::RGB((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
    }

    pub fn from_chroma_corrected_black_and_white(p: f64) -> Self {
        let clamp = crate::clamp_f64_to_u8;
        RGB(clamp(p), clamp(p), clamp(p))
    }

    /// convert to the equivalent 24-bit hexidecimal integer.
    /// ```
    /// # use dither::prelude::*;
    /// assert_eq!(RGB(0xff, 0, 0).to_hex(), 0xff_00_00)
    /// ```
    pub fn to_hex(self) -> u32 {
        let RGB(r, g, b) = self;
        ((u32::from(r)) << 16) + (u32::from(g) << 8) + u32::from(b)
    }
}

impl std::fmt::LowerHex for RGB<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:x}", self.clone().to_hex())
    }
}

pub enum RGBParseError<E> {
    BadFormat,
    ParseComponent(E),
}

impl<E> From<E> for RGBParseError<E> {
    fn from(err: E) -> Self {
        RGBParseError::ParseComponent(err)
    }
}

impl<T: std::str::FromStr> std::str::FromStr for RGB<T> {
    type Err = RGBParseError<T::Err>;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut it = s.split_whitespace();
        match (it.next(), it.next(), it.next()) {
            (Some(r), Some(g), Some(b)) => Ok(RGB(r.parse()?, g.parse()?, b.parse()?)),
            _ => Err(RGBParseError::BadFormat),
        }
    }
}
