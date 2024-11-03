use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::math::{Ray, Vec3};

pub struct Sphere {
    center: Ray,
    radius: f32,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Material>) -> Self {
        Self {
            center: Ray::new(center, Vec3::ZEROS, 0.0),
            radius,
            material,
        }
    }

    pub fn moving(center1: Vec3, center2: Vec3, radius: f32, material: Arc<dyn Material>) -> Self {
        Self {
            center: Ray::new(center1, center2 - center1, 0.0),
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time);
        let oc = current_center - ray.origin;

        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // find the nearest root that lies in the acceptable range
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - current_center) / self.radius;

        Some(HitRecord::new(
            point,
            outward_normal,
            ray,
            root,
            Arc::clone(&self.material),
        ))
    }

    fn aabb(&self) -> Aabb {
        let box1 = Aabb::new(
            self.center.at(0.0) - self.radius,
            self.center.at(0.0) + self.radius,
        );
        let box2 = Aabb::new(
            self.center.at(1.0) - self.radius,
            self.center.at(1.0) + self.radius,
        );

        box1.join(&box2)
    }
}
