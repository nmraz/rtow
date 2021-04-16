use rand::RngCore;

use crate::geom::HitInfo;
use crate::math::{Unit3, Vec3};
use crate::shading::SampledRadiance;

pub trait Light {
    fn sample_incident_at(
        &self,
        hit: &HitInfo,
        rng: &mut dyn RngCore,
    ) -> Option<(SampledRadiance, f64)>;
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
    ) -> Option<(SampledRadiance, f64)> {
        let (dir, dist) = Unit3::new_and_get(self.point - hit.point);
        Some((
            SampledRadiance::new_specular(dir, self.color / dist.powi(2)),
            dist,
        ))
    }
}
