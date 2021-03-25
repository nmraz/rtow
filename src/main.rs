use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

use math::Vec3;

mod img;
mod math;

fn main() -> Result<(), Box<dyn Error>> {
    let width = 256;
    let height = 256;

    let mut pixels = Vec::with_capacity((width * height) as usize);

    for j in (0..height).rev() {
        for i in 0..width {
            let r = (i as f64) / (width as f64 - 1.);
            let g = (j as f64) / (width as f64 - 1.);
            let b = 0.25;

            pixels.push(Vec3::new(r, g, b));
        }
    }

    let raw_pixels = img::pixels_to_raw_rgb(&pixels);

    let mut writer = BufWriter::new(File::create("render.png")?);
    img::write_png(&mut writer, &raw_pixels, width, height)?;

    Ok(())
}
