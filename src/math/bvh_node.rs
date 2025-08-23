use crate::math::aabb::Aabb;
use crate::math::interval::Interval;
use crate::math::ray::Ray;
use std::cmp::Ordering;
use std::sync::Arc;
use crate::core::hittable::{Hittable, HitRecord};


pub struct BvhNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub bbox: Aabb,
}

impl BvhNode {
    pub fn new_from_list(mut objects: Vec<Arc<dyn Hittable + Send + Sync>>) -> Self {
        let len = objects.len();
        Self::new(&mut objects, 0, len)
    }

    pub fn new(objects: &mut [Arc<dyn Hittable + Send + Sync>], start: usize, end: usize) -> Self {
        let mut bbox = Aabb::empty();
        for object in &objects[start..end] {
            bbox = Aabb::from_aabb(&bbox, &object.bounding_box());
        }

        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };

        let object_span = end - start;

        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = if object_span == 1 {
            let obj = objects[start].clone();
            (obj.clone(), obj)
        } else if object_span == 2 {
            let left = objects[start].clone();
            let right = objects[start + 1].clone();
            (left, right)
        } else {
            objects[start..end].sort_by(|a, b| comparator(&a, &b));
            let mid = start + object_span / 2;
            let left = Arc::new(BvhNode::new(objects, start, mid));
            let right = Arc::new(BvhNode::new(objects, mid, end));
            (left, right)
        };

        let bbox = {
            let left_box = left.bounding_box();
            let right_box = right.bounding_box();
            Aabb::from_aabb(&left_box, &right_box)
        };

        BvhNode { left, right, bbox }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: Ray, interval: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, interval.clone()) {
            return false;
        }

        let hit_left = self.left.hit(r, interval.clone(), rec);

        let t_max = if hit_left { rec.t } else { interval.max };
        let new_interval = Interval { min: interval.min, max: t_max };
        let hit_right = self.right.hit(r, new_interval, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

// Box compare helpers

fn box_compare(
    a: &Arc<dyn Hittable + Send + Sync>,
    b: &Arc<dyn Hittable + Send + Sync>,
    axis_index: usize,
) -> Ordering {
    let a_axis = a.bounding_box().axis_interval(axis_index);
    let b_axis = b.bounding_box().axis_interval(axis_index);
    a_axis.min.partial_cmp(&b_axis.min).unwrap_or(Ordering::Equal)
}

fn box_x_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>) -> Ordering {
    box_compare(a, b, 2)
}
