use crate::hittable::{Hittable, HittableList};
use crate::utils::random_unit_vector;
use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn color(&self, depth: u32, world: &HittableList) -> Vec3 {
        if depth == 0 {
            return Vec3::ZEROS;
        }

        if let Some(rec) = world.hit(self, 0.001, f32::INFINITY) {
            let direction = rec.normal + random_unit_vector();
            return Ray::new(rec.point, direction).color(depth - 1, world) * 0.5;
        }

        let unit_direction = self.direction.normalize();
        let a = (unit_direction.y + 1.0) * 0.5;

        Vec3::ONES * (1. - a) + Vec3::new(0.5, 0.7, 1.0) * a
    }
}
