use std::sync::Arc;

use crate::core::hittable::{Hittable, HitRecord};

use crate::core::material::Material;
use crate::math::interval::Interval;
use crate::math::ray::Ray;
use crate::math::vec3::{Point3, Vec3, Vec3Ext};

pub struct Quad {
    pub mat: Arc<dyn Material + Send + Sync>,

    Q: Point3,
    D: f32,
    normal: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Quad {
    pub fn new(Q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material + Send + Sync>) -> Self {
        let n = Vec3::cross(u, v);
        let normal = n.normalize(); 
        let D = Vec3::dot(normal, Q);
        let w = n / Vec3::dot(n, n);
        Quad { Q, D, w, normal, u, v, mat }
    }

    fn is_interior(&self, alpha: f32, beta: f32, rec: &mut HitRecord) -> bool {
        let interval = Interval::new(0.0, 1.0);
        // Given the hit point in plane coordinates, return false if it is outside the 
        // primitive, otherwise set the hit record UV coordinates and return true
        if !interval.contains(alpha) || !interval.contains(beta) {
            return false;
        }

        rec.u = alpha;
        rec.v = beta;
        true
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, interval: &Interval, rec: &mut HitRecord) -> bool {
        let denom = Vec3::dot(self.normal, r.direction);
        if denom.abs() < 1e-8 {
            return false;
        }

        let t = (self.D - Vec3::dot(self.normal, r.origin)) / denom;
        if !interval.contains(t) {
            return false;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates
        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.Q;
        let alpha = Vec3::dot(self.w, Vec3::cross(planar_hitpt_vector, self.v));
        let beta = Vec3::dot(self.w, Vec3::cross(self.u, planar_hitpt_vector));

        if !self.is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.point = intersection;
        rec.material = self.mat.clone();
        rec.set_face_normal(r, &self.normal);
        true
   }
}