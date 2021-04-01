use std::{f64, iter};

use rand::{Rng, RngCore};
use rand_distr::{Distribution, UnitDisc};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::math::{OrthoNormalBasis, Ray, Unit3, Vec3};
use crate::scene::Scene;

pub struct CameraOptions {
    pub pixel_width: u32,
    pub pixel_height: u32,

    pub vert_fov: f64,
    pub aperture: f64,

    pub origin: Vec3,
    pub look_at: Vec3,
    pub vup: Vec3,
}

pub struct Camera {
    origin: Vec3,
    bottom_left: Vec3,

    u: Vec3,
    v: Vec3,
    horiz: Vec3,
    vert: Vec3,

    lens_radius: f64,

    pixel_width: u32,
    pixel_height: u32,

    inv_width: f64,
    inv_height: f64,
}

impl Camera {
    pub fn new(opts: &CameraOptions) -> Self {
        let aspect_ratio = opts.pixel_width as f64 / opts.pixel_height as f64;

        let viewport_height = 2. * (opts.vert_fov * f64::consts::PI / 360.).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let (w, focus_dist) = {
            let mut w = opts.origin - opts.look_at;
            let dist = w.normalize_mut();

            (w, dist)
        };

        // Note: right-handed coordinate system
        let u = opts.vup.cross(&w).normalize();
        let v = w.cross(&u);

        let horiz = focus_dist * viewport_width * u;
        let vert = focus_dist * viewport_height * v;
        let bottom_left = opts.origin - horiz / 2. - vert / 2. - focus_dist * w;

        Self {
            origin: opts.origin,
            bottom_left,

            u,
            v,
            horiz,
            vert,

            lens_radius: opts.aperture / 2.,

            pixel_width: opts.pixel_width,
            pixel_height: opts.pixel_height,

            inv_width: 1. / opts.pixel_width as f64,
            inv_height: 1. / opts.pixel_height as f64,
        }
    }

    pub fn cast_ray(&self, pixel_x: f64, pixel_y: f64, rng: &mut dyn RngCore) -> Ray {
        let dof_offset = if self.lens_radius > 0. {
            let [rdx, rdy]: [f64; 2] = UnitDisc.sample(rng);
            self.lens_radius * (rdx * self.u + rdy * self.v)
        } else {
            Vec3::default()
        };

        let u = pixel_x * self.inv_width;
        let v = 1. - pixel_y * self.inv_height;

        Ray::pointing_through(
            self.origin + dof_offset,
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

    buf.par_iter_mut().enumerate().for_each(|(idx, pixel)| {
        let idx = idx as u32;

        let px = idx % pixel_width;
        let py = idx / pixel_width;

        let mut rng = rand::thread_rng();

        *pixel = iter::repeat_with(|| {
            let ray = camera.cast_ray(
                px as f64 + rng.gen::<f64>(),
                py as f64 + rng.gen::<f64>(),
                &mut rng,
            );
            trace_ray(scene, ray, &mut rng, opts.max_depth)
        })
        .take(opts.samples_per_pixel as usize)
        .sum::<Vec3>()
            / (opts.samples_per_pixel as f64);
    });
}

fn trace_ray(scene: &Scene, mut ray: Ray, rng: &mut dyn RngCore, max_depth: u32) -> Vec3 {
    let mut color = Vec3::from_element(1.);

    for _ in 0..max_depth {
        let (hit, material) = match scene.hit(&ray) {
            Some(data) => data,
            None => return color.component_mul(&sample_background(&ray)),
        };

        let basis = OrthoNormalBasis::from_w(hit.normal);
        let incoming = Unit3::new_unchecked(-basis.trans_from_canonical(*ray.dir));

        let scattered = match material.scatter(incoming, hit.side, rng) {
            Some(scattered) => scattered,
            None => return Vec3::default(),
        };

        color.component_mul_assign(&scattered.attenuation);
        ray = Ray {
            origin: hit.point,
            dir: Unit3::new_unchecked(basis.trans_to_canonical(*scattered.dir)),
        }
    }

    Vec3::default()
}

fn sample_background(ray: &Ray) -> Vec3 {
    let t = 0.5 * (ray.dir[1] + 1.);
    (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.)
}
