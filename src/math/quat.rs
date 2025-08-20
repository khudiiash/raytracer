use std::ops::Mul;

use crate::math::vec3::Vec3;

pub struct Quat {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Quat {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Quat { x, y, z, w }
    }

    pub fn identity() -> Self {
        Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }

    pub fn from_euler(x: f64, y: f64, z: f64) -> Self {
        let x = x * 0.5;
        let y = y * 0.5;
        let z = z * 0.5;

        let sin_x = x.sin();
        let sin_y = y.sin();
        let sin_z = z.sin();

        let c1 = x.cos() * y.cos() * z.cos();
        let c2 = x.cos() * y.sin() * z.cos();
        let c3 = x.cos() * y.cos() * z.sin();
        let c4 = x.sin() * y.cos() * z.cos();
        let c5 = x.sin() * y.sin() * z.cos();
        let c6 = x.sin() * y.sin() * z.sin();

        Quat { x: c1 * c2 - c3 * c4 - c5 * c6,
            y: c1 * c4 + c2 * c3 + c5 * c6,
            z: c1 * c6 - c2 * c5 + c3 * c4,
            w: c1 * c5 + c2 * c6 + c3 * c4,
        }
    }
}

impl Mul<Quat> for Quat {
    type Output = Quat;

    fn mul(self, other: Quat) -> Quat {
        Quat { x: self.x * other.w + self.w * other.x + self.y * other.z - self.z * other.y,
            y: self.y * other.w + self.w * other.y + self.z * other.x - self.x * other.z,
            z: self.z * other.w + self.w * other.z + self.x * other.y - self.y * other.x,
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
        }
    }
}

impl Mul<Vec3> for Quat {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        let qv = Vec3::new(self.x, self.y, self.z);
        let uv = Vec3::cross(&qv, &other);
        let uuv = Vec3::cross(&qv, &uv);
        uv * (2.0 * self.w) + uuv * 2.0 + other * (self.w * self.w - Vec3::dot(&qv, &qv))
    }
}

impl Mul<f64> for Quat {
    type Output = Quat;

    fn mul(self, other: f64) -> Quat {
        Quat { x: self.x * other, y: self.y * other, z: self.z * other, w: self.w * other }
    }
}   

impl Clone for Quat {
    fn clone(&self) -> Self {
        Quat { x: self.x, y: self.y, z: self.z, w: self.w }
    }
}

impl Default for Quat {
    fn default() -> Self {
        Quat::identity()
    }
}
