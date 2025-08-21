use crate::math::color::Color;
use crate::core::hittable::HitRecord;
use crate::math::ray::Ray;
use crate::math::vec3::Point3;

pub trait Material {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}
