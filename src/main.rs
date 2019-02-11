//! # Dither
//!
//! Inspired by: <http://www.tannerhelland.com/4660/dithering-eleven-algorithms-source-code/>

#[macro_use]
extern crate lazy_static;

use structopt::StructOpt;
pub mod color;
pub mod dither;
mod error;
mod img;
mod opts;

#[cfg(test)]
mod tests;

use self::{
    color::{CGA, RGB},
    dither::Ditherer,
    error::{Error, Result},
    img::Img,
    opts::Opt,
};

fn main() -> Result<()> {
    let opts = Opt::from_args();
    _main(&opts)
}

fn _main(opts: &Opt) -> Result<()> {
    if opts.verbose {
        eprintln!(
            concat!(
                "running dither in VERBOSE mode:\n\t",
                "INPUT: {input}\n\t",
                "OUTPUT: {output}\n\t",
                "DITHERER: {dither}\n\t",
                "BIT_DEPTH: {depth}\n\t",
                "COLOR_MODE: {mode}"
            ),
            input = opts.input.canonicalize()?.to_string_lossy(),
            dither = opts.ditherer,
            depth = opts.bit_depth,
            mode = opts.color_mode,
            output = opts.output_path().canonicalize()?.to_string_lossy(),
        );
    }
    let img: Img<RGB<f64>> =
        Img::<RGB<u8>>::load(&opts.input)?.convert_with(|rgb| rgb.convert_with(f64::from));

    if opts.verbose {
        eprintln!(
            "image loaded from \"{}\".\ndithering...",
            opts.input.canonicalize().unwrap().to_string_lossy()
        )
    }
    let quantize = create_quantize_n_bits_func(opts.bit_depth)?;

    let output_img = match opts.color_mode {
        color::Mode::CGA | color::Mode::CustomPalette { .. } if opts.bit_depth > 1 => {
            return Err(Error::IncompatibleOptions);
        }

        color::Mode::Color => opts
            .ditherer
            .dither(img, RGB::map_across(quantize))
            .convert_with(|rgb| rgb.convert_with(clamp_f64_to_u8)),

        color::Mode::CGA => opts
            .ditherer
            .dither(img, CGA::quantize)
            .convert_with(|rgb| rgb.convert_with(clamp_f64_to_u8)),

        color::Mode::BlackAndWhite => {
            let bw_img = img.convert_with(|rgb| rgb.to_chroma_corrected_black_and_white());
            opts.ditherer
                .dither(bw_img, quantize)
                .convert_with(RGB::from_chroma_corrected_black_and_white)
        }

        color::Mode::SingleColor(color) => {
            if opts.verbose {
                eprintln!("single_color mode: {}", color)
            }
            let (front, _) = color::Mode::custom_palette_from_cga(color);

            let bw_img = img.convert_with(|rgb| rgb.to_chroma_corrected_black_and_white());
            let RGB(r, g, b) = front;
            opts.ditherer
                .dither(bw_img, quantize)
                .convert_with(|x: f64| {
                    RGB(
                        clamp_f64_to_u8(f64::from(r) / 255. * x),
                        clamp_f64_to_u8(f64::from(g) / 255. * x),
                        clamp_f64_to_u8(f64::from(b) / 255. * x),
                    )
                })
        }

        color::Mode::CustomPalette { front, back } => {
            if opts.verbose {
                eprintln!("cutom palette: front: {:?}, back {:?} ", &front, &back);
            }
            let bw_img = img.convert_with(|rgb| rgb.to_chroma_corrected_black_and_white());
            opts.ditherer
                .dither(bw_img, quantize)
                .convert_with(create_convert_quantized_to_palette_func(front, back))
                .convert_with(|rgb| rgb.convert_with(clamp_f64_to_u8))
        }
    };
    if opts.verbose {
        eprintln!("dithering complete.\nsaving...");
    }
    output_img.save(opts.output_path().as_ref())?;
    if opts.verbose {
        eprintln!("program finished");
    }
    Ok(())
}

/// quantize to n bits
/// ```
/// # use dither::create_quantize_n_bits_func;
/// let quantize_1_bit = |n: u8| if n > 127 {255, 255-n} else {0, n};
/// let quantization_func = create_quantize_n_bits_func(1);
/// assert_eq!(quantize_1_bit(5), create_quantize_n_bits_func(1)(5));
/// ```
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

fn create_convert_quantized_to_palette_func(
    front: RGB<u8>,
    back: RGB<u8>,
) -> impl Fn(f64) -> RGB<f64> {
    let front = RGB::<f64>::from(front) / 255.;
    let back = RGB::<f64>::from(back) / 255.;
    move |x: f64| front.clone() * x + (back.clone() * (255. - x))
}

pub fn clamp_f64_to_u8(n: f64) -> u8 {
    match n {
        n if n > 255.0 => 255,
        n if n < 0.0 => 0,
        n => n as u8,
    }
}
