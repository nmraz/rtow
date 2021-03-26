use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

use geom::Sphere;
use math::Vec3;
use render::Camera;
use scene::Scene;

mod geom;
mod img;
mod math;
mod render;
mod scene;

fn main() -> Result<(), Box<dyn Error>> {
    let scene = Scene::with_primitives(vec![
        Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)),
        Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)),
    ]);

    let camera = Camera::new(400, 225);

    let mut pixels = vec![Vec3::default(); (camera.pixel_width() * camera.pixel_height()) as usize];
    render::render_to(&mut pixels, &scene, &camera);

    let raw_pixels = img::pixels_to_raw_rgb(&pixels);

    let mut writer = BufWriter::new(File::create("render.png")?);
    img::write_png(
        &mut writer,
        &raw_pixels,
        camera.pixel_width(),
        camera.pixel_height(),
    )?;

    Ok(())
}
