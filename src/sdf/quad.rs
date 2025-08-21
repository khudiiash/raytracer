use std::sync::Arc;

use crate::core::hittable::{Hittable, HitRecord};
use crate::core::hittable_list::HittableList;
use crate::core::material::Material;
use crate::math::aabb::Aabb;
use crate::math::interval::Interval;
use crate::math::ray::Ray;
use crate::math::vec3::{Point3, Vec3};

pub struct Quad {
    pub q: Point3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub normal: Vec3,
    pub d: f64,
    pub bbox: Aabb,
    pub material: Arc<dyn Material + Send + Sync>,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: Arc<dyn Material + Send + Sync>) -> Self {
        let n = Vec3::cross(&u, &v);
        let normal = Vec3::unit_vector(&n);
        let d = Vec3::dot(&normal, &q);
        let w = n / Vec3::dot(&n, &n);

        // Compute the bounding box of all four vertices.
        let p0 = q;
        let p1 = q + u;
        let p2 = q + v;
        let p3 = q + u + v;

        let bbox_diagonal1 = Aabb::from_points(&p0, &p3);
        let bbox_diagonal2 = Aabb::from_points(&p1, &p2);
        let bbox = Aabb::from_aabb(&bbox_diagonal1, &bbox_diagonal2);

        Quad {
            q,
            u,
            v,
            w,
            normal,
            d,
            bbox,
            material,
        }
    }

    /// Returns true if the point (alpha, beta) is inside the quad, and sets rec.u, rec.v
    fn is_interior(&self, alpha: f64, beta: f64, rec: &mut HitRecord) -> bool {
        let unit = Interval::new(0.0, 1.0);
        if !unit.contains(alpha) || !unit.contains(beta) {
            return false;
        }
        rec.u = alpha;
        rec.v = beta;
        true
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, interval: &Interval, rec: &mut HitRecord) -> bool {
        let denom = Vec3::dot(&self.normal, &r.direction);

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return false;
        }

        // Compute intersection t
        let t = (self.d - Vec3::dot(&self.normal, &r.origin)) / denom;
        if !interval.contains(t) {
            return false;
        }

        // Compute intersection point and check if inside quad
        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = Vec3::dot(&self.w, &Vec3::cross(&planar_hitpt_vector, &self.v));
        let beta = Vec3::dot(&self.w, &Vec3::cross(&self.u, &planar_hitpt_vector));

        if !self.is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.point = intersection;
        rec.material = self.material.clone();
        rec.set_face_normal(r, &self.normal);

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

pub fn make_box(a: Point3, b: Point3, material: Arc<dyn Material + Send + Sync>) -> HittableList {
    let min = Vec3::min(&a, &b);
    let max = Vec3::max(&a, &b);

    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);

    let mut sides= HittableList::new();
    sides.add(Quad::new(Point3::new(min.x, min.y, max.z), dx, dy, material.clone())); // front
    sides.add(Quad::new(Point3::new(max.x, min.y, max.z), -dz, dy, material.clone())); // right
    sides.add(Quad::new(Point3::new(max.x, min.y, min.z), -dx, dy, material.clone())); // back
    sides.add(Quad::new(Point3::new(min.x, min.y, min.z), dz, dy, material.clone())); // left
    sides.add(Quad::new(Point3::new(min.x, max.y, max.z), dx, -dz, material.clone())); // top
    sides.add(Quad::new(Point3::new(min.x, min.y, min.z), dx, dz, material.clone())); // bottom
    sides
}