use std::sync::Arc;

use crate::core::material::Material;
use crate::materials::lambertian::Lambertian;
use crate::math::color::Color;
use crate::math::interval::Interval;
use crate::math::mat4::Mat4;
use crate::math::ray::Ray;
use crate::math::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub transform: Mat4,
    pub material: Arc<dyn Material + Send + Sync>,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            point: Point3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
            transform: Mat4::make_identity(),
            material: Arc::new(Lambertian { albedo: Color::default() }),
        }
    }
}

impl Clone for HitRecord {
    fn clone(&self) -> Self {
        HitRecord {
            point: self.point.clone(),
            normal: self.normal.clone(),
            t: self.t,
            front_face: self.front_face,
            transform: self.transform.clone(),
            material: self.material.clone(),
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, interval: &Interval, rec: &mut HitRecord) -> bool;
}