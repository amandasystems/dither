use super::Result;
use super::RGB;
use std::path::Path;
/// A rectangular image on N pixels. Essentially a wrapper around the raw pixel buffer with some helper methods to access by `(x, y)` pair.
#[derive(Clone, Debug, PartialEq)]
pub struct Img<P> {
    buf: Vec<P>,
    width: u32,
}

impl<P> Img<P> {
    /// create an Img<P> from a buf and width. fails if buf.len() is not an exact multiple of width.
    pub fn new(buf: impl IntoIterator<Item = P>, width: u32) -> Option<Self> {
        let buf: Vec<P> = buf.into_iter().collect();
        if width == 0 || buf.len() % width as usize != 0 {
            None
        } else {
            Some(Img { buf, width })
        }
    }
    /// create an Img<P> from a buf and length directly, skipping the bounds check.
    pub const unsafe fn from_raw_buf(buf: Vec<P>, width: u32) -> Self {
        Img { buf, width }
    }

    /// pull the buffer out of the image as a vec
    pub fn into_vec(self) -> Vec<P> {
        self.buf
    }

    /// the height of the image; i.e, `buf.len() / width`
    pub fn height(&self) -> u32 {
        self.len() as u32 / self.width
    }
    /// get the width of the image
    pub fn width(&self) -> u32 {
        self.width
    }
    /// returns an iterator over the pixels in the buffer
    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
    /// returns an iterator that allows modifying each pixel
    pub fn iter_mut(&mut self) -> <&mut Self as IntoIterator>::IntoIter {
        self.buf.iter_mut()
    }

    /// Returns a reference to an element at `(x, y)`.
    pub fn get(&self, (x, y): (u32, u32)) -> Option<&P> {
        self.buf.get(self.idx((x, y)))
    }
    /// get mutable access to the pixel at `(x, y)`;
    pub fn get_mut(&mut self, i: (u32, u32)) -> Option<&mut P> {
        let i = self.idx(i);
        self.buf.get_mut(i)
    }

    /// map a function on P across the image buffer, converting an `Img<P>` to an `Img<Q>`
    pub fn convert_with<Q>(self, convert: impl Fn(P) -> Q) -> Img<Q> {
        let Img { buf, width } = self;
        Img {
            buf: buf.into_iter().map(convert).collect(),
            width,
        }
    }
    #[inline]
    fn idx(&self, (x, y): (u32, u32)) -> usize {
        (y.wrapping_mul(self.width).wrapping_add(x)) as usize
    }
    /// Returns the number of pixels in the image.
    pub fn len(&self) -> usize {
        self.buf.len()
    }
    /// Returns true if the image is not empty. Should always be true.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a pair `(width, height)`.
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height())
    }
}

impl<N: From<u8>> Img<RGB<N>> {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let img = image::open(path)?.to_rgb();

        Ok(Img {
            buf: img.pixels().map(|p| RGB::from(p.data)).collect(),
            width: img.width(),
        })
    }
}

impl Img<RGB<u8>> {
    /// save an image as a `.png` or `.jpg` to the path. the path extension determines the image type
    pub fn save<P: AsRef<Path>>(self, path: P) -> Result<()> {
        let height = self.buf.len() as u32 / self.width;
        let buf = image::RgbImage::from_raw(self.width, height, self.raw_buf()).unwrap();
        buf.save(path)?;
        Ok(())
    }
    /// the raw_buf flattens out each RGB triplet;
    /// ```
    /// use dither::prelude::*;
    /// let img: Img<RGB<u8>> = Img::new(vec![RGB(0, 1, 2), RGB(1, 1, 1)], 1).unwrap();
    /// assert_eq!(img.raw_buf(), vec![0, 1, 2, 1, 1, 1]);
    /// ```
    pub fn raw_buf(self) -> Vec<u8> {
        let mut raw_buf = Vec::with_capacity(self.len() * 3);
        for RGB(r, g, b) in self.buf {
            raw_buf.push(r);
            raw_buf.push(g);
            raw_buf.push(b);
        }
        raw_buf
    }
}

impl<P> std::ops::Index<(u32, u32)> for Img<P> {
    type Output = P;
    fn index(&self, (x, y): (u32, u32)) -> &P {
        &self.buf[self.idx((x, y))]
    }
}

impl<P> std::ops::IndexMut<(u32, u32)> for Img<P> {
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut P {
        let i = self.idx((x, y));
        &mut self.buf[i]
    }
}

impl<P> IntoIterator for Img<P> {
    type Item = P;
    type IntoIter = std::vec::IntoIter<P>;
    fn into_iter(self) -> Self::IntoIter {
        self.buf.into_iter()
    }
}

impl<'a, P> IntoIterator for &'a Img<P> {
    type Item = &'a P;
    type IntoIter = std::slice::Iter<'a, P>;
    fn into_iter(self) -> Self::IntoIter {
        (&self.buf).iter()
    }
}

impl<'a, P> IntoIterator for &'a mut Img<P> {
    type Item = &'a mut P;
    type IntoIter = std::slice::IterMut<'a, P>;
    fn into_iter(self) -> Self::IntoIter {
        (&mut self.buf).iter_mut()
    }
}
