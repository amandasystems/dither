use crate::prelude::*;
use rand::prelude::*;

#[test]
fn test_save_and_load() {
    let img = load_test_image();
    let mut output = std::env::current_dir().unwrap();

    output.push("save_load_test.png");
    img.clone().save(&output).unwrap();

    assert_eq!(img, Img::load(&output).unwrap());
    std::fs::remove_file(output).unwrap();
}

fn load_test_image() -> Img<RGB<u8>> {
    let mut input = std::env::current_dir().unwrap();
    input.push("bunny.png");

    Img::load(input).unwrap()
}
/// No Op ditherer; doesn't do anything;

#[test]
fn test_dither_no_op() {
    const NO_OP_DITHER: Ditherer = Ditherer::new(1., &[]);

    fn no_op(p: RGB<f64>) -> (RGB<f64>, RGB<f64>) {
        (p, RGB::default())
    }
    let mut test_img_buf: Vec<RGB<f64>> = Vec::with_capacity(9);
    for y in 0..3 {
        for x in 0..3 {
            test_img_buf.push(RGB(0., f64::from(y), f64::from(x)));
        }
    }
    let img = Img::new(test_img_buf, 3).unwrap();
    assert_eq!(img, NO_OP_DITHER.dither(img.clone(), no_op));
}

#[test]
fn test_quantize() {
    const TOL: f64 = std::f64::EPSILON;
    let uniform = rand::distributions::Uniform::from((0.)..(255.));
    let mut rng = rand::thread_rng();
    let mut q = create_quantize_n_bits_func(1).unwrap();
    for _ in 0..20 {
        let n = uniform.sample(&mut rng);
        let (want_q, want_r) = quantize_1_bit(n);
        let (got_q, got_r) = q(n);
        assert!((got_q - want_q).abs() < TOL && (got_r - want_r).abs() < 1. + TOL);
    }
}

fn quantize_1_bit(x: f64) -> (f64, f64) {
    if x < 128. {
        (0., x)
    } else {
        (255., x - 255.)
    }
}
