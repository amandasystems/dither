use super::Img;
use std::ops::{Add, Div, Mul};

/// A type of Dither. Available dithers are [Stucki], [Atkinson], [FloydSteinberg], [Burkes], [JarvisJudiceNinke], [Sierra3]
pub trait Dither {
    const DIV: f64;
    const OFFSETS: &'static [(isize, isize, f64)];
    /// dither a 2d matrix.
    /// `P` is the type of pixel ([`u8`], [RGB<f64, f64, f64>]);
    /// S is multiplible and divisble by a **S**CALAR
    /// but adds to ITSELF
    ///
    fn dither<P>(mut img: Img<P>, mut quantize: impl FnMut(P) -> (P, P)) -> super::Img<P>
    where
        P: Add<Output = P> + Mul<f64, Output = P> + Div<f64, Output = P> + Clone + Default,
    {
        let width = img.width() as isize;
        let mut spillover = vec![P::default(); img.len()];
        for (i, p) in img.iter_mut().enumerate() {
            let (quantized, spill) = quantize(p.clone() + spillover[i].clone());
            *p = quantized;

            // add spillover matrices
            for (dx, dy, mul) in Self::OFFSETS.iter().cloned() {
                let j = i as isize + (dy * width) + dx;

                if let Some(stored_spill) = spillover.get_mut(j as usize) {
                    // this cast is OK, since if we go past the edges, we get zero
                    *stored_spill = stored_spill.clone() + (spill.clone() * mul) / Self::DIV;
                }
            }
        }
        img
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub enum Ditherer {
    /// [FloydSteinberg]
    FloydSteinberg,
    /// [Atkinson]
    Atkinson,
    /// [Stucki]
    Stucki,
    /// [Burkes]
    Burkes,
    /// [JardisJudiceNinke]
    JarvisJudiceNinke,
    /// [Sierra3]
    Sierra3,
}

pub struct Stucki;
pub struct Atkinson;
pub struct FloydSteinberg;
pub struct Burkes;
pub struct JarvisJudiceNinke;
pub struct Sierra3;

#[derive(Debug)]
pub struct ErrorUnknownDitherer(String);

const FLOYD: &str = "floyd";
const ATKINSON: &str = "atkinson";
const BURKES: &str = "burkes";
const JARVIS: &str = "jarvis";
const SIERRA: &str = "sierra3";
const STUCKI: &str = "stucki";
impl std::fmt::Display for Ditherer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Ditherer::FloydSteinberg => FLOYD,
                Ditherer::Atkinson => ATKINSON,
                Ditherer::Burkes => BURKES,
                Ditherer::JarvisJudiceNinke => JARVIS,
                Ditherer::Sierra3 => SIERRA,
                Ditherer::Stucki => STUCKI,
            }
        )
    }
}
impl std::str::FromStr for Ditherer {
    type Err = ErrorUnknownDitherer;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_ref() {
            "floyd" | "steinberg" | "floydsteinberg" | "floyd steinberg" => {
                Ditherer::FloydSteinberg
            }
            "atkinson" => Ditherer::Atkinson,
            "stucki" => Ditherer::Stucki,
            "burkes" => Ditherer::Burkes,
            "jarvis" | "judice" | "ninke" => Ditherer::JarvisJudiceNinke,
            "sierra" | "sierra3" => Ditherer::Sierra3,
            _ => return Err(ErrorUnknownDitherer(s.to_string())),
        })
    }
}

impl Ditherer {
    pub fn dither<P>(self, img: Img<P>, quantize: impl FnMut(P) -> (P, P)) -> Img<P>
    where
        P: Add<Output = P> + Mul<f64, Output = P> + Div<f64, Output = P> + Clone + Default,
    {
        match self {
            Ditherer::FloydSteinberg => FloydSteinberg::dither(img, quantize),
            Ditherer::Atkinson => Atkinson::dither(img, quantize),
            Ditherer::Burkes => Burkes::dither(img, quantize),
            Ditherer::Stucki => Stucki::dither(img, quantize),
            Ditherer::JarvisJudiceNinke => JarvisJudiceNinke::dither(img, quantize),
            Ditherer::Sierra3 => Sierra3::dither(img, quantize),
        }
    }
}
impl Dither for Atkinson {
    const DIV: f64 = 8.;
    const OFFSETS: &'static [(isize, isize, f64)] = &[
        // (dx, dy, mul)
        (1, 0, 1.),
        (2, 0, 1.),
        //
        (-1, 1, 1.),
        (0, 1, 1.),
        (1, 1, 1.),
        //
        (0, 2, 1.),
    ];
}

impl Dither for Burkes {
    const DIV: f64 = 32.;
    const OFFSETS: &'static [(isize, isize, f64)] = &[
        // (dx, dy, mul)
        (1, 0, 8.),
        (2, 0, 4.),
        //
        (-2, 1, 2.),
        (-1, 1, 4.),
        (0, 1, 8.),
        (1, 1, 4.),
        (2, 1, 2.),
    ];
}

impl Dither for FloydSteinberg {
    const DIV: f64 = 16.;
    const OFFSETS: &'static [(isize, isize, f64)] =
        &[(1, 0, 7.), (-1, 1, 7.), (0, 1, 5.), (1, 1, 1.)];
}
impl Dither for Stucki {
    const DIV: f64 = 42.;
    const OFFSETS: &'static [(isize, isize, f64)] = &[
        // (dx, dy, mul)
        (1, 0, 8.),
        (2, 0, 4.),
        //
        (-2, 1, 2.),
        (-1, 1, 4.),
        (0, 1, 8.),
        (1, 1, 4.),
        (2, 1, 2.),
        //
        (-2, 2, 1.),
        (-1, 2, 2.),
        (0, 2, 4.),
        (1, 2, 2.),
        (2, 2, 1.),
    ];
}

impl Dither for JarvisJudiceNinke {
    const DIV: f64 = 48.0;
    const OFFSETS: &'static [(isize, isize, f64)] = &[
        // (dx, dy, mul)
        (1, 0, 7.),
        (2, 0, 5.),
        //
        (-2, 1, 3.),
        (-1, 1, 5.),
        (0, 1, 7.),
        (1, 1, 5.),
        (2, 1, 3.),
        //
        (-2, 2, 1.),
        (-1, 2, 3.),
        (0, 2, 5.),
        (1, 2, 3.),
        (2, 2, 1.),
    ];
}

impl Dither for Sierra3 {
    const DIV: f64 = 32.;
    const OFFSETS: &'static [(isize, isize, f64)] = &[
        // (dx, dy, mul)
        (1, 0, 5.),
        (2, 0, 3.),
        //
        (-2, 1, 2.),
        (-1, 1, 4.),
        (0, 1, 5.),
        (1, 1, 4.),
        (2, 1, 2.),
        (-1, 2, 2.),
        //
        (0, 2, 3.),
        (1, 2, 2.),
    ];
}
//
//             X   5   3
//     2   4   5   4   2
//         2   3   2
//           (1/32)

impl std::fmt::Display for ErrorUnknownDitherer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "unknown ditherer: {}", self.0)
    }
}

impl std::error::Error for ErrorUnknownDitherer {}
