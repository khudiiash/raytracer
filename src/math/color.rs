use std::{fs::File, io::{BufWriter, Write}};
use crate::{math::vec3::Vec3, utils::common::linear_to_gamma};
use crate::math::interval::Interval;

pub type Color = Vec3;

pub trait WritableColor {
    fn write_color(&self, writer: &mut BufWriter<File>) -> Result<(), Box<dyn std::error::Error>>;
}

impl WritableColor for Color {
    fn write_color(&self, writer: &mut BufWriter<File>) -> Result<(), Box<dyn std::error::Error>> {
        let r = self.x;
        let g = self.y;
        let b = self.z;

        // Apply gamma correction
        let r = linear_to_gamma(r);
        let g = linear_to_gamma(g);
        let b = linear_to_gamma(b);

        // Translate [0,1] to [0,255]
        let intensity = Interval::new(0.0, 0.999);
        let rbyte = (256.0 * intensity.clamp(r)) as u8;
        let gbyte = (256.0 * intensity.clamp(g)) as u8;
        let bbyte = (256.0 * intensity.clamp(b)) as u8;

        writeln!(writer, "{} {} {}", rbyte, gbyte, bbyte)?;
        Ok(())
    }
}