use glam::{DVec3};
use crate::utils::common::*;

pub type Vec3 = DVec3;
pub type Point3 = DVec3;

pub trait Vec3Ext {
    fn reflect(self, n: Self) -> Self;
    fn refract(self, n: Self, etai_over_etat: f64) -> Self;
    fn random() -> Self where Self: Sized;
    fn random_range(min: f64, max: f64) -> Self where Self: Sized;
    fn random_unit_vector() -> Self where Self: Sized;
    fn random_in_unit_disk() -> Self where Self: Sized;
    fn near_zero(self) -> bool;
    fn unit_vector(self) -> Self;
}

impl Vec3Ext for Vec3 {
    fn reflect(self, n: Self) -> Self {
        self - 2.0 * self.dot(n) * n
    }

    fn refract(self, n: Self, etai_over_etat: f64) -> Self {
        let cos_theta = -self.dot(n);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt() * n);
        r_out_perp + r_out_parallel
    }

    fn random() -> Self {
        Self::new(random(), random(), random())
    }

    fn random_unit_vector() -> Self {
        Self::random().normalize()
    }

    fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(random_range(-1.0, 1.0), random_range(-1.0, 1.0), 0.0);
            if p.length_squared() >= 1.0 { continue; }
            return p;
        }
    }

    fn random_range(min: f64, max: f64) -> Self {
        Self::new(random_range(min, max), random_range(min, max), random_range(min, max))
    }

    fn near_zero(self) -> bool {
        self.length_squared() < 1e-8
    }

    fn unit_vector(self) -> Self {
        self.clone().normalize()
    }   
}
