use std::sync::Arc;

use crate::geom::{Geom, RawHitInfo};
use crate::material::Material;
use crate::math::{Ray, Unit3, Vec3};

use self::bvh::BvhNode;
use self::prim::Primitive;

mod bvh;
mod prim;

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

pub struct SceneBuilder {
    primitives: Vec<Primitive>,
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self {
            primitives: Vec::new(),
        }
    }

    pub fn add_primitive(
        &mut self,
        geom: impl Geom + Sync + 'static,
        material: Arc<dyn Material + Send + Sync>,
    ) {
        self.primitives.push(Primitive::new(geom, material))
    }

    pub fn build(self) -> Scene {
        Scene {
            primitives: bvh::build(self.primitives),
        }
    }
}

pub struct Scene {
    primitives: Option<Box<BvhNode>>,
}

impl Scene {
    pub fn hit(&self, ray: &Ray, t_max: f64) -> Option<HitInfo<'_>> {
        let (prim, raw) = self.primitives.as_ref()?.hit(ray, t_max)?;
        Some(HitInfo::from_raw(ray, &raw, &*prim.material))
    }
}
