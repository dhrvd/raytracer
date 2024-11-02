mod camera;
mod hittable;
mod material;
mod math;
mod objects;
mod render;

use std::sync::Arc;

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::math::{degrees_to_radians, random, random_rng, random_vec3, vec3, Vec3};
use crate::objects::Sphere;

const IMAGE_WIDTH: u32 = 400;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 50;

fn main() {
    let mut world = HittableList::new(Vec::new());

    let material_ground = Arc::new(Lambertian::new(vec3(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = vec3(a as f32 + 0.9 * random(), 0.2, b as f32 + 0.9 * random());

            if (center - vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = random_vec3(0.0, 1.0) * random_vec3(0.0, 1.0);
                    let material = Arc::new(Lambertian::new(albedo));

                    let center2 = center + vec3(0.0, random_rng(0.0, 0.5), 0.0);
                    world.add(Box::new(Sphere::moving(center, center2, 0.2, material)));
                } else if choose_mat < 0.95 {
                    let albedo = random_vec3(0.5, 1.0);
                    let fuzz = random_rng(0.0, 0.5);

                    let material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                } else {
                    let material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Arc::new(Lambertian::new(vec3(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(vec3(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(Metal::new(vec3(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(vec3(4.0, 1.0, 0.0), 1.0, material3)));

    let camera = Camera::new(
        vec3(13.0, 2.0, 3.0),
        Vec3::ZEROS,
        vec3(0.0, 1.0, 0.0),
        degrees_to_radians(20.0),
        degrees_to_radians(0.6),
        10.0,
        (IMAGE_WIDTH, IMAGE_HEIGHT),
    );

    render::render(
        camera,
        world,
        (IMAGE_WIDTH, IMAGE_HEIGHT),
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        "image.png",
    );
}
