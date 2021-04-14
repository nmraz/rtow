use std::f64;

use rand::{Rng, RngCore};
use rand_distr::{Distribution, UnitSphere};

use crate::distr::CosWeightedHemisphere;
use crate::geom::HitSide;
use crate::math::{Unit3, Vec3};
use crate::shading::{SampledRadiance, ShadingInfo};

pub trait Material {
    fn sample_bsdf(&self, shading: &ShadingInfo, rng: &mut dyn RngCore) -> Option<SampledRadiance>;
}

pub struct Diffuse {
    albedo: Vec3,
}

impl Diffuse {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Diffuse {
    fn sample_bsdf(&self, shading: &ShadingInfo, rng: &mut dyn RngCore) -> Option<SampledRadiance> {
        Some(SampledRadiance::new_real(
            CosWeightedHemisphere.sample(rng),
            self.albedo * f64::consts::FRAC_1_PI,
            shading.cos_theta() * f64::consts::FRAC_1_PI,
        ))
    }
}

pub struct Metal {
    albedo: Vec3,
    gloss: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, gloss: f64) -> Self {
        Self {
            albedo,
            gloss: gloss.clamp(0., 1.),
        }
    }
}

impl Material for Metal {
    fn sample_bsdf(&self, shading: &ShadingInfo, rng: &mut dyn RngCore) -> Option<SampledRadiance> {
        let reflected = reflect_z(*shading.incoming);
        let dir = Unit3::new_normalize(
            reflected + (1. - self.gloss) * Vec3::from(UnitSphere.sample(rng)),
        );

        let cos_theta = shading.cos_theta();

        if cos_theta > 0. {
            Some(SampledRadiance::new_specular(
                dir,
                self.albedo.map(|r0| schlick_reflectance(r0, cos_theta)) / shading.cos_theta(),
            ))
        } else {
            None
        }
    }
}

fn reflect_z(incoming: Vec3) -> Vec3 {
    Vec3::new(-incoming[0], -incoming[1], incoming[2])
}

fn schlick_reflectance(r0: f64, cos_theta: f64) -> f64 {
    r0 + (1. - r0) * (1. - cos_theta).powi(5)
}

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }
}

impl Material for Dielectric {
    fn sample_bsdf(&self, shading: &ShadingInfo, rng: &mut dyn RngCore) -> Option<SampledRadiance> {
        let refractive_ratio = match shading.side {
            HitSide::Inside => self.refractive_index,
            HitSide::Outside => 1. / self.refractive_index,
        };

        let incoming = *shading.incoming;
        let cos_theta = shading.cos_theta();
        let sin_theta = shading.sin_theta();

        let (dir, attenuation) = if refractive_ratio * sin_theta > 1.
            || rng.gen::<f64>() < dielectric_reflectance(cos_theta, refractive_ratio)
        {
            (reflect_z(incoming), 1.)
        } else {
            let up = *Vec3::z_axis();

            let refracted_perp = refractive_ratio * (cos_theta * up - incoming);
            let refracted_par = -(1. - refracted_perp.norm_squared()).sqrt() * up;

            (
                refracted_perp + refracted_par,
                refractive_ratio * refractive_ratio,
            )
        };

        Some(SampledRadiance::new_specular(
            Unit3::new_normalize(dir),
            Vec3::from_element(attenuation) / cos_theta,
        ))
    }
}

fn dielectric_reflectance(cos_theta: f64, refractive_ratio: f64) -> f64 {
    let r0 = ((1. - refractive_ratio) / (1. + refractive_ratio)).powi(2);
    schlick_reflectance(r0, cos_theta)
}
