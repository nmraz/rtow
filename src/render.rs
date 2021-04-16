use std::{f64, iter};

use rand::prelude::SliceRandom;
use rand::{Rng, RngCore};
use rand_distr::{Distribution, UnitDisc};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::math::{OrthoNormalBasis, Ray, Unit3, Vec3, EPSILON};
use crate::scene::{PrimitiveHit, Scene};
use crate::shading::ShadingInfo;

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

    u: Unit3,
    v: Unit3,
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

        let (w, focus_dist) = Unit3::new_and_get(opts.origin - opts.look_at);

        let basis = OrthoNormalBasis::from_wv(w, opts.vup);

        let horiz = focus_dist * viewport_width * *basis.u();
        let vert = focus_dist * viewport_height * *basis.v();
        let bottom_left = opts.origin
            - basis.trans_to_canonical(
                focus_dist * Vec3::new(viewport_width / 2., viewport_height / 2., 1.),
            );

        Self {
            origin: opts.origin,
            bottom_left,

            u: basis.u(),
            v: basis.v(),
            horiz,
            vert,

            lens_radius: opts.aperture / 2.,

            pixel_width: opts.pixel_width,
            pixel_height: opts.pixel_height,

            inv_width: 1. / opts.pixel_width as f64,
            inv_height: 1. / opts.pixel_height as f64,
        }
    }

    pub fn cast_ray(&self, pixel_x: u32, pixel_y: u32, rng: &mut dyn RngCore) -> Ray {
        let pixel_x = pixel_x as f64 + rng.gen::<f64>();
        let pixel_y = pixel_y as f64 + rng.gen::<f64>();

        let dof_offset = if self.lens_radius > 0. {
            let [rdx, rdy]: [f64; 2] = UnitDisc.sample(rng);
            self.lens_radius * (rdx * *self.u + rdy * *self.v)
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
            let ray = camera.cast_ray(px, py, &mut rng);
            trace_ray(scene, ray, &mut rng, opts.max_depth)
        })
        .take(opts.samples_per_pixel as usize)
        .sum::<Vec3>()
            / (opts.samples_per_pixel as f64);
    });
}

fn trace_ray(scene: &Scene, mut ray: Ray, rng: &mut dyn RngCore, max_depth: u32) -> Vec3 {
    let mut radiance = Vec3::default();
    let mut throughput = Vec3::from_element(1.);

    for _ in 0..max_depth {
        let hit = match scene.hit(&ray, f64::INFINITY) {
            Some(hit) => hit,
            None => {
                radiance += throughput.component_mul(&sample_background(&ray));
                break;
            }
        };

        let shading_info = hit.shading_info(&ray);

        if !hit.material.is_always_specular() {
            if let Some(direct_radiance) = sample_light(scene, &hit, &shading_info, rng) {
                radiance += throughput.component_mul(&direct_radiance);
            }
        }

        let sample = match hit.material.sample_bsdf(&shading_info, rng) {
            Some(sample) => sample,
            None => break,
        };

        throughput.component_mul_assign(&sample.scaled_color());

        ray = hit.geom_hit.spawn_ray(sample.dir);
    }

    radiance
}

fn sample_light(
    scene: &Scene,
    hit: &PrimitiveHit<'_>,
    shading_info: &ShadingInfo,
    rng: &mut dyn RngCore,
) -> Option<Vec3> {
    let light = scene.lights().choose(rng)?;
    let (sample, dist) = light.sample_incident_at(hit.geom_hit.point, rng)?;

    let occluded = scene
        .hit(&hit.geom_hit.spawn_ray(sample.dir), dist - EPSILON)
        .is_some();

    if !occluded {
        let radiance = sample
            .scaled_color()
            .component_mul(&hit.material.bsdf(shading_info, sample.dir));

        Some(radiance)
    } else {
        None
    }
}

fn sample_background(ray: &Ray) -> Vec3 {
    let t = 0.5 * (ray.dir[1] + 1.);
    (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.)
}
