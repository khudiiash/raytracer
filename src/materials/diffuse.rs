use crate::core::material::Material;
use crate::math::color::Color;
use crate::math::ray::Ray;
use crate::core::hittable::HitRecord;
use crate::math::vec3::{Vec3, Vec3Ext};

pub struct Diffuse {
    pub albedo: Color,
}

impl Material for Diffuse {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        *scattered = Ray { origin: rec.point, direction: scatter_direction };
        *attenuation = self.albedo.clone();
        true
    }
}