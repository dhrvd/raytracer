use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils::{random_float, random_unit_vector};
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        let s = 1e-8;
        if scatter_direction.x < s && scatter_direction.y < s && scatter_direction.z < s {
            scatter_direction = rec.normal;
        }

        Some((self.albedo, Ray::new(rec.point, scatter_direction)))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected =
            reflect(r_in.direction, rec.normal).normalize() + (random_unit_vector() * self.fuzz);

        if reflected.dot(rec.normal) > 0.0 {
            Some((self.albedo, Ray::new(rec.point, reflected)))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction.normalize();
        let cos_theta = -unit_direction.dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = {
            if ri * sin_theta > 1.0 || reflectance(cos_theta, ri) > random_float(0.0, 1.0) {
                reflect(unit_direction, rec.normal)
            } else {
                refract(unit_direction, rec.normal, ri)
            }
        };

        Some((Vec3::ONES, Ray::new(rec.point, direction)))
    }
}

fn reflect(vector: Vec3, normal: Vec3) -> Vec3 {
    vector - normal * vector.dot(normal) * 2.0
}

fn refract(uv: Vec3, normal: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = -uv.dot(normal).min(1.0);

    let r_out_perp = (uv + normal * cos_theta) * etai_over_etat;
    let r_out_parallel = normal * -(1.0 - r_out_perp.length_squared()).abs().sqrt();

    r_out_perp + r_out_parallel
}

// uses shlick's approximation for reflectance
fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
