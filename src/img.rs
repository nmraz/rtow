use std::io::Write;

use png::{BitDepth, ColorType, Encoder, EncodingError};

use crate::math::Vec3;

fn channel_to_raw(channel: f64) -> u8 {
    (channel.clamp(0., 1.) * 255.) as u8
}

pub fn pixels_to_raw_rgb(pixels: &[Vec3]) -> Vec<u8> {
    pixels
        .iter()
        .flatten()
        .map(|&val| channel_to_raw(val))
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
