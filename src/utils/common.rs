use rand::Rng;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;
pub const DEG_TO_RAD: f64 = PI / 180.0;
pub const RAD_TO_DEG: f64 = 180.0 / PI;
pub const EPSILON: f64 = 1e-8;

thread_local! {
    static RNG: std::cell::RefCell<fastrand::Rng> =
        std::cell::RefCell::new(fastrand::Rng::new());
}

#[inline]
pub fn random() -> f64 {
    RNG.with(|rng| rng.borrow_mut().f64())
}

#[inline]
pub fn random_range(min: f64, max: f64) -> f64 {
    RNG.with(|rng| rng.borrow_mut().f64() * (max - min) + min)
}

pub fn random_int(min: i32, max: i32) -> i32 {
    RNG.with(|rng| (rng.borrow_mut().f64() * ((max - min) as f64)).floor() as i32 + min)
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.001 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn fmt_samples(samples: u64) -> String {
    if samples >= 1_000_000_000 {
        format!("{:.1}B", samples as f64 / 1_000_000_000.0)
    } else if samples >= 1_000_000 {
        format!("{:.1}M", samples as f64 / 1_000_000.0)
    } else if samples >= 1_000 {
        format!("{:.1}K", samples as f64 / 1_000.0)
    } else {
        format!("{}", samples)
    }
}
pub fn fmt_time(seconds: f64) -> String {
    let total_seconds = seconds.round() as u64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let secs = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}
