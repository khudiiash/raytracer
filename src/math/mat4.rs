use std::ops::Mul;
use std::fmt;

use crate::math::{quat::Quat, vec3::Vec3, vec4::Vec4};

pub struct Mat4 {
    pub m: [f64; 16],
}

impl Mat4 {
    pub fn new(m: [f64; 16]) -> Self {
        Mat4 { m }
    }

    pub fn make_identity() -> Self {
        Mat4::new([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn make_translation(x: f64, y: f64, z: f64) -> Self {
        Mat4::new([
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn make_scale(x: f64, y: f64, z: f64) -> Self {
        Mat4::new([
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn inverse(&self) -> Self {
        let mut result = Mat4::make_identity();
        let det = self.m[0] * (self.m[5] * self.m[10] - self.m[6] * self.m[9]) -
                  self.m[1] * (self.m[4] * self.m[10] - self.m[6] * self.m[8]) +
                  self.m[2] * (self.m[4] * self.m[9] - self.m[5] * self.m[8]);
        if det.abs() < 1e-8 {
            return Mat4::make_identity();
        }
        let inv_det = 1.0 / det;
        result.m[0] = (self.m[5] * self.m[10] - self.m[6] * self.m[9]) * inv_det;
        result.m[1] = (self.m[2] * self.m[9] - self.m[1] * self.m[10]) * inv_det;
        result.m[2] = (self.m[1] * self.m[6] - self.m[2] * self.m[5]) * inv_det;
        result.m[4] = (self.m[6] * self.m[8] - self.m[4] * self.m[10]) * inv_det;
        result.m[5] = (self.m[0] * self.m[10] - self.m[2] * self.m[8]) * inv_det;
        result.m[6] = (self.m[2] * self.m[4] - self.m[0] * self.m[6]) * inv_det;
        result.m[8] = (self.m[4] * self.m[9] - self.m[5] * self.m[8]) * inv_det;
        result.m[9] = (self.m[1] * self.m[8] - self.m[0] * self.m[9]) * inv_det;
        result.m[10] = (self.m[0] * self.m[5] - self.m[1] * self.m[4]) * inv_det;
        result.m[12] = (self.m[12] * self.m[13] - self.m[13] * self.m[12]) * inv_det;
        result.m[13] = (self.m[10] * self.m[12] - self.m[8] * self.m[14]) * inv_det;
        result.m[14] = (self.m[8] * self.m[13] - self.m[10] * self.m[12]) * inv_det;
        result
    }

    pub fn transpose(&self) -> Self {
        Mat4::new([
            self.m[0], self.m[4], self.m[8], self.m[12],
            self.m[1], self.m[5], self.m[9], self.m[13],
            self.m[2], self.m[6], self.m[10], self.m[14],
            self.m[3], self.m[7], self.m[11], self.m[15],
        ])
    }

    /// Transforms a point (Vec3) by this matrix, treating the input as a position (w=1).
    /// Returns the transformed Vec3, performing perspective divide if needed.
    pub fn transform_point(&self, point: &Vec3) -> Vec3 {
        // Matrix is in column-major order: m[0..3] is first column, m[4..7] is second, etc.
        let x = point.x();
        let y = point.y();
        let z = point.z();
        let m = &self.m;

        let tx = m[0] * x + m[4] * y + m[8]  * z + m[12];
        let ty = m[1] * x + m[5] * y + m[9]  * z + m[13];
        let tz = m[2] * x + m[6] * y + m[10] * z + m[14];
        let tw = m[3] * x + m[7] * y + m[11] * z + m[15];

        if tw.abs() > 1e-8 {
            Vec3::new(tx / tw, ty / tw, tz / tw)
        } else {
            Vec3::new(tx, ty, tz)
        }
    }

    pub fn set_translation(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
        self.m[12] = x;
        self.m[13] = y;
        self.m[14] = z;
        self
    }

    pub fn translate(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
        self.m[12] += x;
        self.m[13] += y;
        self.m[14] += z;
        self
    }

    pub fn set_scale(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
        self.m[0] = x;
        self.m[5] = y;
        self.m[10] = z;
        self
    }

    pub fn scale(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
        self.m[0] *= x;
        self.m[5] *= y;
        self.m[10] *= z;
        self
    }

    pub fn set_rotation(&mut self, rotation: &Quat) -> &mut Self {
        self.m[0] = rotation.x;
        self.m[1] = rotation.y;
        self.m[2] = rotation.z;
        self.m[3] = rotation.w;
        self
    }

    pub fn rotate_x(&mut self, angle: f64) -> &mut Self {
        let s = angle.sin();
        let c = angle.cos();
        let m1 = self.m[5];
        let m2 = self.m[6];
        let m4 = self.m[9];
        let m5 = self.m[10];

        self.m[5] = c;
        self.m[6] = m2 * s + m1 * c;
        self.m[9] = m4 * s + m5 * c;
        self.m[10] = m5 * s - m4 * c;
        self
    }

    pub fn rotate_y(&mut self, angle: f64) -> &mut Self {
        let s = angle.sin();
        let c = angle.cos();
        let m0 = self.m[0];
        let m2 = self.m[2];
        let m8 = self.m[8];
        let m10 = self.m[10];

        self.m[0] = m0 * c + m2 * s;
        self.m[2] = m2 * c - m0 * s;
        self.m[8] = m8 * c + m10 * s;
        self.m[10] = m10 * c - m8 * s;
        self
    }

    pub fn rotate_z(&mut self, angle: f64) -> &mut Self {
        let s = angle.sin();
        let c = angle.cos();
        let m0 = self.m[0];
        let m1 = self.m[1];
        let m4 = self.m[4];
        let m5 = self.m[5];

        self.m[0] = m0 * c - m1 * s;
        self.m[1] = m1 * c + m0 * s;
        self.m[4] = m4 * c - m5 * s;
        self.m[5] = m5 * c + m4 * s;
        self
    }

    pub fn look_at(&mut self, eye: &Vec3, center: &Vec3, up: &Vec3) -> &mut Self {
        let mut result = Mat4::make_identity();
        let f = Vec3::unit_vector(&(*center - *eye));
        let s = Vec3::unit_vector(&Vec3::cross(&f, &up));
        let u = Vec3::cross(&s, &f);

        result.m[0] = s.x();
        result.m[1] = u.x();
        result.m[2] = -f.x();
        result.m[4] = s.y();
        result.m[5] = u.y();
        result.m[6] = -f.y();
        result.m[8] = s.z();
        result.m[9] = u.z();
        result.m[10] = -f.z();
        result.m[12] = -Vec3::dot(&s, &eye);
        result.m[13] = -Vec3::dot(&u, &eye);
        result.m[14] = Vec3::dot(&f, &eye);
        *self = result;
        self
    }

}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Mat4 {
        let mut result = Mat4::new([0.0; 16]);
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.m[i * 4 + j] += self.m[i * 4 + k] * other.m[k * 4 + j];
                }
            }
        }
        result
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Vec4 {
        Vec4::new(
            self.m[0] * other.x() + self.m[1] * other.y() + self.m[2] * other.z(),
            self.m[4] * other.x() + self.m[5] * other.y() + self.m[6] * other.z(),
            self.m[8] * other.x() + self.m[9] * other.y() + self.m[10] * other.z(),
            self.m[12] * other.x() + self.m[13] * other.y() + self.m[14] * other.z() + self.m[15] * other.w(),
        )
    }
}

impl Mul<Vec3> for Mat4 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.m[0] * other.x() + self.m[1] * other.y() + self.m[2] * other.z(),
            self.m[4] * other.x() + self.m[5] * other.y() + self.m[6] * other.z(),
            self.m[8] * other.x() + self.m[9] * other.y() + self.m[10] * other.z(),
        )
    }
}   

impl Clone for Mat4 {
    fn clone(&self) -> Self {
        Mat4 { m: self.m.clone() }
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        Mat4::make_identity()
    }
}

impl fmt::Display for Mat4 {    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mat4(\n{}", self.m[0])?;
        for i in 1..16 {
            if i % 4 == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", self.m[i])?;
        }
        write!(f, "\n)")
    }
}