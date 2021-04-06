use std::sync::Arc;

use crate::geom::{Geom, RawHitInfo};
use crate::material::Material;
use crate::math::{Ray, Unit3, Vec3};

use self::bvh::BvhNode;

mod bvh;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitSide {
    Inside,
    Outside,
}

pub struct HitInfo<'a> {
    pub point: Vec3,
    pub normal: Unit3,
    pub side: HitSide,
    pub material: &'a dyn Material,
}

impl<'a> HitInfo<'a> {
    pub fn from_raw(ray: &Ray, raw: &RawHitInfo, material: &'a dyn Material) -> Self {
        let &RawHitInfo { t, outward_normal } = raw;

        let point = ray.at(t);

        let (normal, side) = if ray.dir.dot(&outward_normal) > 0. {
            (-outward_normal, HitSide::Inside)
        } else {
            (outward_normal, HitSide::Outside)
        };

        Self {
            point,
            normal,
            side,
            material,
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
    primitives: Option<Box<BvhNode>>,
}

impl Scene {
    pub fn with_primitives(primitives: Vec<Primitive>) -> Self {
        Self {
            primitives: bvh::build(primitives),
        }
    }

    pub fn hit(&self, ray: &Ray) -> Option<HitInfo> {
        let (prim, raw) = self.primitives.as_ref()?.hit(ray, f64::INFINITY)?;
        Some(HitInfo::from_raw(ray, &raw, &*prim.material))
    }
}
