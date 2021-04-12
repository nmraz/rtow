use crate::geom::RawHitInfo;
use crate::math::{Aabb, Ray, Vec3, EPSILON};

use super::Primitive;

enum BvhNodeData {
    Leaf {
        prim: Primitive,
    },
    Interior {
        left: Box<BvhNode>,
        right: Box<BvhNode>,
    },
}

pub struct BvhNode {
    bounds: Aabb,
    data: BvhNodeData,
}

impl BvhNode {
    pub fn hit(&self, ray: &Ray, t_max: f64) -> Option<(&Primitive, RawHitInfo)> {
        if !self.bounds.hit(ray, EPSILON, t_max) {
            return None;
        }

        match &self.data {
            BvhNodeData::Leaf { prim } => prim.geom.hit(ray, t_max).map(|info| (prim, info)),
            BvhNodeData::Interior { left, right } => {
                let left_hit = left.hit(ray, t_max);
                let right_hit = right.hit(ray, left_hit.map_or(t_max, |(_prim, info)| info.t));

                match (left_hit, right_hit) {
                    (None, Some(hit)) => Some(hit),
                    (Some((_priml, il)), Some((primr, ir))) if ir.t < il.t => Some((primr, ir)),
                    _ => left_hit,
                }
            }
        }
    }
}

pub fn build(primitives: impl IntoIterator<Item = Primitive>) -> Option<Box<BvhNode>> {
    do_build(
        primitives
            .into_iter()
            .map(|prim| {
                let bounds = prim.geom.bounds();

                TaggedPrimitive {
                    prim,
                    bounds,
                    centroid: bounds.centroid(),
                }
            })
            .collect(),
    )
}

struct TaggedPrimitive {
    prim: Primitive,
    bounds: Aabb,
    centroid: Vec3,
}

fn do_build(mut tagged_primitives: Vec<TaggedPrimitive>) -> Option<Box<BvhNode>> {
    if tagged_primitives.is_empty() {
        return None;
    }

    if tagged_primitives.len() == 1 {
        let first = tagged_primitives.pop().unwrap();

        return Some(Box::new(BvhNode {
            bounds: first.bounds,
            data: BvhNodeData::Leaf { prim: first.prim },
        }));
    }

    let bounds = tagged_primitives[1..]
        .iter()
        .fold(tagged_primitives[0].bounds, |aabb, next| {
            aabb.union(&next.bounds)
        });

    // Partition the boxes by centroid values, using the axis along which the extent spanned by the
    // centroids is the longest.

    let centroid_bounds = tagged_primitives[1..].iter().fold(
        Aabb::at_point(tagged_primitives[0].centroid),
        |aabb, next| aabb.extend(next.centroid),
    );

    let longest_axis = (centroid_bounds.max_point - centroid_bounds.min_point).imax();
    let mid = tagged_primitives.len() / 2;

    tagged_primitives.select_nth_unstable_by(mid, |tp1, tp2| {
        tp1.centroid[longest_axis]
            .partial_cmp(&tp2.centroid[longest_axis])
            .unwrap()
    });

    let (left, right) = {
        let right = tagged_primitives.split_off(mid);
        (tagged_primitives, right)
    };

    Some(Box::new(BvhNode {
        bounds,
        data: BvhNodeData::Interior {
            left: do_build(left).unwrap(),
            right: do_build(right).unwrap(),
        },
    }))
}
