use crate::math::{Aabb, OrthoNormalBasis, Ray, Unit3, Vec3, EPSILON};

#[derive(Debug, Clone, Copy)]
pub struct RawHitInfo {
    pub t: f64,
    pub outward_normal: Unit3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitSide {
    Inside,
    Outside,
}

pub struct HitInfo {
    pub point: Vec3,
    pub basis: OrthoNormalBasis,
    pub side: HitSide,
}

impl HitInfo {
    pub fn from_raw(ray: &Ray, raw: &RawHitInfo) -> Self {
        let &RawHitInfo { t, outward_normal } = raw;

        let point = ray.at(t);

        let (normal, side) = if ray.dir.dot(&outward_normal) > 0. {
            (-outward_normal, HitSide::Inside)
        } else {
            (outward_normal, HitSide::Outside)
        };

        let basis = OrthoNormalBasis::from_w(normal);

        Self { point, basis, side }
    }

    pub fn world_to_local(&self, world: Unit3) -> Unit3 {
        Unit3::new_unchecked(self.basis.trans_from_canonical(*world))
    }

    pub fn local_to_world(&self, local: Unit3) -> Unit3 {
        Unit3::new_unchecked(self.basis.trans_to_canonical(*local))
    }

    pub fn spawn_ray(&self, local_dir: Unit3) -> Ray {
        Ray {
            origin: self.point,
            dir: self.local_to_world(local_dir),
        }
    }
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
