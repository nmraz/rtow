use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use rand::{Rng, RngCore, SeedableRng};
use rand_pcg::Pcg64;
use structopt::StructOpt;

use geom::Sphere;
use material::{Dielectric, Diffuse, Material, Metal};
use math::Vec3;
use render::{Camera, CameraOptions, RenderOptions};
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

    /// Seed to use when generating the scene. If this is not provided, a random seed will be used.
    #[structopt(long)]
    pub scene_seed: Option<u64>,

    /// Vertical field of view, in degrees
    #[structopt(long, default_value = "50")]
    pub vfov: f64,

    /// Width of the camera aperture. Specify 0 for a pinhole camera.
    #[structopt(long, default_value = "0")]
    pub aperture: f64,

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

    let scene_seed = args
        .scene_seed
        .unwrap_or_else(|| rand::thread_rng().next_u64());

    println!("Scene seed: {}", scene_seed);

    let mut scene_rng = Pcg64::seed_from_u64(scene_seed);
    let scene = build_scene(&mut scene_rng);

    let camera_opts = CameraOptions {
        pixel_width: args.width,
        pixel_height: args.height,

        vert_fov: args.vfov,
        aperture: args.aperture,

        origin: Vec3::new(12., 2., 3.),
        look_at: Vec3::new(3.3, 0.5, 0.7),
        vup: Vec3::new(0., 1., 0.),
    };

    let camera = Camera::new(&camera_opts);

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

    let raw_pixels = img::pixels_to_srgb(&pixels);
    let mut writer = BufWriter::new(File::create(args.output_filename)?);
    img::write_png(
        &mut writer,
        &raw_pixels,
        camera.pixel_width(),
        camera.pixel_height(),
    )?;

    Ok(())
}

fn build_scene(rng: &mut impl Rng) -> Scene {
    const RANGE: i32 = 11;

    let ground_material = Arc::new(Diffuse::new(Vec3::new(0.5, 0.5, 0.5)));
    let glass_material = Arc::new(Dielectric::new(1.5));

    let mut primitives = vec![
        Primitive::new(
            Sphere::new(Vec3::new(0., -1000., 0.), 1000.),
            ground_material,
        ),
        Primitive::new(
            Sphere::new(Vec3::new(0., 1., 0.), 1.),
            glass_material.clone(),
        ),
        Primitive::new(
            Sphere::new(Vec3::new(-4., 1., 0.), 1.),
            Arc::new(Diffuse::new(Vec3::new(0.4, 0.2, 0.1))),
        ),
        Primitive::new(
            Sphere::new(Vec3::new(4., 1., 0.), 1.),
            Arc::new(Metal::new(Vec3::new(0.5, 0.6, 0.7), 1.)),
        ),
    ];

    for a in -RANGE..RANGE {
        for b in -RANGE..RANGE {
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Vec3::new(4., 0.2, 0.)).norm() <= 0.9 {
                continue;
            }

            let material_kind: f64 = rng.gen();

            let material: Arc<dyn Material + Send + Sync> = if material_kind < 0.75 {
                Arc::new(Diffuse::new(Vec3::new(rng.gen(), rng.gen(), rng.gen())))
            } else if material_kind < 0.95 {
                let albedo = Vec3::new(
                    rng.gen_range(0.5..1.),
                    rng.gen_range(0.5..1.),
                    rng.gen_range(0.5..1.),
                );

                let gloss = rng.gen_range(0.5..1.);

                Arc::new(Metal::new(albedo, gloss))
            } else {
                glass_material.clone()
            };

            primitives.push(Primitive::new(Sphere::new(center, 0.2), material));
        }
    }

    Scene::with_primitives(primitives)
}
