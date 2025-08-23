//! Adapted from Peter Shirley's "Ray Tracing in One Weekend" series (CC0)
//! See <http://creativecommons.org/publicdomain/zero/1.0/>

use std::sync::Arc;

use crate::core::material::Material;
use crate::materials::lambertian::Lambertian;
use crate::math::aabb::Aabb;
use crate::math::interval::Interval;
use crate::math::ray::Ray;
use crate::math::vec3::{Point3, Vec3};

/// Stores information about a ray-object intersection.
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Option<&'static dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {

        HitRecord {
            point: Point3::default(),
            normal: Vec3::default(),
            material: None,
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }
}

impl Clone for HitRecord {
    fn clone(&self) -> Self {
        HitRecord {
            point: self.point,
            normal: self.normal,
            material: self.material,
            t: self.t,
            u: self.u,
            v: self.v,
            front_face: self.front_face,
        }
    }
}

impl HitRecord {
    /// Sets the normal and front_face fields based on the ray and outward normal.
    /// The outward normal is assumed to be unit length.
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot_two(r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

/// Trait for objects that can be intersected by rays.
pub trait Hittable: Send + Sync {
    /// Returns true if the ray hits the object within the interval, and fills rec with hit info.
    fn hit(&self, r: Ray, interval: Interval, rec: &mut HitRecord) -> bool;

    /// Returns the axis-aligned bounding box of the object.
    fn bounding_box(&self) -> &Aabb;
}

pub struct Translate {
    pub hittable: Arc<dyn Hittable + Send + Sync>,
    pub offset: Vec3,
    pub bbox: Aabb,
}

impl Translate {
    pub fn new(hittable: Arc<dyn Hittable + Send + Sync>, offset: Vec3) -> Self {
        let bbox = hittable.bounding_box().clone() + offset;
        Translate { hittable, offset, bbox }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: Ray, interval: Interval, rec: &mut HitRecord) -> bool {
        let offset_r = Ray { origin: r.origin - self.offset, direction: r.direction };
        if !self.hittable.hit(offset_r, interval, rec) {
            return false;
        }
        rec.point += self.offset;
        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

pub struct RotateY {
    pub hittable: Arc<dyn Hittable + Send + Sync>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub bbox: Aabb,
}

impl RotateY {
    pub fn new(hittable: Arc<dyn Hittable + Send + Sync>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = hittable.bounding_box().clone();

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        bbox = Aabb::from_points(&min, &max);

        RotateY { hittable, sin_theta, cos_theta, bbox }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: Ray, interval: Interval, rec: &mut HitRecord) -> bool {
        // Transform the ray from world space to object space.

        let origin = Point3::new(
            (self.cos_theta * r.origin.x) - (self.sin_theta * r.origin.z),
            r.origin.y,
            (self.sin_theta * r.origin.x) + (self.cos_theta * r.origin.z),
        );

        let direction = Vec3::new(
            (self.cos_theta * r.direction.x) - (self.sin_theta * r.direction.z),
            r.direction.y,
            (self.sin_theta * r.direction.x) + (self.cos_theta * r.direction.z),
        );

        let rotated_r = Ray {
            origin,
            direction,
            ..r
        };

        // Determine whether an intersection exists in object space (and if so, where).
        if !self.hittable.hit(rotated_r, interval, rec) {
            return false;
        }

        // Transform the intersection from object space back to world space.
        rec.point = Point3::new(
            (self.cos_theta * rec.point.x) + (self.sin_theta * rec.point.z),
            rec.point.y,
            (-self.sin_theta * rec.point.x) + (self.cos_theta * rec.point.z),
        );

        rec.normal = Vec3::new(
            (self.cos_theta * rec.normal.x) + (self.sin_theta * rec.normal.z),
            rec.normal.y,
            (-self.sin_theta * rec.normal.x) + (self.cos_theta * rec.normal.z),
        );

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
