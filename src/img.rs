use super::Result;
use super::RGB;
use std::path::Path;
/// A rectangular image on N pixels.
#[derive(Clone, Debug, PartialEq)]
pub struct Img<P> {
    pub buf: Vec<P>,
    pub width: u32,
}

impl<P> Img<P> {
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
        ((y * self.width) + x) as usize
    }
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a reference to an element.
    pub fn get(&self, (x, y): (u32, u32)) -> Option<&P> {
        self.buf.get(self.idx((x, y)))
    }
    /// Returns a pair `(width, height)`.
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.len() as u32 / self.width as u32)
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
    /// use dither::img::*;
    /// let img: Img<RGB<u8>> = Img{buf: vec![RGB(0, 1, 2), RGB(1, 1, 1)], width: 1};
    /// assert_eq!(img.raw_buf(), vec![0, 1, 2, 1, 1, 1]);
    /// ```
    fn raw_buf(self) -> Vec<u8> {
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
