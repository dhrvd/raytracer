mod hittable;
mod ray;
mod vec3;

use crate::hittable::{HitRecord, Object, Sphere};
use crate::ray::Ray;
use crate::vec3::Vec3;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;

const IMAGE_HEIGHT: i32 = {
    let height = IMAGE_WIDTH as f32 / ASPECT_RATIO;
    if height < 1. {
        1
    } else {
        height as i32
    }
};

pub fn world_hit(
    ray: &Ray,
    ray_tmin: f32,
    ray_tmax: f32,
    world: &Vec<Object>,
) -> Option<HitRecord> {
    let mut temp_rec = None;
    let mut closest_so_far = ray_tmax;

    for object in world {
        if let Some(rec) = object.hit(ray, ray_tmin, closest_so_far) {
            closest_so_far = rec.t;
            temp_rec = Some(rec);
        }
    }

    temp_rec
}

fn ray_color(ray: &Ray, world: &Vec<Object>) -> Vec3 {
    if let Some(rec) = world_hit(ray, 0.0, f32::INFINITY, world) {
        return (rec.normal + 1.0) * 0.5;
    }

    let unit_direction = ray.direction.normalize();
    let a = (unit_direction.y + 1.0) * 0.5;

    Vec3::ONES * (1. - a) + Vec3::new(0.5, 0.7, 1.0) * a
}

fn main() {
    // world
    let world: Vec<Object> = vec![
        Sphere::create(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::create(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ];

    // camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32);

    let focal_length = 1.0;
    let camera_center = Vec3::ZEROS;

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f32;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f32;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f32) + (pixel_delta_v * j as f32);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let color = ray_color(&ray, &world);

            // translate the [0, 1] component values to the byte range [0, 255]
            let rbyte = (255.999 * color.x) as i32;
            let gbyte = (255.999 * color.y) as i32;
            let bbyte = (255.999 * color.z) as i32;

            println!("{} {} {}", rbyte, gbyte, bbyte);
        }
    }

    eprintln!("Done!")
}
