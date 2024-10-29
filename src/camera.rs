use crate::hittable::{HitRecord, Object, World};
use crate::ray::Ray;
use crate::utils::Interval;
use crate::vec3::Vec3;

pub struct Camera {
    center: Vec3,
    image_width: u32,
    image_height: u32,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
}

impl Camera {
    pub fn new(center: Vec3, focal_length: f32, aspect_ratio: f32, image_width: u32) -> Self {
        let image_height = (image_width as f32 / aspect_ratio).max(1.0) as u32;

        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            center,
            image_width,
            image_height,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
        }
    }

    pub fn default(aspect_ratio: f32, image_width: u32) -> Self {
        Self::new(Vec3::ZEROS, 1.0, aspect_ratio, image_width)
    }

    pub fn render(&self, world: &World) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_u * i as f32)
                    + (self.pixel_delta_v * j as f32);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let color = self.ray_color(&ray, &world);

                // translate the [0, 1] component values to the byte range [0, 255]
                let rbyte = (255.999 * color.x) as i32;
                let gbyte = (255.999 * color.y) as i32;
                let bbyte = (255.999 * color.z) as i32;

                println!("{} {} {}", rbyte, gbyte, bbyte);
            }
        }

        eprintln!("Done!")
    }

    fn world_hit(&self, ray: &Ray, ray_t: Interval, world: &World) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = ray_t.max;

        for object in world {
            if let Some(rec) = object.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }

    fn ray_color(&self, ray: &Ray, world: &Vec<Object>) -> Vec3 {
        if let Some(rec) = self.world_hit(ray, Interval::new(0.0, f32::INFINITY), world) {
            return (rec.normal + 1.0) * 0.5;
        }

        let unit_direction = ray.direction.normalize();
        let a = (unit_direction.y + 1.0) * 0.5;

        Vec3::ONES * (1. - a) + Vec3::new(0.5, 0.7, 1.0) * a
    }
}
