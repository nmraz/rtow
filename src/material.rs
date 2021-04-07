use rand::{Rng, RngCore};
use rand_distr::{Distribution, UnitSphere};

use crate::distr::CosWeightedHemisphere;
use crate::math::{Unit3, Vec3};
use crate::scene::HitSide;

#[derive(Debug, Clone, Copy)]
pub struct ScatteredRay {
    pub dir: Unit3,
    pub attenuation: Vec3,
}

#[derive(Debug, Clone, Copy)]
pub struct Radiance {
    pub scattered: Option<ScatteredRay>,
    pub emitted: Vec3,
}

impl Radiance {
    pub fn none() -> Self {
        Self {
            scattered: None,
            emitted: Vec3::default(),
        }
    }

    pub fn scattered(dir: Unit3, attenuation: Vec3) -> Self {
        Self {
            scattered: Some(ScatteredRay { dir, attenuation }),
            emitted: Vec3::default(),
        }
    }

    pub fn emitted(emitted: Vec3) -> Self {
        Self {
            scattered: None,
            emitted,
        }
    }
}

pub trait Material {
    fn radiance(&self, incoming: Unit3, side: HitSide, rng: &mut dyn RngCore) -> Radiance;
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
    fn radiance(&self, _incoming: Unit3, _side: HitSide, rng: &mut dyn RngCore) -> Radiance {
        Radiance::scattered(CosWeightedHemisphere.sample(rng), self.albedo)
    }
}

pub struct Metal {
    pub albedo: Vec3,
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
    fn radiance(&self, incoming: Unit3, _side: HitSide, rng: &mut dyn RngCore) -> Radiance {
        let reflected = reflect_z(*incoming);
        let dir = Unit3::new_normalize(
            reflected + (1. - self.gloss) * Vec3::from(UnitSphere.sample(rng)),
        );

        let cos_theta = dir[2];

        if cos_theta > 0. {
            Radiance::scattered(
                dir,
                self.albedo.map(|r0| schlick_reflectance(r0, cos_theta)),
            )
        } else {
            Radiance::none()
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
    pub refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }
}

impl Material for Dielectric {
    fn radiance(&self, incoming: Unit3, side: HitSide, rng: &mut dyn RngCore) -> Radiance {
        let refractive_ratio = match side {
            HitSide::Inside => self.refractive_index,
            HitSide::Outside => 1. / self.refractive_index,
        };

        let cos_theta = incoming[2];
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let (dir, attenuation) = if refractive_ratio * sin_theta > 1.
            || rng.gen::<f64>() < dielectric_reflectance(cos_theta, refractive_ratio)
        {
            (reflect_z(*incoming), 1.)
        } else {
            let up = *Vec3::z_axis();

            let refracted_perp = refractive_ratio * (cos_theta * up - *incoming);
            let refracted_par = -(1. - refracted_perp.norm_squared()).sqrt() * up;

            (
                refracted_perp + refracted_par,
                refractive_ratio * refractive_ratio,
            )
        };

        Radiance::scattered(Unit3::new_normalize(dir), Vec3::from_element(attenuation))
    }
}

fn dielectric_reflectance(cos_theta: f64, refractive_ratio: f64) -> f64 {
    let r0 = ((1. - refractive_ratio) / (1. + refractive_ratio)).powi(2);
    schlick_reflectance(r0, cos_theta)
}
