use crate::math::{vec3, Ray, Vec3};

#[derive(Clone, Copy)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub const EMPTY: Self = Self {
        min: Vec3::from_v(f32::INFINITY),
        max: Vec3::from_v(f32::NEG_INFINITY),
    };

    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn join(&self, other: &Self) -> Self {
        Aabb::new(
            vec3(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            vec3(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        )
    }

    pub fn longest_axis(&self) -> usize {
        [
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        ]
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(index, _)| index)
        .unwrap_or(0)
    }

    pub fn hit(&self, ray: &Ray, mut ray_tmin: f32, mut ray_tmax: f32) -> bool {
        for axis in 0..3 {
            let inv_d = 1.0 / ray.direction[axis];

            let mut t0 = (self.min[axis] - ray.origin[axis]) * inv_d;
            let mut t1 = (self.max[axis] - ray.origin[axis]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            ray_tmin = t0.max(ray_tmin);
            ray_tmax = t1.min(ray_tmax);

            if ray_tmax <= ray_tmin {
                return false;
            }
        }

        true
    }
}
