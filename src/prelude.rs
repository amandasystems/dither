//! Re-exports of the most common traits and types.
pub use super::*;

pub use self::{
    color::{CGA, RGB},
    ditherer::{Dither, Ditherer},
    error::{Error, Result},
    img::Img,
    opts::Opt,
};
