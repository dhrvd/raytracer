use rand::{thread_rng, Rng};

use crate::vec3::Vec3;

pub fn random_float(min: f32, max: f32) -> f32 {
    let mut rng = thread_rng();
    rng.gen_range(min..max)
}

pub fn sample_square() -> Vec3 {
    Vec3::new(
        random_float(0.0, 1.0) - 0.5,
        random_float(0.0, 1.0) - 0.5,
        0.0,
    )
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

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random_float(-1.0, 1.0), random_float(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}
