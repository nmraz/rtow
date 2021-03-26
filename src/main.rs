use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;

use geom::Sphere;
use material::Diffuse;
use math::Vec3;
use render::{Camera, RenderOptions};
use scene::{Primitive, Scene};

mod geom;
mod img;
mod material;
mod math;
mod render;
mod scene;

fn main() -> Result<(), Box<dyn Error>> {
    let gray_material = Arc::new(Diffuse::new(Vec3::new(0.5, 0.5, 0.5)));

    let scene = Scene::with_primitives(vec![
        Primitive::new(
            Sphere::new(Vec3::new(0., 0., -1.), 0.5),
            gray_material.clone(),
        ),
        Primitive::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.), gray_material),
    ]);

    let camera = Camera::new(400, 225);

    let opts = RenderOptions {
        samples_per_pixel: 100,
        max_depth: 50,
    };

    let mut pixels = vec![Vec3::default(); (camera.pixel_width() * camera.pixel_height()) as usize];
    render::render_to(&mut pixels, &scene, &camera, &opts);

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
