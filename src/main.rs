use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;

use geom::Sphere;
use material::{Dielectric, Diffuse, Metal};
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
    let ground_material = Arc::new(Diffuse::new(Vec3::new(0.5, 0.5, 0.5)));
    let pink_material = Arc::new(Diffuse::new(Vec3::new(1., 0.2, 0.2)));
    let gold_material = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.95));
    let diamond_material = Arc::new(Dielectric::new(2.4));

    let scene = Scene::with_primitives(vec![
        Primitive::new(Sphere::new(Vec3::new(-0.5, 0., -1.), 0.5), pink_material),
        Primitive::new(Sphere::new(Vec3::new(0.5, 0., -1.), 0.5), gold_material),
        Primitive::new(
            Sphere::new(Vec3::new(0., -0.15, -0.5), 0.1),
            diamond_material,
        ),
        Primitive::new(
            Sphere::new(Vec3::new(0., -100.5, -1.), 100.),
            ground_material,
        ),
    ]);

    let camera = Camera::new(960, 540);

    let opts = RenderOptions {
        samples_per_pixel: 1000,
        max_depth: 20,
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
