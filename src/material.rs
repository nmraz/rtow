use std::f64;

use rand::{Rng, RngCore};
use rand_distr::Distribution;

use crate::distr::CosWeightedHemisphere;
use crate::geom::HitSide;
use crate::math::{Unit3, Vec3};
use crate::shading::{self, same_hemisphere, SampledRadiance, ShadingInfo};

pub trait Material {
    fn sample_bsdf(
        &self,
        shading_info: &ShadingInfo,
        rng: &mut dyn RngCore,
    ) -> Option<SampledRadiance>;
    fn bsdf(&self, shading_info: &ShadingInfo, incoming: Unit3) -> Vec3;

    fn pdf(&self, shading_info: &ShadingInfo, incoming: Unit3) -> f64;
    fn is_always_specular(&self) -> bool {
        false
    }
}

pub struct SpecularScatter {
    pub dir: Unit3,
    pub attenuation: Vec3,
}

impl SpecularScatter {
    pub fn new(dir: Unit3, attenuation: Vec3) -> Self {
        Self { dir, attenuation }
    }
}

pub trait SpecularMaterial {
    fn sample_specular_scatter(
        &self,
        shading_info: &ShadingInfo,
        rng: &mut dyn RngCore,
    ) -> Option<SpecularScatter>;
}

impl<M: SpecularMaterial> Material for M {
    fn sample_bsdf(
        &self,
        shading_info: &ShadingInfo,
        rng: &mut dyn RngCore,
    ) -> Option<SampledRadiance> {
        let scatter = self.sample_specular_scatter(shading_info, rng)?;
        Some(SampledRadiance::new_delta(
            scatter.dir,
            scatter.attenuation / shading::cos_theta(scatter.dir),
        ))
    }

    fn bsdf(&self, _shading_info: &ShadingInfo, _incoming: Unit3) -> Vec3 {
        Vec3::default()
    }

    fn pdf(&self, _shading_info: &ShadingInfo, _incoming: Unit3) -> f64 {
        0.
    }

    fn is_always_specular(&self) -> bool {
        true
    }
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
    fn sample_bsdf(
        &self,
        _shading_info: &ShadingInfo,
        rng: &mut dyn RngCore,
    ) -> Option<SampledRadiance> {
        let dir = CosWeightedHemisphere.sample(rng);
        Some(SampledRadiance::new_real(
            dir,
            self.albedo * f64::consts::FRAC_1_PI,
            shading::cos_theta(dir) * f64::consts::FRAC_1_PI,
        ))
    }

    fn bsdf(&self, _shading_info: &ShadingInfo, _incoming: Unit3) -> Vec3 {
        self.albedo * f64::consts::FRAC_1_PI
    }

    fn pdf(&self, shading_info: &ShadingInfo, incoming: Unit3) -> f64 {
        if same_hemisphere(*incoming, *shading_info.outgoing) {
            shading::cos_theta(incoming)
        } else {
            0.
        }
    }
}

pub struct Mirror {
    color: Vec3,
}

impl Mirror {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl SpecularMaterial for Mirror {
    fn sample_specular_scatter(
        &self,
        shading_info: &ShadingInfo,
        _rng: &mut dyn RngCore,
    ) -> Option<SpecularScatter> {
        let reflected = reflect_z(*shading_info.outgoing);
        Some(SpecularScatter::new(
            Unit3::new_unchecked(reflected),
            self.color,
        ))
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

impl SpecularMaterial for Dielectric {
    fn sample_specular_scatter(
        &self,
        shading_info: &ShadingInfo,
        rng: &mut dyn RngCore,
    ) -> Option<SpecularScatter> {
        let refractive_ratio = match shading_info.side {
            HitSide::Inside => self.refractive_index,
            HitSide::Outside => 1. / self.refractive_index,
        };

        let outgoing = *shading_info.outgoing;
        let cos_theta = shading_info.cos_theta();
        let sin_theta = shading_info.sin_theta();

        let dir = if refractive_ratio * sin_theta > 1.
            || rng.gen::<f64>() < dielectric_reflectance(cos_theta, refractive_ratio)
        {
            reflect_z(outgoing)
        } else {
            let up = *Vec3::z_axis();

            let refracted_perp = refractive_ratio * (cos_theta * up - outgoing);
            let refracted_par = -(1. - refracted_perp.norm_squared()).sqrt() * up;

            refracted_perp + refracted_par
        };

        Some(SpecularScatter::new(
            Unit3::new_normalize(dir),
            Vec3::from_element(1.),
        ))
    }
}

fn dielectric_reflectance(cos_theta: f64, refractive_ratio: f64) -> f64 {
    let r0 = ((1. - refractive_ratio) / (1. + refractive_ratio)).powi(2);
    schlick_reflectance(r0, cos_theta)
}
