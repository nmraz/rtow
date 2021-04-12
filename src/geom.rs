use crate::math::{Aabb, Ray, Unit3, Vec3, EPSILON};

#[derive(Debug, Clone, Copy)]
pub struct RawHitInfo {
    pub t: f64,
    pub outward_normal: Unit3,
}

pub trait Geom {
    fn bounds(&self) -> Aabb;
    fn hit(&self, ray: &Ray, t_max: f64) -> Option<RawHitInfo>;
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
    fn bounds(&self) -> Aabb {
        let radius_vec = Vec3::from_element(self.radius);
        Aabb::new(self.center - radius_vec, self.center + radius_vec)
    }

    fn hit(&self, ray: &Ray, t_max: f64) -> Option<RawHitInfo> {
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

        let t = [t1, t2]
            .iter()
            .copied()
            .find(|t| (EPSILON..t_max).contains(t))?;

        let normal = Unit3::new_unchecked((ray.at(t) - self.center) / self.radius);

        Some(RawHitInfo {
            t,
            outward_normal: normal,
        })
    }
}
