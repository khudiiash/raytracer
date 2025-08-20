use std::sync::Arc;

use crate::math::vec3::{Point3, Vec3};
use crate::core::hittable::{Hittable, HitRecord};
use crate::core::material::Material;
use crate::math::interval::Interval;
use crate::math::ray::Ray;

pub struct Cylinder {
    pub center: Point3,
    pub radius: f64,
    pub height: f64,
    pub mat: Arc<dyn Material + Send + Sync>,
}

impl Cylinder {
    pub fn new(center: Point3, radius: f64, height: f64, mat: Arc<dyn Material + Send + Sync>) -> Self {
        Cylinder { center, radius, height, mat }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, r: &Ray, interval: &Interval, rec: &mut HitRecord) -> bool {
        // Cylinder aligned along y-axis, centered at self.center
        // Equation: (x - cx)^2 + (z - cz)^2 = r^2, y in [cy - h/2, cy + h/2]

        let dx = r.direction.x();
        let dz = r.direction.z();
        let ox = r.origin.x() - self.center.x();
        let oz = r.origin.z() - self.center.z();

        let a = dx * dx + dz * dz;
        let b = 2.0 * (ox * dx + oz * dz);
        let c = ox * ox + oz * oz - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Try the smaller root first
        let mut root = (-b - sqrtd) / (2.0 * a);
        let mut y = r.origin.y() + root * r.direction.y();
        let y_min = self.center.y() - self.height / 2.0;
        let y_max = self.center.y() + self.height / 2.0;

        // Check if the intersection is within the cylinder's height
        if !interval.surrounds(root) || y < y_min || y > y_max {
            // Try the other root
            root = (-b + sqrtd) / (2.0 * a);
            y = r.origin.y() + root * r.direction.y();
            if !interval.surrounds(root) || y < y_min || y > y_max {
                // Check for caps intersection
                // Top cap
                let cap_y = y_max;
                if r.direction.y().abs() > 1e-8 {
                    let t_cap = (cap_y - r.origin.y()) / r.direction.y();
                    if interval.surrounds(t_cap) {
                        let x_cap = r.origin.x() + t_cap * r.direction.x() - self.center.x();
                        let z_cap = r.origin.z() + t_cap * r.direction.z() - self.center.z();
                        if x_cap * x_cap + z_cap * z_cap <= self.radius * self.radius {
                            rec.t = t_cap;
                            rec.point = r.at(rec.t);
                            let outward_normal = Vec3::new(0.0, 1.0, 0.0);
                            rec.set_face_normal(r, &outward_normal);
                            rec.material = self.mat.clone();
                            return true;
                        }
                    }
                }
                // Bottom cap
                let cap_y = y_min;
                if r.direction.y().abs() > 1e-8 {
                    let t_cap = (cap_y - r.origin.y()) / r.direction.y();
                    if interval.surrounds(t_cap) {
                        let x_cap = r.origin.x() + t_cap * r.direction.x() - self.center.x();
                        let z_cap = r.origin.z() + t_cap * r.direction.z() - self.center.z();
                        if x_cap * x_cap + z_cap * z_cap <= self.radius * self.radius {
                            rec.t = t_cap;
                            rec.point = r.at(rec.t);
                            let outward_normal = Vec3::new(0.0, -1.0, 0.0);
                            rec.set_face_normal(r, &outward_normal);
                            rec.material = self.mat.clone();
                            return true;
                        }
                    }
                }
                return false;
            }
        }

        // Hit the side of the cylinder
        rec.t = root;
        rec.point = r.at(rec.t);
        let outward_normal = Vec3::new(
            (rec.point.x() - self.center.x()) / self.radius,
            0.0,
            (rec.point.z() - self.center.z()) / self.radius,
        );
        rec.set_face_normal(r, &outward_normal);
        rec.material = self.mat.clone();
        true
    }
}