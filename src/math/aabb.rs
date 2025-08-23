use crate::math::vec3::{Vec3, Point3};
use crate::math::interval::Interval;
use crate::math::ray::Ray;

#[derive(Clone, Debug)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    /// The default AABB is empty, since intervals are empty by default.
    pub fn empty() -> Self {
        Aabb {
            x: Interval::new_empty(),
            y: Interval::new_empty(),
            z: Interval::new_empty(),
        }
    }

    pub fn universe() -> Self {
        Aabb {
            x: Interval::new_universe(),
            y: Interval::new_universe(),
            z: Interval::new_universe(),
        }
    }

    pub fn from_intervals(x: Interval, y: Interval, z: Interval) -> Self {
        let mut aabb = Aabb { x, y, z };
        aabb.pad_to_minimums();
        aabb
    }

    /// Construct from two points (order doesn't matter)
    pub fn from_points(a: &Point3, b: &Point3) -> Self {
        let x = if a.x <= b.x {
            Interval::new(a.x, b.x)
        } else {
            Interval::new(b.x, a.x)
        };
        let y = if a.y <= b.y {
            Interval::new(a.y, b.y)
        } else {
            Interval::new(b.y, a.y)
        };
        let z = if a.z <= b.z {
            Interval::new(a.z, b.z)
        } else {
            Interval::new(b.z, a.z)
        };
        let mut aabb = Aabb { x, y, z };
        aabb.pad_to_minimums();
        aabb
    }

    /// Construct from two AABBs (enclosing both)
    pub fn from_aabb(box0: &Aabb, box1: &Aabb) -> Self {
        let x = Interval::from_two(&box0.x, &box1.x);
        let y = Interval::from_two(&box0.y, &box1.y);
        let z = Interval::from_two(&box0.z, &box1.z);
        Aabb { x, y, z }
    }

    pub fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    /// Ray-AABB intersection test
    pub fn hit(&self, r: Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin;
        let ray_dir = r.direction;

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];
            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;

            let (t_min, t_max) = if t0 < t1 { (t0, t1) } else { (t1, t0) };

            if t_min > ray_t.min {
                ray_t.min = t_min;
            }
            if t_max < ray_t.max {
                ray_t.max = t_max;
            }
            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    /// Returns the index of the longest axis of the bounding box.
    pub fn longest_axis(&self) -> usize {
        let x_size = self.x.size();
        let y_size = self.y.size();
        let z_size = self.z.size();
        if x_size > y_size {
            if x_size > z_size { 0 } else { 2 }
        } else {
            if y_size > z_size { 1 } else { 2 }
        }
    }

    fn pad_to_minimums(&mut self) {
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }
}

// AABB + Vec3
impl std::ops::Add<Vec3> for Aabb {
    type Output = Aabb;
    fn add(self, offset: Vec3) -> Aabb {
        Aabb {
            x: self.x + offset.x,
            y: self.y + offset.y,
            z: self.z + offset.z,
        }
    }
}

// Vec3 + AABB
impl std::ops::Add<Aabb> for Vec3 {
    type Output = Aabb;
    fn add(self, bbox: Aabb) -> Aabb {
        bbox + self
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Aabb::empty()
    }
}
