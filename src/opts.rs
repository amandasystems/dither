use crate::prelude::*;
use std::borrow::Cow;
use std::path::{Path, PathBuf};
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
    /// - bw (default)
    /// - color
    /// - cga => load the cga palette. equivalent to "cga.plt".
    /// - $COLOR => single-color mode. options are
    /// - $FILENAME" => load palette from file, listed as line-separated RGB values. see "cga.plt" and the readme for more information on palette files.
    #[structopt(short = "c", long = "color", default_value = "bw")]
    pub color_mode: color::Mode,
}

impl Opt {
    /// the actual output path. if opts.output exists, this is that, otherwise, this is
    /// `"{base}_dithered_{dither}_{color}_{depth}.png"`,
    /// where base is the [canonicalized][std::fs::canonicalize] input path, stripped of it's extension.
    /// `$dither bunny.png --color=color --dither=atkinson --depth=2` will save to `bunny_atkinson_c_2.png`
    ///
    /// ```
    /// # use dither::prelude::*;
    /// # use std::path::{PathBuf,Path};
    /// let mut opt = Opt::default();
    /// opt.bit_depth=1;
    /// opt.input = PathBuf::from("bunny.png".to_string());
    /// let got_path = opt.output_path().unwrap();
    /// assert_eq!("bunny_dithered_floyd_bw_1.png", Path::file_name(got_path.as_ref().as_ref()).unwrap().to_string_lossy());
    /// ```
    ///
    pub fn input_path<'a>(&'a self) -> Result<PathBuf> {
        match self.input.canonicalize() {
            Err(err) => return Err(Error::Input(IOError::new(err, &self.input))),
            Ok(abs_path) => Ok(abs_path),
        }
    }
    pub fn output_path<'a>(&'a self) -> Result<Cow<'a, Path>> {
        if let Some(path) = &self.output {
            return Ok(Cow::Borrowed(&path));
        }

        let abs_path = match self.input.canonicalize() {
            Err(err) => return Err(Error::Input(IOError::new(err, &self.input))),
            Ok(abs_path) => abs_path,
        };
        let path = format!(
            "{base}_dithered_{dither}_{color}_{depth}.png",
            base = abs_path.file_stem().unwrap_or_default().to_string_lossy(),
            dither = self.ditherer,
            color = self.color_mode,
            depth = self.bit_depth,
        );
        Ok(Cow::Owned(PathBuf::from(path)))
    }
}
