use crate::ray::Ray;
use crate::utils::Interval;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
}

pub enum Object {
    Sphere(Sphere),
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

impl Object {
    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match *self {
            Object::Sphere(ref object) => object.hit(ray, ray_t),
        }
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn create(center: Vec3, radius: f32) -> Object {
        Object::Sphere(Self { center, radius })
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;

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
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);
        let mut normal = (point - self.center) / self.radius;

        if ray.direction.dot(normal) > 0.0 {
            normal = -normal
        };

        Some(HitRecord {
            t: root,
            point,
            normal,
        })
    }
}
