//! # Dither
//!
//! Inspired by: <http://www.tannerhelland.com/4660/dithering-eleven-algorithms-source-code/>

#[macro_use]
extern crate lazy_static;

/// handling of color
pub mod color;
pub mod ditherer;
pub mod error;
pub mod img;
pub mod opts;
pub mod prelude;

use self::prelude::*;
#[cfg(test)]
mod tests;

/// quantize to n bits
pub fn create_quantize_n_bits_func(n: u8) -> Result<impl FnMut(f64) -> (f64, f64)> {
    if n == 0 || n > 7 {
        Err(Error::BadBitDepth(n))
    } else {
        Ok(move |x: f64| {
            let step_size = 256. / f64::from(n);

            let floor = f64::floor(x / step_size) * step_size;
            let floor_rem = x - floor;

            let ceil = f64::ceil(x / step_size) * step_size;
            let ceil_rem = ceil - x;

            if floor_rem < ceil_rem {
                let quot = f64::max(floor, 0.0);
                (quot, floor_rem)
            } else {
                let quot = f64::min(255.0, ceil);
                (quot, -ceil_rem)
            }
        })
    }
}

/// create a function that converts a quantized black-and-white image to the appropriate palette. i.e,
/// ```
/// # use dither::create_convert_quantized_to_palette_func;
/// # use dither::prelude::*;
/// let r2b = create_convert_quantized_to_palette_func(RGB(255, 0, 0), RGB(0, 0, 255));
/// assert_eq!(r2b(55.), RGB(55., 0., 200.));
/// ```
pub fn create_convert_quantized_to_palette_func(
    front: RGB<u8>,
    back: RGB<u8>,
) -> impl Fn(f64) -> RGB<f64> {
    let front = RGB::<f64>::from(front) / 255.;
    let back = RGB::<f64>::from(back) / 255.;
    move |x: f64| front.clone() * x + (back.clone() * (255. - x))
}

/// clamp a f64 to the closest u8, rounding non-integers.
/// ```
/// # use dither::clamp_f64_to_u8;
/// assert_eq!(clamp_f64_to_u8(255.2), 255);
/// assert_eq!(clamp_f64_to_u8(2.8), 3);
/// ```
pub fn clamp_f64_to_u8(n: f64) -> u8 {
    match n {
        n if n > 255.0 => 255,
        n if n < 0.0 => 0,
        n => n.round() as u8,
    }
}
