use std::array::IntoIter;
use std::io::Write;

use png::{BitDepth, ColorType, Encoder, EncodingError};

use crate::math::Vec3;

fn luminance(color: &Vec3) -> f64 {
    0.2126 * color[0] + 0.7152 * color[1] + 0.0722 * color[2]
}

fn tone_map(color: &Vec3, max_y: f64) -> Vec3 {
    let y = luminance(color);
    let scale = (1. + y / max_y.powi(2)) / (1. + y);

    scale * color
}

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
    let max_y = pixels
        .iter()
        .map(luminance)
        .max_by(|y1, y2| y1.partial_cmp(y2).unwrap())
        .unwrap_or(1.);

    pixels
        .iter()
        .map(|color| tone_map(color, max_y))
        .flat_map(|color| {
            let vals: [_; 3] = color.into();
            IntoIter::new(vals)
        })
        .map(channel_to_raw)
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
