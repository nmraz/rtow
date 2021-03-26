use std::iter;

use rand::Rng;

use crate::math::{Ray, Vec3};
use crate::scene::Scene;

pub struct Camera {
    origin: Vec3,
    bottom_left: Vec3,

    horiz: Vec3,
    vert: Vec3,

    pixel_width: u32,
    pixel_height: u32,
}

impl Camera {
    pub fn new(pixel_width: u32, pixel_height: u32) -> Self {
        let aspect_ratio = pixel_width as f64 / pixel_height as f64;

        let viewport_height = 2.;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.;

        let origin = Vec3::default();

        let horiz = Vec3::new(viewport_width, 0., 0.);
        let vert = Vec3::new(0., viewport_height, 0.);
        let bottom_left = origin - horiz / 2. - vert / 2. - Vec3::new(0., 0., focal_length);

        Self {
            origin,
            bottom_left,
            horiz,
            vert,

            pixel_width,
            pixel_height,
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
}

pub fn render_to(buf: &mut [Vec3], scene: &Scene, camera: &Camera, opts: &RenderOptions) {
    let pixel_height = camera.pixel_height();
    let pixel_width = camera.pixel_width();

    assert_eq!(buf.len(), (pixel_width * pixel_height) as usize);

    let mut rng = rand::thread_rng();

    for py in 0..pixel_height {
        for px in 0..pixel_width {
            let color = iter::repeat_with(|| {
                let ray =
                    camera.cast_ray(px as f64 + rng.gen::<f64>(), py as f64 + rng.gen::<f64>());
                ray_color(&ray, scene)
            })
            .take(opts.samples_per_pixel as usize)
            .sum::<Vec3>()
                / (opts.samples_per_pixel as f64);

            buf[(py * pixel_width + px) as usize] = color;
        }
    }
}

fn ray_color(ray: &Ray, scene: &Scene) -> Vec3 {
    if let Some(info) = scene.hit(ray) {
        return 0.5 * (info.normal.as_ref() + Vec3::new(1., 1., 1.));
    }

    let t = 0.5 * (ray.dir[1] + 1.);
    (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.)
}
