use crate::geom::HitSide;
use crate::math::{Unit3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct ShadingInfo {
    pub side: HitSide,
    pub incoming: Unit3,
}

impl ShadingInfo {
    pub fn cos_theta(&self) -> f64 {
        self.incoming[2]
    }

    pub fn sin_theta(&self) -> f64 {
        (1. - self.cos_theta().powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Pdf {
    Real(f64),
    Delta,
}

#[derive(Debug, Clone, Copy)]
pub struct SampledRadiance {
    pub dir: Unit3,
    pub attenuation: Vec3,
    pub pdf: Pdf,
}

impl SampledRadiance {
    pub fn new_real(dir: Unit3, attenuation: Vec3, pdf: f64) -> Self {
        Self {
            dir,
            attenuation,
            pdf: Pdf::Real(pdf),
        }
    }

    pub fn new_specular(dir: Unit3, attenuation: Vec3) -> Self {
        Self {
            dir,
            attenuation,
            pdf: Pdf::Delta,
        }
    }
}
