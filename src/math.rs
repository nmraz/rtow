use nalgebra::{Unit, Vector3};

pub const EPSILON: f64 = 1e-9;

pub type Vec3 = Vector3<f64>;
pub type Unit3 = Unit<Vec3>;

pub struct OrthoNormalBasis {
    u: Unit3,
    v: Unit3,
    w: Unit3,
}

impl OrthoNormalBasis {
    pub fn from_wv(w: Unit3, v: Vec3) -> Self {
        let u = Unit3::new_normalize(v.cross(&w));
        let v = Unit3::new_unchecked(w.cross(&u));

        Self { u, v, w }
    }

    pub fn from_w(w: Unit3) -> Self {
        let other = if w.dot(&Vec3::x_axis()) > 0.9999 {
            Vec3::y_axis()
        } else {
            Vec3::x_axis()
        };

        let u = Unit3::new_normalize(w.cross(&other));
        let v = Unit3::new_unchecked(w.cross(&u));

        Self { u, v, w }
    }

    pub fn u(&self) -> Unit3 {
        self.u
    }

    pub fn v(&self) -> Unit3 {
        self.v
    }

    pub fn w(&self) -> Unit3 {
        self.w
    }

    pub fn trans_from_canonical(&self, point: Vec3) -> Vec3 {
        Vec3::new(point.dot(&self.u), point.dot(&self.v), point.dot(&self.w))
    }

    pub fn trans_to_canonical(&self, point: Vec3) -> Vec3 {
        point[0] * self.u.as_ref() + point[1] * self.v.as_ref() + point[2] * self.w.as_ref()
    }
}

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
