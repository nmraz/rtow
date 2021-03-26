use crate::geom::Geom;
use crate::math::{Ray, Unit3, Vec3};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitSide {
    Inside,
    Outside,
}

pub struct HitInfo {
    pub point: Vec3,
    pub normal: Unit3,
    pub side: HitSide,
}

impl HitInfo {
    pub fn from_geom_ray(geom: &dyn Geom, ray: &Ray, t: f64) -> Self {
        let point = ray.at(t);
        let outward_normal = geom.outward_normal_at(point);

        let (normal, side) = if ray.dir.dot(&outward_normal) > 0. {
            (-outward_normal, HitSide::Inside)
        } else {
            (outward_normal, HitSide::Outside)
        };

        Self {
            point,
            normal,
            side,
        }
    }
}

pub struct Scene {
    primitives: Vec<Box<dyn Geom>>,
}

impl Scene {
    pub fn with_primitives(primitives: Vec<Box<dyn Geom>>) -> Self {
        Self { primitives }
    }

    pub fn hit(&self, ray: &Ray) -> Option<HitInfo> {
        let (geom, t) = self
            .primitives
            .iter()
            .map(|geom| (geom.as_ref(), geom.hit(ray)))
            .fold(None, |nearest, (geom, t)| match (nearest, t) {
                (None, Some(t)) => Some((geom, t)),
                (Some((_, nearest_t)), Some(t)) if t < nearest_t => Some((geom, t)),
                _ => nearest,
            })?;

        Some(HitInfo::from_geom_ray(geom, ray, t))
    }
}
