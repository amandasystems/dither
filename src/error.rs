//! Error and result types for runtime error.
use crate::color;

/// Handling of runtime errors in main.
#[derive(Debug)]
pub enum Error {
    /// An error saving an image from file.
    Output(std::io::Error, String),
    // An error loading an image.
    Input(image::ImageError, String),
    /// A bit depth that's not in the [range][std::ops::Range] `0..8`
    BadBitDepth(u8),
    /// An error creating a [color::Mode]
    Color(color::Error),
    /// Incompatible [options][crate::Opt] specified by the user.
    IncompatibleOptions,
}

/// Result type for [Error]
pub type Result<T> = std::result::Result<T, Error>;

impl<'a> std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Input(err, path) => write!(f, "input error loading from {}: {}",path, err ),
            Error::Output(err, path) => write!(f, "output error saving to {}: {}", path, err),
            Error::BadBitDepth(n) => write!(f, "bit depth must be between 1 and 7, but was {}", n),
            Error::Color(err) => err.fmt(f),
            Error::IncompatibleOptions => write!(
                f,
                "the palette (-p, --palette) option is mututally exclusive with the bit depth and color options."
            ),
        }
    }
}

impl From<color::Error> for Error {
    fn from(err: color::Error) -> Self {
        Error::Color(err)
    }
}

impl std::error::Error for Error {}
