use crate::core::material::Material;
use crate::math::color::Color;
use crate::math::ray::Ray;
use crate::core::hittable::HitRecord;
use crate::math::vec3::Vec3;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Default for Metal {
    fn default() -> Self {
        Metal { albedo: Color::new(0.5, 0.5, 0.5), fuzz: 0.1 }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(&r_in.direction, &rec.normal);
        *scattered = Ray { origin: rec.point, direction: reflected + self.fuzz * Vec3::random_unit_vector() };
        *attenuation = self.albedo.clone();
        true
    }
}
