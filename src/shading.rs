use crate::geom::HitSide;
use crate::math::{Unit3, Vec3};

pub fn cos_theta(dir: Unit3) -> f64 {
    dir[2]
}

pub fn sin_theta(dir: Unit3) -> f64 {
    (1. - cos_theta(dir).powi(2)).sqrt()
}

pub fn same_hemisphere(incoming: Vec3, outgoing: Vec3) -> bool {
    incoming[2] * outgoing[2] > 0.
}

#[derive(Debug, Clone, Copy)]
pub struct ShadingInfo {
    pub side: HitSide,
    pub outgoing: Unit3,
}

impl ShadingInfo {
    pub fn cos_theta(&self) -> f64 {
        cos_theta(self.outgoing)
    }

    pub fn sin_theta(&self) -> f64 {
        sin_theta(self.outgoing)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Pdf {
    Real(f64),
    Delta,
}

impl Pdf {
    pub fn factor(&self) -> f64 {
        match self {
            Pdf::Real(val) => 1. / val,
            Pdf::Delta => 1.,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SampledRadiance {
    pub dir: Unit3,
    pub color: Vec3,
    pub pdf: Pdf,
}

impl SampledRadiance {
    pub fn new_real(dir: Unit3, color: Vec3, pdf: f64) -> Self {
        Self {
            dir,
            color,
            pdf: Pdf::Real(pdf),
        }
    }

    pub fn new_delta(dir: Unit3, color: Vec3) -> Self {
        Self {
            dir,
            color,
            pdf: Pdf::Delta,
        }
    }

    pub fn scaled_color(&self) -> Vec3 {
        cos_theta(self.dir) * self.pdf.factor() * self.color
    }
}
