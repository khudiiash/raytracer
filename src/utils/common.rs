use rand::Rng;

pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = 3.1415926535897932385;
pub const DEG_TO_RAD: f32 = PI / 180.0;
pub const RAD_TO_DEG: f32 = 180.0 / PI;
pub const EPSILON: f32 = 1e-8;


pub fn random() -> f32 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn random_range(min: f32, max: f32) -> f32 {
    rand::thread_rng().gen_range(min..max)
}

pub fn random_int(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min..max) 
}

pub fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.001 {
        linear_component.sqrt()
    } else {
        0.0
    }
}