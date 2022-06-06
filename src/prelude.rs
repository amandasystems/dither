//! Re-exports of the most common traits and types.
pub use super::*;

#[cfg(feature = "clap")]
pub use self::{
    color::{palette::Palette, RGB},
    ditherer::{Dither, Ditherer},
    error::{Error, Result},
    img::Img,
    opts::Opt,
};


#[cfg(not(feature = "clap"))]
pub use self::{
    color::{palette::Palette, RGB},
    ditherer::{Dither, Ditherer},
    error::{Error, Result},
    img::Img,
};

#[cfg(feature = "image")]
pub use self::error::IOError;
