use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

mod img;

fn main() -> Result<(), Box<dyn Error>> {
    let width = 256;
    let height = 256;

    let mut pixels = Vec::with_capacity((width * height * 3) as usize);

    for j in (0..height).rev() {
        for i in 0..width {
            let r = (i as f64) / (width as f64 - 1.);
            let g = (j as f64) / (width as f64 - 1.);
            let b = 0.25;

            pixels.push((r * 255.999) as u8);
            pixels.push((g * 255.999) as u8);
            pixels.push((b * 255.999) as u8);
        }
    }

    let mut writer = BufWriter::new(File::create("render.png")?);
    img::write_png(&mut writer, &pixels, width, height)?;

    Ok(())
}
