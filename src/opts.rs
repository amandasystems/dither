use crate::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;
#[derive(Debug, StructOpt, Default, PartialEq, Clone)]
#[structopt(name = "dither")]
/// Command-line interface & arguments. See [structopt].
pub struct Opt {
    /// Provide verbose debug information. Default is false.
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,
    /// Input file. Supported file types:
    /// `PNG`
    /// `JPEG`
    /// `GIF`
    /// `BMP`
    /// `ICO`
    /// `TIFF`
    #[structopt(name = "input", parse(from_os_str))]
    pub input: PathBuf,

    /// Color depth. Must be between 1 and 8. See [create_quantize_n_bits_func][crate::create_quantize_n_bits_func] and [create_convert_quantized_to_palette_func][crate::create_convert_quantized_to_palette_func]
    #[structopt(long = "depth", default_value = "1")]
    pub bit_depth: u8,

    /// Output file: will be written to as a .png or .jpg (inferred from file extension). If left empty,
    /// a default output path will be created: see [Opt::output_path]
    #[structopt(name = "output", parse(from_os_str))]
    pub output: Option<PathBuf>,

    /// Ditherering algorithm to use. Options are
    /// - "floyd" (default)
    /// - "atkinson"
    /// - "stucki",
    /// - "burkes"
    /// - "jarvis"
    /// - "sierra3"
    ///
    #[structopt(short = "d", long = "dither", default_value = "floyd")]
    pub ditherer: Ditherer<'static>,

    /// Color mode to use.
    /// Options are
    /// -"bw" -> grayscale (black and white if bit-depth = 1)
    /// - $COLOR => single-color mode. options are
    ///     - "BLUE"
    ///     - "GREEN"
    ///     - "CYAN"
    ///     - "RED"
    ///     - "MAGENTA"
    ///     - "BROWN",
    ///     - "LIGHT_GRAY"
    ///     - "GRAY"
    ///     - "LIGHT_BLUE",
    ///     - "LIGHT_GREEN"
    ///     - "LIGHT_CYAN",
    ///     - "LIGHT_RED"
    ///     - "LIGHT_MAGENTA"
    ///     - "YELLOW"
    ///     - "WHITE"
    /// - ("0xYYYYYY 0xZZZZZZ") -> user specified 1bit user color palette; where the first is foreground in hexidecimal and the second is background.
    /// - "cga" -> sixteen-color CGA. ignores bit depth; causes error on bit depth > 1
    #[structopt(short = "c", long = "color", default_value = "bw")]
    pub color_mode: color::Mode,
}

impl Opt {
    /// the actual output path. if opts.output exists, this is that; otherwise, this is
    /// `"{base}_dithered_{dither}_{color}_{depth}.png"`,
    /// where base is the input path, stripped of it's extension.
    /// `$dither bunny.png --color=color --dither=atkinson --depth=2` will save to `bunny_atkinson_c_2.png`
    ///
    /// ```
    /// # use dither::prelude::*;
    /// let mut opt = Opt::default();
    /// opt.bit_depth=1;
    /// opt.input = std::path::PathBuf::from("bunny.png".to_string());
    /// assert_eq!(opt.output_path().to_string_lossy(), "bunny_dithered_floyd_bw_1.png");
    /// ```
    pub fn output_path(&self) -> std::borrow::Cow<PathBuf> {
        if let Some(output) = &self.output {
            std::borrow::Cow::Borrowed(output)
        } else {
            let output_stem = self
                .input
                .file_stem()
                .unwrap_or_else(|| self.input.as_ref()) // no extension; use the whole path
                .to_string_lossy();
            std::borrow::Cow::Owned(std::path::PathBuf::from(format!(
                "{base}_dithered_{dither}_{color}_{depth}.png",
                base = output_stem,
                dither = self.ditherer,
                color = self.color_mode,
                depth = self.bit_depth,
            )))
        }
    }
}
