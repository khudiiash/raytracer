use crate::core::material::Material;
use crate::math::color::Color;
use crate::math::ray::Ray;
use crate::core::hittable::HitRecord;
use crate::math::vec3::Vec3;

pub struct Lambertian {
    pub albedo: Color,
}

impl Default for Lambertian {
    fn default() -> Self {
        Lambertian { albedo: Color::white() }
    }
}

impl Clone for Lambertian {
    fn clone(&self) -> Self {
        Lambertian { albedo: self.albedo.clone() }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        // Catch degerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray { origin: rec.point, direction: scatter_direction };
        *attenuation = self.albedo.clone();
        true
    }
}