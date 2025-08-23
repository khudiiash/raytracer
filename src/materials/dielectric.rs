use crate::core::material::Material;
use crate::math::color::Color;
use crate::math::ray::Ray;
use crate::core::hittable::HitRecord;
use crate::math::vec3::Vec3;
use crate::utils::common::random;

#[derive(Clone)]
pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 * r0 + (1.0 - r0 * r0) * (1.0 - cosine).powi(5)
    }
}

impl Default for Dielectric {
    fn default() -> Self {
        Dielectric { ref_idx: 1.5 }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face { 1.0 / self.ref_idx } else { self.ref_idx };
        let unit_direction = Vec3::unit_vector(r_in.direction);
        let cos_theta = Vec3::dot_two(-unit_direction, rec.normal);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, ri) > random() {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::refract(unit_direction, rec.normal, ri)
        };

        *scattered = Ray { origin: rec.point, direction };
        true
    }

}
