use rand::Rng;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
};

use crate::math::Ray;

pub enum BVHNode {
    Node {
        left: Box<dyn Hittable>,
        right: Box<dyn Hittable>,
        aabb: Aabb,
    },
    Leaf(Box<dyn Hittable>),
}

impl BVHNode {
    pub fn new(hittables: &mut Vec<Box<dyn Hittable>>) -> Self {
        match hittables.len() {
            0 => panic!("cannot construct BVH from an empty hittable list"),
            1 => BVHNode::Leaf(hittables.pop().unwrap()),
            2 => {
                let left = hittables.remove(0);
                let right = hittables.remove(0);
                let aabb = left.aabb().join(&right.aabb());

                BVHNode::Node { left, right, aabb }
            }
            _ => {
                let axis = rand::thread_rng().gen_range(0..3);
                hittables
                    .sort_by(|a, b| a.aabb().min[axis].partial_cmp(&b.aabb().min[axis]).unwrap());

                let mut right_hittables = hittables.split_off(hittables.len() / 2);

                let left = BVHNode::new(hittables);
                let right = BVHNode::new(&mut right_hittables);
                let aabb = left.aabb().join(&right.aabb());

                BVHNode::Node {
                    left: Box::new(left),
                    right: Box::new(right),
                    aabb,
                }
            }
        }
    }
}

impl Hittable for BVHNode {
    fn aabb(&self) -> Aabb {
        match self {
            BVHNode::Node { aabb, .. } => *aabb,
            BVHNode::Leaf(h) => h.aabb(),
        }
    }

    fn hit(&self, ray: &Ray, ray_tmin: f32, mut ray_tmax: f32) -> Option<HitRecord> {
        if !self.aabb().hit(ray, ray_tmin, ray_tmax) {
            return None;
        }

        match self {
            BVHNode::Leaf(h) => h.hit(ray, ray_tmin, ray_tmax),
            BVHNode::Node { left, right, .. } => {
                let hit_left = left.hit(ray, ray_tmin, ray_tmax);
                ray_tmax = hit_left.as_ref().map_or(ray_tmax, |rec| rec.t);
                let hit_right = right.hit(ray, ray_tmin, ray_tmax);

                if hit_right.is_some() {
                    hit_right
                } else {
                    hit_left
                }
            }
        }
    }
}
