use crate::math::aabb::Aabb;
use crate::core::hittable::{Hittable, HitRecord};
use crate::math::bvh_node::BvhNode;
use crate::math::ray::Ray;
use crate::math::interval::Interval;
use std::sync::Arc;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
    pub bbox: Aabb,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: Vec::new(), bbox: Aabb::empty() }
    }

    pub fn from_bvh(bvh: BvhNode) -> Self {
        HittableList { objects: vec![Arc::new(bvh)], bbox: Aabb::empty() }
    }

    pub fn add<T>(&mut self, object: T) where T: Hittable + Send + Sync + 'static,
    {
        let arc_obj = Arc::new(object);
        self.bbox = Aabb::from_aabb(&self.bbox, &arc_obj.bounding_box());
        self.objects.push(arc_obj);
    }

    pub fn hit(&self, r: &Ray, interval: &Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = interval.max;

        for object in self.objects.iter() {
            if object.hit(r, &Interval::new(interval.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, interval: &Interval, rec: &mut HitRecord) -> bool {
        self.hit(r, interval, rec)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}