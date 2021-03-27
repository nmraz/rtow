use std::convert::TryInto;
use std::{f64, iter};

use rand::{Rng, RngCore};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::math::{Ray, Vec3};
use crate::scene::Scene;

pub struct CameraOptions {
    pub pixel_width: u32,
    pub pixel_height: u32,
    pub vert_fov: f64,

    pub origin: Vec3,
    pub look_at: Vec3,
    pub vup: Vec3,
}

pub struct Camera {
    origin: Vec3,
    bottom_left: Vec3,

    horiz: Vec3,
    vert: Vec3,

    pixel_width: u32,
    pixel_height: u32,
}

impl Camera {
    pub fn new(opts: &CameraOptions) -> Self {
        let aspect_ratio = opts.pixel_width as f64 / opts.pixel_height as f64;
        let focal_length = 1.;

        let viewport_height = 2. * focal_length * (opts.vert_fov * f64::consts::PI / 360.).tan();
        let viewport_width = aspect_ratio * viewport_height;

        // Note: right-handed coordinate system
        let w = (opts.origin - opts.look_at).normalize();
        let u = opts.vup.cross(&w).normalize();
        let v = w.cross(&u);

        let horiz = viewport_width * u;
        let vert = viewport_height * v;
        let bottom_left = opts.origin - horiz / 2. - vert / 2. - Vec3::new(0., 0., focal_length);

        Self {
            origin: opts.origin,
            bottom_left,
            horiz,
            vert,

            pixel_width: opts.pixel_width,
            pixel_height: opts.pixel_height,
        }
    }

    pub fn cast_ray(&self, pixel_x: f64, pixel_y: f64) -> Ray {
        let u = pixel_x / self.pixel_width as f64;
        let v = 1. - pixel_y / self.pixel_height as f64;

        Ray::pointing_through(
            self.origin,
            self.bottom_left + u * self.horiz + v * self.vert,
        )
    }

    pub fn pixel_width(&self) -> u32 {
        self.pixel_width
    }

    pub fn pixel_height(&self) -> u32 {
        self.pixel_height
    }
}

pub struct RenderOptions {
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}

pub fn render_to(buf: &mut [Vec3], scene: &Scene, camera: &Camera, opts: &RenderOptions) {
    let pixel_height = camera.pixel_height();
    let pixel_width = camera.pixel_width();

    assert_eq!(buf.len(), (pixel_width * pixel_height) as usize);

    let max_depth = opts.max_depth.try_into().unwrap();

    buf.par_iter_mut().enumerate().for_each(|(idx, pixel)| {
        let idx = idx as u32;

        let px = idx % pixel_width;
        let py = idx / pixel_width;

        let mut rng = rand::thread_rng();

        *pixel = iter::repeat_with(|| {
            let ray = camera.cast_ray(px as f64 + rng.gen::<f64>(), py as f64 + rng.gen::<f64>());
            trace_ray(scene, &ray, &mut rng, max_depth)
        })
        .take(opts.samples_per_pixel as usize)
        .sum::<Vec3>()
            / (opts.samples_per_pixel as f64);
    });
}

fn trace_ray(scene: &Scene, ray: &Ray, rng: &mut dyn RngCore, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::default();
    }

    if let Some((hit, material)) = scene.hit(ray) {
        return material
            .scatter(ray.dir, &hit, rng)
            .map(|scattered| {
                scattered.attenuation.component_mul(&trace_ray(
                    scene,
                    &Ray {
                        origin: hit.point,
                        dir: scattered.dir,
                    },
                    rng,
                    depth - 1,
                ))
            })
            .unwrap_or_default();
    }

    let t = 0.5 * (ray.dir[1] + 1.);
    (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.)
}
