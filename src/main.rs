mod camera;
mod hittable;
mod ray;
mod utils;
mod vec3;

use crate::camera::Camera;
use crate::hittable::Sphere;
use crate::vec3::Vec3;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;

fn main() {
    let world = vec![
        Sphere::create(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::create(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ];

    let camera = Camera::default(ASPECT_RATIO, IMAGE_WIDTH);
    camera.render(&world, 100);
}
