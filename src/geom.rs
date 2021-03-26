use crate::math::{Ray, Unit3, Vec3};

pub trait Geom {
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
    fn hit(&self, ray: &Ray) -> Option<f64> {
        let oc = ray.origin - self.center;
        let half_b = oc.dot(&ray.dir);
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - c;

        if discriminant >= 0. {
            Some(-half_b - discriminant.sqrt())
        } else {
            None
        }
    }

    fn outward_normal_at(&self, point: Vec3) -> Unit3 {
        Unit3::new_normalize(point - self.center)
    }
}
