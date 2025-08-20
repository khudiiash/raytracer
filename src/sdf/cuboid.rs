use crate::core::hittable::{Hittable, HitRecord};
use crate::math::mat4::Mat4;
use crate::math::{vec3::{Vec3, Point3}, ray::Ray};
use crate::math::interval::Interval;
use crate::core::material::Material;
use std::sync::Arc;

pub struct Cuboid {
    pub half_size: Vec3,
    pub mat: Arc<dyn Material + Send + Sync>,
    pub transform: Mat4,
}

impl Cuboid {
    pub fn new(half_size: Vec3, mat: Arc<dyn Material + Send + Sync>) -> Self {
        Cuboid { half_size, mat, transform: Mat4::make_identity() }
    }
}

impl Hittable for Cuboid {
    fn hit(&self, r: &Ray, interval: &Interval, rec: &mut HitRecord) -> bool {
        // 1. Transform ray into local space
        let inv_transform = self.transform.inverse();
        let local_origin = inv_transform.transform_point(&r.origin);
        // For direction, transform as a vector (no translation):
        let local_direction = inv_transform.transform_point(&(r.origin + r.direction)) - local_origin;
        let local_ray = Ray::new(local_origin, local_direction);

        // 2. Ray-box intersection in local space (AABB at origin)
        let min = -self.half_size;
        let max = self.half_size;
        let mut tmin = interval.min;
        let mut tmax = interval.max;
        let mut hit_normal = Vec3::default();

        for a in 0..3 {
            let inv_d = 1.0 / local_ray.direction[a];
            let mut t0 = (min[a] - local_ray.origin[a]) * inv_d;
            let mut t1 = (max[a] - local_ray.origin[a]) * inv_d;
            let mut normal = Vec3::default();
            normal[a] = 1.0;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
                normal[a] = -1.0;
            }
            if t0 > tmin {
                tmin = t0;
                hit_normal = normal;
            }
            if t1 < tmax {
                tmax = t1;
            }
            if tmax <= tmin {
                return false;
            }
        }

        let t = tmin;
        if !interval.surrounds(t) {
            return false;
        }

        // 3. Transform intersection point and normal back to world space
        let local_hit_point = local_ray.at(t);
        let world_hit_point = self.transform.transform_point(&local_hit_point);
        // For normal, use the normal matrix (inverse transpose of linear part)
        let normal_matrix = self.transform.inverse().transpose();
        let world_normal = Vec3::unit_vector(&normal_matrix.transform_point(&hit_normal));

        // Compute t in world space for correct depth sorting
        let t_world = (world_hit_point - r.origin).length() / r.direction.length();

        rec.t = t_world;
        rec.point = world_hit_point;
        rec.set_face_normal(r, &world_normal);
        rec.material = self.mat.clone();
        true
    }
}




