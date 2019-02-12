use super::Img;
use std::ops::{Add, Div, Mul};

/// A type of Dither. Available dithers are [Stucki], [Atkinson], [FloydSteinberg], [Burkes], [JarvisJudiceNinke], [Sierra3].
/// See [tanner helland's excellent writeup on dithering algorithms](http://www.tannerhelland.com/4660/dithering-eleven-algorithms-source-code/)
/// for details.
#[derive(Clone, Debug)]
pub struct Ditherer<'a> {
    div: f64,
    offsets: &'a [(isize, isize, f64)],
    name: Option<&'a str>,
}

impl<'a> Ditherer<'a> {
    pub const fn new(div: f64, offsets: &'a [(isize, isize, f64)]) -> Self {
        Ditherer {
            div,
            offsets,
            name: None,
        }
    }
}

pub trait Dither<P> {
    fn dither(&self, img: Img<P>, quantize: impl FnMut(P) -> (P, P)) -> Img<P>;
}

impl<'a, P> Dither<P> for Ditherer<'a>
where
    P: Add<Output = P> + Mul<f64, Output = P> + Div<f64, Output = P> + Clone + Default,
{
    /// dither a 2d matrix.
    /// `P` is the type of pixel ([`u8`], [RGB<f64, f64, f64>]);
    /// S is multiplible and divisble by a **S**CALAR
    /// but adds to ITSELF
    ///
    fn dither(&self, mut img: Img<P>, mut quantize: impl FnMut(P) -> (P, P)) -> super::Img<P> {
        let width = img.width() as isize;
        let mut spillover = vec![P::default(); img.len()];
        for (i, p) in img.iter_mut().enumerate() {
            let (quantized, spill) = quantize(p.clone() + spillover[i].clone());
            *p = quantized;

            // add spillover matrices
            for (dx, dy, mul) in self.offsets.iter().cloned() {
                let j = i as isize + (dy * width) + dx;

                if let Some(stored_spill) = spillover.get_mut(j as usize) {
                    // this cast is OK, since if we go past the edges, we get zero
                    *stored_spill = stored_spill.clone() + (spill.clone() * mul) / self.div;
                }
            }
        }
        img
    }
}

#[derive(Debug)]
pub struct ErrorUnknownDitherer(String);

impl std::str::FromStr for Ditherer<'static> {
    type Err = ErrorUnknownDitherer;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_ref() {
            "floyd" | "steinberg" | "floydsteinberg" | "floyd steinberg" => FLOYD_STEINBERG,
            "atkinson" => ATKINSON,
            "stucki" => STUCKI,
            "burkes" => BURKES,
            "jarvis" | "judice" | "ninke" => JARVIS_JUDICE_NINKE,
            "sierra" | "sierra3" => SIERRA_3,
            _ => return Err(ErrorUnknownDitherer(s.to_string())),
        })
    }
}
pub const ATKINSON: Ditherer = Ditherer {
    name: Some("atkinson"),
    div: 8.,
    offsets: &[
        // (dx, dy, mul)
        (1, 0, 1.),
        (2, 0, 1.),
        //
        (-1, 1, 1.),
        (0, 1, 1.),
        (1, 1, 1.),
        //
        (0, 2, 1.),
    ],
};

pub const BURKES: Ditherer = Ditherer {
    name: Some("burkes"),
    div: 32.,
    offsets: &[
        // (dx, dy, mul)
        (1, 0, 8.),
        (2, 0, 4.),
        //
        (-2, 1, 2.),
        (-1, 1, 4.),
        (0, 1, 8.),
        (1, 1, 4.),
        (2, 1, 2.),
    ],
};

pub const FLOYD_STEINBERG: Ditherer = Ditherer {
    name: Some("floyd"),
    div: 16.,
    offsets: &[(1, 0, 7.), (-1, 1, 7.), (0, 1, 5.), (1, 1, 1.)],
};
pub const STUCKI: Ditherer = Ditherer {
    name: Some("stucki"),
    div: 42.,
    offsets: &[
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
    ],
};

pub const JARVIS_JUDICE_NINKE: Ditherer = Ditherer {
    name: Some("jarvis"),
    div: 48.0,
    offsets: &[
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
    ],
};

pub const SIERRA_3: Ditherer = Ditherer {
    name: Some("sierra3"),
    div: 32.,
    offsets: &[
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
    ],
};
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

impl<'a> std::fmt::Display for Ditherer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if let Some(name) = self.name {
                name
            } else {
                "custom ditherer"
            }
        )
    }
}

impl std::error::Error for ErrorUnknownDitherer {}

impl<'a> Eq for Ditherer<'a> {}

impl<'a> PartialEq for Ditherer<'a> {
    fn eq(&self, other: &Self) -> bool {
        (self.div, self.offsets) == (other.div, other.offsets)
    }
}
