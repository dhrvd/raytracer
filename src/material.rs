use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils::random_unit_vector;
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
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }

    fn reflect(vector: Vec3, normal: Vec3) -> Vec3 {
        vector - normal * vector.dot(normal) * 2.0
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = Metal::reflect(r_in.direction, rec.normal);

        Some((self.albedo, Ray::new(rec.point, reflected)))
    }
}
