use rand::{Rng, RngCore};
use rand_distr::{Distribution, UnitSphere};

use crate::math::{Unit3, Vec3, EPSILON};
use crate::scene::{HitInfo, HitSide};

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
    fn scatter(
        &self,
        incoming: Unit3,
        hit: &HitInfo,
        rng: &mut dyn RngCore,
    ) -> Option<ScatteredRay> {
        let reflected = reflect(*incoming, hit.normal);
        let dir = Unit3::new_normalize(
            reflected + (1. - self.gloss) * Vec3::from(UnitSphere.sample(rng)),
        );

        if dir.dot(&hit.normal) > 0. {
            Some(ScatteredRay {
                dir,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}

fn reflect(incoming: Vec3, n: Unit3) -> Vec3 {
    incoming - 2. * incoming.dot(&n) * n.as_ref()
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
    fn scatter(
        &self,
        incoming: Unit3,
        hit: &HitInfo,
        rng: &mut dyn RngCore,
    ) -> Option<ScatteredRay> {
        let refractive_ratio = match hit.side {
            HitSide::Inside => self.refractive_index,
            HitSide::Outside => 1. / self.refractive_index,
        };

        let cos_theta = (-incoming).dot(&hit.normal);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let dir = if refractive_ratio * sin_theta > 1.
            || rng.gen::<f64>() < schlick_reflectance(cos_theta, refractive_ratio)
        {
            reflect(*incoming, hit.normal)
        } else {
            let refracted_perp = refractive_ratio * (*incoming + cos_theta * hit.normal.as_ref());
            let refracted_par = -(1. - refracted_perp.norm_squared()).sqrt() * hit.normal.as_ref();

            refracted_perp + refracted_par
        };

        Some(ScatteredRay {
            dir: Unit3::new_normalize(dir),
            attenuation: Vec3::from_element(1.),
        })
    }
}

fn schlick_reflectance(cos_theta: f64, refractive_ratio: f64) -> f64 {
    let r0 = ((1. - refractive_ratio) / (1. + refractive_ratio)).powi(2);
    r0 + (1. - r0) * (1. - cos_theta).powi(5)
}
