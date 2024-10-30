mod camera;
mod hittable;
mod material;
mod objects;
mod ray;
mod utils;
mod vec3;

use std::f32::consts::PI;
use std::sync::Arc;

use material::Material;
use utils::{random_float, random_vec3};

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::objects::Sphere;
use crate::vec3::Vec3;

const LOOKFROM: Vec3 = Vec3::new(13.0, 2.0, 3.0);
const LOOKAT: Vec3 = Vec3::ZEROS;
const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
const VFOV: f32 = 20.0 * PI / 180.0;

const DEFOCUS_ANGLE: f32 = 0.6 * PI / 180.0;
const FOCUS_DIST: f32 = 10.0;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 1200;

fn main() {
    let mut world = HittableList::new(Vec::new());

    let material_ground = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_float(0.0, 1.0);
            let center = Vec3::new(
                a as f32 + 0.9 * random_float(0.0, 1.0),
                0.2,
                b as f32 + 0.9 * random_float(0.0, 1.0),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Arc<dyn Material> = {
                    if choose_mat < 0.8 {
                        let albedo = random_vec3(0.0, 1.0) * random_vec3(0.0, 1.0);
                        Arc::new(Lambertian::new(albedo))
                    } else if choose_mat < 0.95 {
                        let albedo = random_vec3(0.5, 1.0);
                        let fuzz = random_float(0.0, 0.5);

                        Arc::new(Metal::new(albedo, fuzz))
                    } else {
                        Arc::new(Dielectric::new(1.5))
                    }
                };

                world.add(Box::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3)));

    let camera = Camera::new(
        LOOKFROM,
        LOOKAT,
        VUP,
        VFOV,
        DEFOCUS_ANGLE,
        FOCUS_DIST,
        ASPECT_RATIO,
        IMAGE_WIDTH,
    );
    camera.render(&world, 500, 50);
}
