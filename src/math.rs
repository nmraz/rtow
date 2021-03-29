use nalgebra::{Unit, Vector3};

pub const EPSILON: f64 = 1e-9;

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

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min_point: Vec3,
    pub max_point: Vec3,
}

impl Aabb {
    pub fn at_point(point: Vec3) -> Self {
        Self {
            min_point: point,
            max_point: point,
        }
    }

    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            min_point: a.inf(&b),
            max_point: a.sup(&b),
        }
    }

    pub fn extend(&self, point: Vec3) -> Self {
        Self {
            min_point: self.min_point.inf(&point),
            max_point: self.max_point.sup(&point),
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            min_point: self.min_point.inf(&other.min_point),
            max_point: self.max_point.sup(&other.max_point),
        }
    }

    pub fn centroid(&self) -> Vec3 {
        (self.min_point + self.max_point) / 2.
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for i in 0..3 {
            let inv_d = 1. / ray.dir[i];

            let (t0, t1) = {
                let t0 = (self.min_point[i] - ray.origin[i]) * inv_d;
                let t1 = (self.max_point[i] - ray.origin[i]) * inv_d;

                if inv_d >= 0. {
                    (t0, t1)
                } else {
                    (t1, t0)
                }
            };

            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}
