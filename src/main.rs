use std::{fs::File, io::{self, BufWriter, Write}};
use indicatif::{ProgressBar, ProgressStyle};

mod utils;
mod math;
mod core;
mod sdf;

use crate::core::hittable_list::*;
use crate::math::vec3::{Point3, Vec3};
use crate::math::ray::Ray;
use crate::math::color::WritableColor;
use crate::sdf::sphere::Sphere;

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 1028;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const MAX_COLOR: u8 = 255;

// Camera
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
const FOCAL_LENGTH: f64 = 1.0;
static CAMERA_CENTER: Point3 = Point3 { e: [0.0, 0.0, 0.0] };

// Output
const OUTPUT_FILE: &str = "output.ppm";

fn main() -> io::Result<()> {
    let focal_length: f64 = 1.0;
    let camera_center = math::vec3::Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and vertical viewport edges
    let viewport_u = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = camera_center - viewport_u/2.0 - viewport_v/2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // World
    let mut world = HittableList::new();
    let sphere = Sphere::new(
        Point3 { e: [0.0, 0.0, -1.0] },
        0.5
    );
    let sphere2 = Sphere::new(
        Point3 { e: [0.0, -100.5, 1.0] },
        100.0
    );
    world.add(Box::new(sphere));
    world.add(Box::new(sphere2));

    // Output file
    let file = File::create(OUTPUT_FILE)?;
    let mut writer = BufWriter::new(file);
    let pb = ProgressBar::new((IMAGE_WIDTH * IMAGE_HEIGHT).into());
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}%").unwrap());

    // Header
    writeln!(writer, "P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, MAX_COLOR)?;

    // Render
    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            pb.inc(1);
            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u + j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - CAMERA_CENTER;
            let ray = Ray {
                origin: CAMERA_CENTER,
                direction: ray_direction,
            };

            let color = ray.color(&world);
            color.write_color(&mut writer).unwrap();
        }
    }
    pb.finish();
    Ok(())
}
