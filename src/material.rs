use rand::RngCore;
use rand_distr::{Distribution, UnitSphere};

use crate::math::{Unit3, Vec3, EPSILON};
use crate::scene::HitInfo;

#[derive(Debug, Clone, Copy)]
pub struct ScatteredRay {
    pub dir: Unit3,
    pub attenuation: Vec3,
}

pub trait Material {
    fn scatter(
        &self,
        incoming: Unit3,
        hit: &HitInfo,
        rng: &mut dyn RngCore,
    ) -> Option<ScatteredRay>;
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
    fn scatter(
        &self,
        _incoming: Unit3,
        hit: &HitInfo,
        rng: &mut dyn RngCore,
    ) -> Option<ScatteredRay> {
        let unit: Vec3 = UnitSphere.sample(rng).into();
        let dir = hit.normal.as_ref() + unit;

        Some(ScatteredRay {
            dir: Unit3::try_new(dir, EPSILON).unwrap_or(hit.normal),
            attenuation: self.albedo,
        })
    }
}
