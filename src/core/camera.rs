use std::{fs::File, io::{self, BufWriter, Write}};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

use crate::math::{mat4::Mat4, vec3::{Point3, Vec3}};
use crate::math::ray::Ray;
use crate::math::color::{Color, WritableColor};
use crate::math::interval::Interval;
use crate::core::hittable_list::HittableList;
use crate::core::hittable::HitRecord;
use crate::utils::common::*;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 1028;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const MAX_COLOR: u8 = 255;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
const FOCAL_LENGTH: f64 = 1.0;
const CAMERA_CENTER: Point3 = Point3 { e: [0.0, 0.0, 0.0] };

pub struct Camera {
    pub transform: Mat4,
    pub aspect_ratio: f64, // Ratio of image width over height
    pub image_width: u32, // Rendered image width in pixel count
    pub samples_per_pixel: u32, // Count of random samples for each pixel
    pub max_depth: u32, // Maximum depth of ray recursion
    pub defocus_angle: f64, // The angle of the defocus disk
    pub focus_distance: f64, // The distance to the focal plane
    pub vfov: f64,
    pub eye: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,
    image_height: u32, // Rendered image height
    center: Point3, // Camera center
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
    pixel00_loc: Point3, // Location of pixel 0,0
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples


    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    w: Vec3,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            transform: Mat4::make_identity(),
            aspect_ratio: ASPECT_RATIO,
            image_width: IMAGE_WIDTH,
            samples_per_pixel: 100,
            image_height: IMAGE_HEIGHT,
            center: CAMERA_CENTER,
            max_depth: 10,
            defocus_angle: 0.0,
            focus_distance: 1.0,
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            pixel00_loc: Point3::default(),
            pixel_samples_scale: 1.0 / 16.0,
            vfov: 90.0,
            eye: Vec3::new(0.0, 0.0, -1.0),
            look_at: Vec3::default(),
            up: Vec3::new(0.0, 1.0, 0.0),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
            w: Vec3::default(),
            u: Vec3::default(),
            v: Vec3::default(),
        }
    }

    pub fn render(&mut self, world: &HittableList, writer: &mut BufWriter<File>) -> io::Result<()> {
        self.initialize();

        writeln!(writer, "P3\n{} {}\n{}", self.image_width, self.image_height, MAX_COLOR)?;

        let pb = ProgressBar::new(((self.image_width * self.image_height) * self.samples_per_pixel).into());
        pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}%").unwrap());

        // Prepare all pixel coordinates
        let image_width = self.image_width;
        let image_height = self.image_height;
        let samples_per_pixel = self.samples_per_pixel;
        let max_depth = self.max_depth;
        let pixel_samples_scale = self.pixel_samples_scale;
        let camera = self.clone();
        let world = world;
        // Collect all pixel coordinates
        let pixels: Vec<(u32, u32)> = (0..image_height)
            .flat_map(|j| (0..image_width).map(move |i| (i, j)))
            .collect();

        // Parallel rendering
        let pixel_colors: Vec<Color> = pixels.par_iter().map(|&(i, j)| {
            let mut pixel_color = Color::default();
            for _ in 0..samples_per_pixel {
                let ray = camera.get_ray(i, j);
                pb.inc(1);
                pixel_color += camera.ray_color(&ray, max_depth, world);
            }
            pixel_color * pixel_samples_scale
        }).collect();

        // Write pixels sequentially
        for color in pixel_colors.iter() {
            color.write_color(writer).unwrap();
            pb.inc(1);
        }
        pb.finish();
        Ok(())
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.image_height = self.image_height.max(1);
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.eye;

        let theta = DEG_TO_RAD * self.vfov;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_distance;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = Vec3::unit_vector(&(self.eye - self.look_at));
        self.u = Vec3::unit_vector(&Vec3::cross(&self.up, &self.w));
        self.v = Vec3::cross(&self.w, &self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center - (self.focus_distance * self.w) - (viewport_u / 2.0) - (viewport_v / 2.0);
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_distance * ((DEG_TO_RAD * self.defocus_angle) / 2.0).tan();
        self.defocus_disk_u = defocus_radius * self.u;
        self.defocus_disk_v = defocus_radius * self.v;
    }

    pub fn defocus_disk_sample(&self) -> Vec3 {
        // Returns a random point in the camera defocus disk.
        let p = self.sample_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    fn ray_color(&self, r: &Ray, depth: u32, world: &HittableList) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered
        if depth <= 0 { return Color::default(); }

        let mut rec = HitRecord::default();
        if world.hit(r, &Interval::new(0.001, INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            if rec.material.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * self.ray_color(&scattered, depth - 1, world);
            }
            return Color::black();
        }
        // Sky gradient
        let unit_direction = Vec3::unit_vector(&r.direction);
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the origin
        // and directed at randomly sampled point around the pixel location i, j
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin;
        Ray { origin: ray_origin, direction: ray_direction }
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-0.5, -0.5]-[+0.5, +0.5] square around the origin
        Vec3::new(random() - 0.5, random() - 0.5, 0.0)
    }

    fn sample_disk(&self) -> Vec3 {
        // Returns a random point in the unit disk
        loop {
            let p = Vec3::new(random_range(-1.0, 1.0), random_range(-1.0, 1.0), 0.0);
            if p.length_squared() >= 1.0 { continue; }
            return p;
        }
    }

}

impl Clone for Camera {
    fn clone(&self) -> Self {
        Camera {
            transform: self.transform.clone(),
            aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
            image_height: self.image_height,
            center: self.center,
            pixel_delta_u: self.pixel_delta_u,
            pixel_delta_v: self.pixel_delta_v,
            pixel00_loc: self.pixel00_loc,
            pixel_samples_scale: self.pixel_samples_scale,
            up: self.up,
            defocus_disk_u: self.defocus_disk_u,
            defocus_disk_v: self.defocus_disk_v,
            vfov: self.vfov,
            eye: self.eye,
            look_at: self.look_at,
            defocus_angle: self.defocus_angle,
            focus_distance: self.focus_distance,
            w: self.w,
            u: self.u,
            v: self.v,
        }
    }
}