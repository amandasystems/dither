//! The actual runtime library.
use dither::prelude::*;
use structopt::StructOpt;
fn main() -> Result<()> {
    let opts = Opt::from_args();
    _main(&opts)
}

pub fn _main(opts: &Opt) -> Result<()> {
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
    let quantize = dither::create_quantize_n_bits_func(opts.bit_depth)?;

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

            let bw_img = img.convert_with(|rgb| rgb.to_chroma_corrected_black_and_white());
            let RGB(r, g, b) = RGB::<u8>::from(color);

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
