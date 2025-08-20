use crate::core::hittable::Hittable;
use crate::core::hittable_list::HittableList;
use crate::core::material::Material;
use crate::math::vec3::{Point3, Vec3};
use crate::sdf::quad::Quad;
use std::sync::Arc;

/// Returns a HittableList representing a box (cuboid) defined by two opposite corners and a material.
pub fn cuboid(a: Point3, b: Point3, mat: Arc<dyn Material + Send + Sync>) -> HittableList {
    let mut sides = HittableList::new();

    let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);

    // front
    sides.add(Box::new(Quad::new(
        Point3::new(min.x, min.y, max.z),
        dx,
        dy,
        mat.clone(),
    )));
    // right
    sides.add(Box::new(Quad::new(
        Point3::new(max.x, min.y, max.z),
        -dz,
        dy,
        mat.clone(),
    )));
    // back
    sides.add(Box::new(Quad::new(
        Point3::new(max.x, min.y, min.z),
        -dx,
        dy,
        mat.clone(),
    )));
    // left
    sides.add(Box::new(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dz,
        dy,
        mat.clone(),
    )));
    // top
    sides.add(Box::new(Quad::new(
        Point3::new(min.x, max.y, max.z),
        dx,
        -dz,
        mat.clone(),
    )));
    // bottom
    sides.add(Box::new(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dx,
        dz,
        mat.clone(),
    )));

    sides
}




