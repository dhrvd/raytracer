pub mod random;
pub mod ray;
pub mod vec3;

pub use {random::*, ray::*, vec3::*};

pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = std::f32::consts::PI;

pub const fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}
