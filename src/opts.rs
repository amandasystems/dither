use std::path::PathBuf;
use structopt::StructOpt;
#[derive(Debug, StructOpt, PartialEq, Clone)]
#[structopt(name = "dither")]
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

    /// Color depth. Must be between 1 and 8.
    #[structopt(long = "depth", default_value = "1")]
    pub bit_depth: u8,

    /// Output file: will be written to as a .png or .jpg (inferred from file extension). If left empty,
    /// a default output path will be created: see [dither::opts::Opt::output_path]
    #[structopt(name = "output", parse(from_os_str))]
    output: Option<PathBuf>,

    /// Ditherering algorithm to use. Options are
    /// "floyd", "atkinson", "stucki", "burkes","jarvis", "sierra3"
    #[structopt(short = "d", long = "dither", default_value = "floyd")]
    pub ditherer: super::Ditherer,

    /// Color mode to use.
    /// Options are "color", "bw", "cga", $SINGLE_COLOR, "$FRONT_PALETTE $BACKPALETTE";
    /// (all colors with specified bit depth)
    /// "bw" -> black and white mode (grayscale in higher birt depths)
    /// $COLOR => single-color mode. options are
    ///     ["BLUE", "GREEN", "CYAN",
    ///     "RED", "MAGENTA", "BROWN",
    ///     "LIGHT_GRAY", "GRAY", "LIGHT_BLUE",
    /// "LIGHT_GREEN", "LIGHT_CYAN", "LIGHT_RED"
    /// "LIGHT_MAGENTA", "YELLOW", "WHITE"
    /// ("0xZZZZZZ 0xZZZZZZZ") -> user specified 1bit user color palette; where the first is foreground in hexidecimal and the second is background.
    /// "cga" -> sixteen-color CGA. ignores bit depth; casues error on bit depth > 1
    #[structopt(short = "c", long = "color", default_value = "bw")]
    pub color_mode: super::color::Mode,
}

impl Opt {
    /// the actual output path. if opts.output exists, this is that; otherwise, this is
    /// "{base}_dithered_{dither}_{color}_{depth}.png",
    /// where base is the input path, stripped of it's extension.
    /// `$dither bunny.png --color=color --dither=atkinson --depth=2` will save to bunny_atkinson_c_2.png
    ///
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
