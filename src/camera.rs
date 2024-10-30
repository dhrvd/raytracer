use std::time::Instant;

use crate::hittable::HittableList;
use crate::ray::Ray;
use crate::utils::{linear_to_gamma, sample_square};
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

    pub fn render(&self, world: &HittableList, samples_per_pixel: u32, max_depth: u32) {
        eprintln!(
            "Rendering {}x{} at {} samples per pixel with max depth {}",
            self.image_width, self.image_height, samples_per_pixel, max_depth
        );
        let start = Instant::now();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut color = Vec3::ZEROS;
                for _ in 0..samples_per_pixel {
                    color += self.get_ray(i, j).color(max_depth, world)
                }
                color /= samples_per_pixel as f32;

                // translate the [0, 1] component values to the byte range [0, 255]
                let rbyte = (255.999 * linear_to_gamma(color.x)) as i32;
                let gbyte = (255.999 * linear_to_gamma(color.y)) as i32;
                let bbyte = (255.999 * linear_to_gamma(color.z)) as i32;

                println!("{} {} {}", rbyte, gbyte, bbyte);
            }
        }

        eprintln!("Done! {:?}", start.elapsed());
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (offset.x + i as f32))
            + (self.pixel_delta_v * (offset.y + j as f32));

        Ray::new(self.center, pixel_sample - self.center)
    }
}
