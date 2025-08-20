use std::ops::{Mul, Add, Div};

use crate::utils::common::{random, random_range, EPSILON};
use crate::math::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4 {
    pub e: [f64; 4],
    pub w: f64,
}

impl Vec4 {
    pub fn new(e0: f64, e1: f64, e2: f64, w: f64) -> Self {
        Self { e: [e0, e1, e2, w], w }
    }

    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }
    pub fn w(&self) -> f64 { self.e[3] }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2] + self.e[3]*self.e[3]
    }
    
    pub fn dot(u: &Vec4, v: &Vec4) -> f64 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2] + u.e[3] * v.e[3]
    }
    
    pub fn cross(u: &Vec4, v: &Vec4) -> Vec4 {
        Vec4::new(
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
            u.e[3] * v.e[3],
        )
    }
    
    pub fn unit_vector(v: &Vec4) -> Vec4 {
        Vec4::new(
            v.e[0] / v.length(),
            v.e[1] / v.length(),
            v.e[2] / v.length(),
            v.e[3] / v.length(),
        )
    }
    
    
    
    pub fn random() -> Vec4 {
        Vec4::new(random(), random(), random(), random())
    }
    
    
    
    pub fn random_range(min: f64, max: f64) -> Vec4 {
        Vec4::new(random_range(min, max), random_range(min, max), random_range(min, max), random_range(min, max))
    }
    
    
    
}

impl Mul<Vec4> for Vec4 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Vec4 {
        Vec4::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
            self.e[3] * other.e[3],
        )
    }
}

impl Mul<Vec3> for Vec4 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] * other.x(),
            self.e[1] * other.y(),
            self.e[2] * other.z(),
        )
    }
}

impl Mul<f64> for Vec4 {
    type Output = Vec4;

    fn mul(self, t: f64) -> Vec4 {
        Vec4::new(
            self.e[0] * t,
            self.e[1] * t,
            self.e[2] * t,
            self.e[3] * t,
        )
    }
}

impl Div<f64> for Vec4 {
    type Output = Vec4;

    fn div(self, t: f64) -> Vec4 {
        Vec4::new(
            self.e[0] / t,
            self.e[1] / t,
            self.e[2] / t,
            self.e[3] / t,
        )
    }
}

impl Add for Vec4 {
    type Output = Vec4;

    fn add(self, other: Vec4) -> Vec4 {
        Vec4::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
            self.e[3] + other.e[3],
        )
    }
}