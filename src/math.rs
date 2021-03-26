use nalgebra::{Unit, Vector3};

pub type Vec3 = Vector3<f64>;
pub type Unit3 = Unit<Vec3>;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Unit3,
}

impl Ray {
    pub fn pointing_through(origin: Vec3, target: Vec3) -> Self {
        Self {
            origin,
            dir: Unit3::new_normalize(target - origin),
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.dir.into_inner()
    }
}
