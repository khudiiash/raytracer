use std::{fs::File, io::{BufWriter, Write}};
use crate::vec3::Vec3;

pub type Color = Vec3;


pub trait WritableColor {
    fn write_color(&self, writer: &mut BufWriter<File>) -> Result<(), Box<dyn std::error::Error>>;
}

impl WritableColor for Color {
    fn write_color(&self, writer: &mut BufWriter<File>) -> Result<(), Box<dyn std::error::Error>> {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        // Translate [0,1] to [0,255]
        r = ((256.0 * r.clamp(0.0, 0.999)) as u8).into();
        g = ((256.0 * g.clamp(0.0, 0.999)) as u8).into();
        b = ((256.0 * b.clamp(0.0, 0.999)) as u8).into();

        writeln!(writer, "{} {} {}", r, g, b)?;
        Ok(())
    }
}