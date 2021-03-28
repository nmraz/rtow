use crate::math::{Ray, Unit3, Vec3, EPSILON};

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min_point: Vec3,
    pub max_point: Vec3,
}

impl Aabb {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            min_point: a.inf(&b),
            max_point: a.sup(&b),
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            min_point: self.min_point.inf(&other.min_point),
            max_point: self.max_point.sup(&other.max_point),
        }
    }

    pub fn intersects(&self, ray: &Ray) -> bool {
        let mut t_min: f64 = 0.;
        let mut t_max: f64 = f64::INFINITY;

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

pub trait Geom {
    fn aabb(&self) -> Aabb;
    fn hit(&self, ray: &Ray) -> Option<f64>;
    fn outward_normal_at(&self, point: Vec3) -> Unit3;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Geom for Sphere {
    fn aabb(&self) -> Aabb {
        let radius_vec = Vec3::from_element(self.radius);
        Aabb::new(self.center - radius_vec, self.center + radius_vec)
    }

    fn hit(&self, ray: &Ray) -> Option<f64> {
        let oc = ray.origin - self.center;
        let b = oc.dot(&ray.dir);
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = b * b - c;

        if discriminant < 0. {
            return None;
        }

        let radical = discriminant.sqrt();

        let t1 = -b - radical;
        let t2 = -b + radical;

        // Find the intersection nearest to the origin (minimal `t`), but never report
        // intersections behind the origin (negative `t`).
        if t1 > EPSILON {
            Some(t1)
        } else if t2 > EPSILON {
            Some(t2)
        } else {
            None
        }
    }

    fn outward_normal_at(&self, point: Vec3) -> Unit3 {
        Unit3::new_normalize(point - self.center)
    }
}
