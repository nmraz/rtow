use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

use math::{Ray, Unit3, Vec3};

mod img;
mod math;

fn main() -> Result<(), Box<dyn Error>> {
    let aspect_ratio = 16. / 9.;

    let img_width = 400;
    let img_height = (img_width as f64 / aspect_ratio) as _;

    let viewport_height = 2.;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.;

    let origin = Vec3::default();

    let horiz = Vec3::new(viewport_width, 0., 0.);
    let vert = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner = origin - horiz / 2. - vert / 2. - Vec3::new(0., 0., focal_length);

    let mut pixels = Vec::with_capacity((img_width * img_height) as usize);

    for j in (0..img_height).rev() {
        for i in 0..img_width {
            let u = (i as f64) / (img_width as f64 - 1.);
            let v = (j as f64) / (img_height as f64 - 1.);

            let ray = Ray {
                origin,
                dir: Unit3::new_normalize(lower_left_corner + u * horiz + v * vert - origin),
            };

            pixels.push(ray_color(&ray));
        }
    }

    let raw_pixels = img::pixels_to_raw_rgb(&pixels);

    let mut writer = BufWriter::new(File::create("render.png")?);
    img::write_png(&mut writer, &raw_pixels, img_width, img_height)?;

    Ok(())
}

fn ray_color(ray: &Ray) -> Vec3 {
    let center = Vec3::new(0., 0., -1.);

    if let Some(t) = hit_sphere(center, 0.5, ray) {
        let n = (ray.at(t) - center).normalize();
        return 0.5 * (n + Vec3::new(1., 1., 1.));
    }

    let t = 0.5 * (ray.dir[1] + 1.);
    (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.)
}

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = ray.origin - center;
    let b = 2. * oc.dot(ray.dir.as_ref());
    let c = oc.norm_squared() - radius * radius;

    let discriminant = b * b - 4. * c;

    if discriminant >= 0. {
        Some((-b - discriminant.sqrt()) / 2.)
    } else {
        None
    }
}
