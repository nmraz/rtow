use rand::RngCore;

use crate::geom::HitInfo;
use crate::math::{Ray, Unit3, Vec3};
use crate::shading::SampledRadiance;

#[derive(Debug, Clone, Copy)]
pub struct SampledLightRadiance {
    pub radiance: SampledRadiance,
    pub t: f64,
}

impl SampledLightRadiance {
    pub fn new(radiance: SampledRadiance, t: f64) -> Self {
        Self { radiance, t }
    }
}

pub struct EmittedRadiance {
    pub color: Vec3,
    pub t: f64,
}

impl EmittedRadiance {
    pub fn new(color: Vec3, t: f64) -> Self {
        Self { color, t }
    }
}

pub trait Light {
    fn sample_incident_at(
        &self,
        hit: &HitInfo,
        rng: &mut dyn RngCore,
    ) -> Option<SampledLightRadiance>;
    fn pdf(&self, hit: &HitInfo, local_dir: Unit3) -> f64;

    fn emitted(&self, ray: &Ray) -> Option<EmittedRadiance>;
}

pub struct PointLight {
    point: Vec3,
    color: Vec3,
}

impl PointLight {
    pub fn new(point: Vec3, color: Vec3) -> Self {
        Self { point, color }
    }
}

impl Light for PointLight {
    fn sample_incident_at(
        &self,
        hit: &HitInfo,
        _rng: &mut dyn RngCore,
    ) -> Option<SampledLightRadiance> {
        let (dir, t) = Unit3::new_and_get(self.point - hit.point);
        Some(SampledLightRadiance::new(
            SampledRadiance::new_delta(hit.world_to_local(dir), self.color / t.powi(2)),
            t,
        ))
    }

    fn pdf(&self, _hit: &HitInfo, _local_dir: Unit3) -> f64 {
        0.
    }

    fn emitted(&self, _ray: &Ray) -> Option<EmittedRadiance> {
        None
    }
}
