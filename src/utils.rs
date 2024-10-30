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

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();

    if on_unit_sphere.dot(*normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub const EMPTY: Self = Self::new(f32::INFINITY, f32::NEG_INFINITY);
    pub const UNIVERSE: Self = Self::new(f32::NEG_INFINITY, f32::INFINITY);

    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        x.clamp(self.min, self.max)
    }
}
