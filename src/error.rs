//! Error and result types for runtime error.
use crate::color;
#[derive(Debug)]
/// Handling of runtime errors in main.
pub enum Error {
    /// An error from std::io;
    IO(std::io::Error),
    // An error from the [image] package.
    Image(image::ImageError),
    /// A bit depth that's not in the [range][std::ops::Range] `0..8`
    BadBitDepth(u8),
    /// An error creating a [color::Mode]
    Color(color::Error),
    /// Incompatible [options][crate::Opt] specified by the user.
    IncompatibleOptions,
}

/// Result type for [Error]
pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Image(err) => write!(f, "image library error: {}", err),
            Error::IO(err) => write!(f, "io error: {}", err),
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
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<image::ImageError> for Error {
    fn from(err: image::ImageError) -> Self {
        Error::Image(err)
    }
}
