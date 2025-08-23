use crate::core::hittable::HitRecord;
use crate::core::material::Material;
use crate::math::color::Color;
use crate::math::ray::Ray;
use crate::math::vec3::Point3;

pub struct DiffuseLight {
    pub emit: Color,
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: Point3) -> Color {
        self.emit
    }

    fn scatter(&self, r_in: Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        false
    }
}
