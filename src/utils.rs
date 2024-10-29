use rand::Rng;

use crate::vec3::Vec3;

pub fn random_float() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

pub fn sample_square() -> Vec3 {
    Vec3::new(random_float() - 0.5, random_float() - 0.5, 0.0)
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
