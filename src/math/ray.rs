use crate::math::interval::Interval;
use crate::utils::common::*;
use crate::core::hittable::{HitRecord, Hittable};
use crate::core::hittable_list::HittableList;
use crate::math::vec3::{Point3, Vec3};
use crate::math::color::Color;

pub struct Ray {
    pub origin: Point3,
    pub direction: Point3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn hit_sphere(&self, center: Point3, radius: f64) -> f64 {
        let oc = center - self.origin;
        let a = self.direction.length_squared();
        let h = Vec3::dot_two(self.direction, oc);
        let c = oc.length_squared() - radius * radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return -1.0;
        } else {
            (h - discriminant.sqrt()) / a
        }
    }

    pub fn color(self, world: &HittableList) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(self, Interval::new(0.0, INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }
        // Sky gradient
        let unit_direction = Vec3::unit_vector(self.direction);
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray { origin: Point3::default(), direction: Vec3::default() }
    }
}

impl Clone for Ray {
    fn clone(&self) -> Self {
        Ray { origin: self.origin.clone(), direction: self.direction.clone() }
    }
}

impl Copy for Ray {}
