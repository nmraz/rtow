use std::sync::Arc;

use crate::geom::Geom;
use crate::material::Material;
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

pub struct Primitive {
    pub geom: Box<dyn Geom + Sync>,
    pub material: Arc<dyn Material + Send + Sync>,
}

impl Primitive {
    pub fn new(
        geom: impl Geom + Sync + 'static,
        material: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        Self {
            geom: Box::new(geom),
            material,
        }
    }
}

pub struct Scene {
    primitives: Vec<Primitive>,
}

impl Scene {
    pub fn with_primitives(primitives: Vec<Primitive>) -> Self {
        Self { primitives }
    }

    pub fn hit(&self, ray: &Ray) -> Option<(HitInfo, &dyn Material)> {
        let (prim, t) = self
            .primitives
            .iter()
            .map(|prim| (prim, prim.geom.hit(ray)))
            .fold(None, |nearest, (prim, t)| match (nearest, t) {
                (None, Some(t)) => Some((prim, t)),
                (Some((_, nearest_t)), Some(t)) if t < nearest_t => Some((prim, t)),
                _ => nearest,
            })?;

        Some((HitInfo::from_geom_ray(&*prim.geom, ray, t), &*prim.material))
    }
}
