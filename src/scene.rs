use std::sync::Arc;

use crate::geom::{Geom, HitInfo};
use crate::light::Light;
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

    pub fn shading_info(&self, ray: &Ray) -> ShadingInfo {
        let outgoing = -self.geom_hit.world_to_local(ray.dir);

        ShadingInfo {
            side: self.geom_hit.side,
            outgoing,
        }
    }
}

pub struct SceneBuilder {
    primitives: Vec<Primitive>,
    lights: Vec<Arc<dyn Light + Send + Sync>>,
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self {
            primitives: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn add_primitive(
        &mut self,
        geom: impl Geom + Sync + 'static,
        material: Arc<dyn Material + Send + Sync>,
    ) {
        self.primitives.push(Primitive::new(geom, material))
    }

    pub fn add_light(&mut self, light: impl Light + Send + Sync + 'static) {
        self.lights.push(Arc::new(light))
    }

    pub fn build(self) -> Scene {
        Scene {
            primitives: bvh::build(self.primitives),
            lights: self.lights,
        }
    }
}

pub struct Scene {
    primitives: Option<Box<BvhNode>>,
    lights: Vec<Arc<dyn Light + Send + Sync>>,
}

impl Scene {
    pub fn hit(&self, ray: &Ray, t_max: f64) -> Option<PrimitiveHit<'_>> {
        let (prim, raw) = self.primitives.as_ref()?.hit(ray, t_max)?;
        let geom_hit = HitInfo::from_raw(ray, &raw);
        Some(PrimitiveHit::new(geom_hit, &*prim.material))
    }

    pub fn lights(&self) -> &[Arc<dyn Light + Send + Sync>] {
        &self.lights
    }
}
