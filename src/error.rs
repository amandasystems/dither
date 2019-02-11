#[derive(Debug)]
/// Error types for [super::main]
pub enum Error {
    IO(std::io::Error),
    Image(image::ImageError),
    BadBitDepth(u8),
    Color(super::color::Error),
    IncompatibleOptions,
}

/// Result type for [super::main]
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

impl From<super::color::Error> for Error {
    fn from(err: super::color::Error) -> Self {
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
