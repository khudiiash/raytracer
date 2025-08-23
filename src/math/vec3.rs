use crate::utils::common::{random, random_range, EPSILON};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn dot_two(u: Vec3, v: Vec3) -> f64 {
        u.x*v.x + u.y*v.y + u.z*v.z
    }

    pub fn dot(self, v: Vec3) -> f64 {
        self.x*v.x + self.y*v.y + self.z*v.z
    }

    pub fn cross_two(u: Vec3, v: Vec3) -> Vec3 {
        Vec3 {
            x: u.y*v.z - u.z*v.y,
            y: u.z*v.x - u.x*v.z,
            z: u.x*v.y - u.y*v.x,
        }
    }

    pub fn cross(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }

    pub fn min(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x.min(v.x), self.y.min(v.y), self.z.min(v.z))
    }

    pub fn max(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x.max(v.x), self.y.max(v.y), self.z.max(v.z))
    }

    pub fn min_two(u: Vec3, v: Vec3) -> Vec3 {
        Vec3::new(u.x.min(v.x), u.y.min(v.y), u.z.min(v.z))
    }
    pub fn max_two(u: Vec3, v: Vec3) -> Vec3 {
        Vec3::new(u.x.max(v.x), u.y.max(v.y), u.z.max(v.z))
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            let length_sq = p.length_squared();
            if EPSILON < length_sq && length_sq < 1.0 {
                return p / length_sq.sqrt();
            }
        }
    }

    pub fn random_on_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if Vec3::dot_two(on_unit_sphere, normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn random() -> Vec3 {
        Vec3::new(random(), random(), random())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(random_range(min, max), random_range(min, max), random_range(min, max))
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * Vec3::dot_two(v, n) * n
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = Vec3::dot_two(-uv, n);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt() * n);
        r_out_perp + r_out_parallel
    }

}

use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, MulAssign, DivAssign, Index, IndexMut};
use std::fmt;

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x+other.x, self.y+other.y, self.z+other.z)
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, other: f64) -> Vec3 {
        Vec3::new(self.x+other, self.y+other, self.z+other)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x-other.x, self.y-other.y, self.z-other.z)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x*other.x, self.y*other.y, self.z*other.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.x*t, self.y*t, self.z*t)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self*v.x, self*v.y, self*v.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        (1.0/t) * self
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0/t;
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

// Alias for geometric clarity
pub type Point3 = Vec3;
