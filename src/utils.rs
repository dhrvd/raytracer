use rand::{thread_rng, Rng};

use crate::vec3::Vec3;

pub fn random_float(min: f32, max: f32) -> f32 {
    let mut rng = thread_rng();
    rng.gen_range(min..max)
}

pub fn random_vec3(min: f32, max: f32) -> Vec3 {
    Vec3::new(
        random_float(min, max),
        random_float(min, max),
        random_float(min, max),
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
