use std::io::Write;

use png::{BitDepth, ColorType, Encoder, EncodingError};

use crate::math::Vec3;

fn gamma_correct(v: f64) -> f64 {
    if v <= 0.0031308 {
        12.92 * v
    } else {
        1.055 * v.powf(1. / 2.4) - 0.055
    }
}

fn channel_to_raw(v: f64) -> u8 {
    (gamma_correct(v) * 255. + 0.5).clamp(0., 255.) as u8
}

pub fn pixels_to_srgb(pixels: &[Vec3]) -> Vec<u8> {
    pixels
        .iter()
        .flatten()
        .map(|&v| channel_to_raw(v))
        .collect()
}

pub fn write_png<W: Write>(
    writer: &mut W,
    raw_pixels: &[u8],
    width: u32,
    height: u32,
) -> Result<(), EncodingError> {
    assert_eq!(raw_pixels.len(), (width * height * 3) as usize);

    let mut enc = Encoder::new(writer, width, height);
    enc.set_color(ColorType::RGB);
    enc.set_depth(BitDepth::Eight);

    enc.write_header()?.write_image_data(raw_pixels)
}
