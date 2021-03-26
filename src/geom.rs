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
        if t1 > 0. {
            Some(t1)
        } else if t2 > 0. {
            Some(t2)
        } else {
            None
        }
    }

    fn outward_normal_at(&self, point: Vec3) -> Unit3 {
        Unit3::new_normalize(point - self.center)
    }
}
