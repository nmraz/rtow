use std::sync::Arc;

use crate::geom::Geom;
use crate::material::Material;

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
