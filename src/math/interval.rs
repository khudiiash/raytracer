#[derive(Clone, Debug)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    /// Default interval is empty: min = +infinity, max = -infinity
    pub fn new_empty() -> Self {
        Interval { min: f64::INFINITY, max: f64::NEG_INFINITY }
    }

    /// Universe interval: min = -infinity, max = +infinity
    pub fn new_universe() -> Self {
        Interval { min: f64::NEG_INFINITY, max: f64::INFINITY }
    }

    /// Construct from min and max
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    /// Construct tightly enclosing the two input intervals
    pub fn from_two(a: &Interval, b: &Interval) -> Self {
        Interval {
            min: if a.min <= b.min { a.min } else { b.min },
            max: if a.max >= b.max { a.max } else { b.max },
        }
    }

    /// Size of the interval
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// Returns true if x is in [min, max]
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// Returns true if x is in (min, max)
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    /// Clamp x to [min, max]
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    /// Expand the interval by delta (total), padding both sides by delta/2
    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Interval {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}

// Static constants for empty and universe intervals
impl Interval {
    pub const EMPTY: Interval = Interval { min: f64::INFINITY, max: f64::NEG_INFINITY };
    pub const UNIVERSE: Interval = Interval { min: f64::NEG_INFINITY, max: f64::INFINITY };
}

impl Default for Interval {
    fn default() -> Self {
        Interval::new_empty()
    }
}

// interval + displacement
impl std::ops::Add<f64> for Interval {
    type Output = Interval;
    fn add(self, displacement: f64) -> Interval {
        Interval {
            min: self.min + displacement,
            max: self.max + displacement,
        }
    }
}

// displacement + interval
impl std::ops::Add<Interval> for f64 {
    type Output = Interval;
    fn add(self, ival: Interval) -> Interval {
        ival + self
    }
}
