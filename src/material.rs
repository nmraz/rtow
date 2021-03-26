use rand::{Rng, RngCore};

use crate::math::{Ray, Unit3, Vec3, EPSILON};
use crate::scene::HitInfo;

#[derive(Debug, Clone, Copy)]
pub struct ScatteredRay {
    pub dir: Unit3,
    pub attenuation: Vec3,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitInfo, rng: &mut dyn RngCore) -> Option<ScatteredRay>;
}

pub struct Diffuse {
    pub albedo: Vec3,
}

impl Diffuse {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Diffuse {
    fn scatter(&self, _ray: &Ray, hit: &HitInfo, rng: &mut dyn RngCore) -> Option<ScatteredRay> {
        let dir = hit.normal.as_ref() + sample_unit_vec(rng);

        Some(ScatteredRay {
            dir: Unit3::try_new(dir, EPSILON).unwrap_or(hit.normal),
            attenuation: self.albedo,
        })
    }
}

fn sample_unit_vec(rng: &mut dyn RngCore) -> Vec3 {
    sample_unit_sphere(rng).normalize()
}

fn sample_unit_sphere(rng: &mut dyn RngCore) -> Vec3 {
    loop {
        let v = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        let norm_squared = v.norm_squared();

        if norm_squared > EPSILON && norm_squared < 1. {
            break v;
        }
    }
}
