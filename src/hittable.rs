use std::sync::Arc;

use crate::aabb::Aabb;
use crate::material::Material;
use crate::math::{Ray, Vec3};

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord>;
    fn aabb(&self) -> Aabb;
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        point: Vec3,
        outward_normal: Vec3,
        ray: &Ray,
        t: f32,
        material: Arc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }
}
