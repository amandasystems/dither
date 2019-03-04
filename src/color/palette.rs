pub type Palette = [RGB<u8>];
use super::Error;
use crate::color::{cga, Mode, RGB};
/// parse a palette, specified as 6-digit hexidecimal RGB values (w/ optional 0x prefix) separated by newlines.
/// lines consisting entirely of whitespace or starting with `//` are ignored.
/// /// don't forget to include at least two colors (probably including one of WHITE (0xffffff) or BLACK(0xffffff))
/// ```
/// # use dither::color::palette;
/// # use dither::prelude::*;
/// # use std::collections::HashSet;
/// let input = "
/// // BLACK
/// 0x000000
/// // RED
/// 0xFF0000
/// // GREEN
/// 00ff00
/// ";
/// let want_colors: HashSet<_> = vec![RGB(0, 0, 0), RGB(0xff, 0x00, 0x00), RGB(0x00, 0xff, 0x00)].into_iter().collect();
/// assert_eq!(want_colors,  palette::parse(input).unwrap());
/// ```
pub fn parse<T: std::iter::FromIterator<RGB<u8>>>(s: &str) -> Result<T, Error> {
    use std::str::FromStr;
    let filtered: Vec<&str> = s
        .lines()
        .map(str::trim)
        .filter(|line| {
            !(line.is_empty() || line.starts_with("//") || line.chars().all(char::is_whitespace))
        })
        .collect();
    if filtered.len() <= 2 {
        Err(Error::PaletteTooSmall)
    } else {
        filtered.into_iter().map(RGB::<u8>::from_str).collect()
    }
}

#[test]
fn test_parse() {
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::{Path, PathBuf};
    const GARBAGE: &str = "ASDASLKJAS";

    let tt: Vec<(&str, Mode)> = vec![
        ("bw", Mode::BlackAndWhite),
        ("c", Mode::Color),
        ("color", Mode::Color),
        ("RED", Mode::SingleColor(cga::RED)),
        ("blue", Mode::SingleColor(cga::BLUE)),
        ("LigHT_CYAN", Mode::SingleColor(cga::LIGHT_CYAN)),
        ("cga", Mode::CGA),
        ("cRaYoN", Mode::CRAYON),
    ];
    for (s, want) in tt {
        assert_eq!(s.parse::<Mode>().unwrap(), want);
    }
    assert!(GARBAGE.parse::<Mode>().is_err());

    let mut input = std::env::current_dir().unwrap();
    input.push("temp_cga.plt");

    let mut file = File::create(&input).unwrap();
    write!(
        file,
        "
0x000000
0x0000AA
0x00AA00
0x00AAAA
0xAA00AA
0xAA0000
0xAA5500
0xAAAAAA
0x555555
0x5555FF
0x55FF55
0x55FFFF
0xFF5555
0xFF55FF
0xFFFF55
0xFFFFFF"
    )
    .unwrap();
    let want_palette: HashSet<RGB<u8>> = cga::ALL.iter().cloned().collect();
    if let Mode::Palette {
        palette: got_palette,
        ..
    } = input.to_string_lossy().parse::<Mode>().unwrap()
    {
        assert_eq!(want_palette, got_palette.iter().cloned().collect());
    } else {
        panic!("bad")
    }
    std::fs::remove_file(input).unwrap();
}
/// create a quantization function from the specified palette, returning the pair
/// `(nearest_neighbor, dist_from_neighbor)`
pub fn quantize(palette: &Palette) -> impl Fn(RGB<f64>) -> (RGB<f64>, RGB<f64>) {
    // the naive implementation is faster than using a k-d tree for small palettes;
    // see https://blog.krum.io/k-d-trees/

    let palette = palette.to_vec();
    move |RGB(r0, g0, b0)| {
        let mut min_abs_err = std::f64::INFINITY;
        let (mut nearest_neighbor, mut dist_from_neighbor) = (RGB(0., 0., 0.), RGB(0., 0., 0.));

        for RGB(r1, g1, b1) in palette.iter().cloned().map(RGB::<f64>::from) {
            let abs_err = f64::abs(r0 - r1) + f64::abs(g0 - g1) + f64::abs(b0 - b1);
            if abs_err < min_abs_err {
                dist_from_neighbor = RGB(r0 - r1, g0 - g1, b0 - b1);
                nearest_neighbor = RGB(r1, g1, b1);
                min_abs_err = abs_err;
            }
        }
        (nearest_neighbor, dist_from_neighbor)
    }
}
