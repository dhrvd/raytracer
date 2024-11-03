use rand::{thread_rng, Rng};

use crate::math::{vec3, Vec3};

pub fn random_rng(min: f32, max: f32) -> f32 {
    let mut rng = thread_rng();
    rng.gen_range(min..max)
}

pub fn random() -> f32 {
    random_rng(0.0, 1.0)
}

pub fn random_vec3(min: f32, max: f32) -> Vec3 {
    vec3(
        random_rng(min, max),
        random_rng(min, max),
        random_rng(min, max),
    )
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = random_vec3(-1.0, 1.0);

        if p.length_squared() <= 1.0 {
            return p / p.length();
        }
    }
}
