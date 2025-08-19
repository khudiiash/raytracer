use crate::common::*;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;

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
        let h = Vec3::dot(&self.direction, &oc);
        let c = oc.length_squared() - radius * radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return -1.0;
        } else {
            (h - discriminant.sqrt()) / a
        }
    }

    pub fn color(&self, world: &HittableList) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(self, 0.0, INFINITY, &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }
        // Sky gradient
        let unit_direction = Vec3::unit_vector(&self.direction);
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}