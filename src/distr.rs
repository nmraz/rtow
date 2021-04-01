use std::f64;

use rand::Rng;
use rand_distr::Distribution;

use crate::math::{Unit3, Vec3};

pub struct CosWeightedHemisphere;

impl Distribution<Unit3> for CosWeightedHemisphere {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Unit3 {
        let radius_squared: f64 = rng.gen();
        let phi = rng.gen_range(0.0..f64::consts::TAU);

        let radius = radius_squared.sqrt();
        Unit3::new_unchecked(Vec3::new(
            radius * phi.cos(),
            radius * phi.sin(),
            (1. - radius_squared).sqrt(),
        ))
    }
}
