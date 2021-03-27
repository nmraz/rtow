use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use structopt::StructOpt;

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

#[derive(StructOpt)]
struct CliArgs {
    /// Width of rendered image, in pixels
    #[structopt(long, short)]
    pub width: u32,

    /// Height of rendered image, in pixels
    #[structopt(long, short)]
    pub height: u32,

    /// Maximum bounce depth
    #[structopt(long, default_value = "10")]
    pub max_depth: u32,

    /// Number of samples to gather per pixel
    #[structopt(long = "spp", default_value = "100")]
    pub samples_per_pixel: u32,

    /// Output filename
    #[structopt(short, default_value = "render.png")]
    pub output_filename: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::from_args();

    let scene = build_scene();
    let camera = Camera::new(args.width, args.height);

    let opts = RenderOptions {
        samples_per_pixel: args.samples_per_pixel,
        max_depth: args.max_depth,
    };

    println!(
        "Rendering {} at {}×{}, {}spp, depth {}",
        args.output_filename.display(),
        args.width,
        args.height,
        args.samples_per_pixel,
        args.max_depth
    );

    let start_time = Instant::now();

    let mut pixels = vec![Vec3::default(); (camera.pixel_width() * camera.pixel_height()) as usize];
    render::render_to(&mut pixels, &scene, &camera, &opts);

    let elapsed = Instant::now() - start_time;
    println!("Rendered in {}s", elapsed.as_secs_f64());

    let raw_pixels = img::pixels_to_raw_rgb(&pixels);
    let mut writer = BufWriter::new(File::create(args.output_filename)?);
    img::write_png(
        &mut writer,
        &raw_pixels,
        camera.pixel_width(),
        camera.pixel_height(),
    )?;

    Ok(())
}

fn build_scene() -> Scene {
    let ground_material = Arc::new(Diffuse::new(Vec3::new(0.5, 0.5, 0.5)));
    let pink_material = Arc::new(Diffuse::new(Vec3::new(1., 0.2, 0.2)));
    let gold_material = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.95));
    let diamond_material = Arc::new(Dielectric::new(2.4));

    Scene::with_primitives(vec![
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
    ])
}
