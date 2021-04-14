use std::sync::Arc;

use crate::geom::{Geom, HitInfo};
use crate::material::Material;
use crate::math::Ray;
use crate::shading::ShadingInfo;

use self::bvh::BvhNode;
use self::prim::Primitive;

mod bvh;
mod prim;

pub struct PrimitiveHit<'a> {
    pub geom_hit: HitInfo,
    pub material: &'a dyn Material,
}

impl<'a> PrimitiveHit<'a> {
    pub fn new(geom_hit: HitInfo, material: &'a dyn Material) -> Self {
        Self { geom_hit, material }
    }

    pub fn shading(&self, ray: &Ray) -> ShadingInfo {
        let incoming = -self.geom_hit.world_to_local(ray.dir);

        ShadingInfo {
            side: self.geom_hit.side,
            incoming,
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
    pub fn hit(&self, ray: &Ray, t_max: f64) -> Option<PrimitiveHit<'_>> {
        let (prim, raw) = self.primitives.as_ref()?.hit(ray, t_max)?;
        let geom_hit = HitInfo::from_raw(ray, &raw);
        Some(PrimitiveHit::new(geom_hit, &*prim.material))
    }
}
