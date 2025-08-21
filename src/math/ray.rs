use crate::math::vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Point3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray { origin: Point3::default(), direction: Vec3::default() }
    }
}

impl Clone for Ray {
    fn clone(&self) -> Self {
        Ray { origin: self.origin.clone(), direction: self.direction.clone() }
    }
}

impl Copy for Ray {}