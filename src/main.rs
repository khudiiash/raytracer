
mod utils;
mod math;
mod core;
mod materials;
mod sdf;

use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;

use crate::core::material::Material;
use crate::core::{camera::Camera, hittable_list::*};
use crate::materials::dielectric::Dielectric;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::math::color::Color;
use crate::math::vec3::{Point3, Vec3};
use crate::sdf::Sphere;

const OUTPUT_FILE: &str = "output.ppm";

fn main() {
    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 512;
    camera.max_depth = 50;
    camera.defocus_angle = 0.6;
    camera.focus_distance = 10.0;
    camera.vfov = 20.0;
    camera.eye = Vec3::new(13.0, 2.0, 3.0);
    camera.look_at = Vec3::new(0.0, 0.0, 0.0);

    let mut world = HittableList::new();

    // Ground
    let ground_material = Arc::new(Lambertian { albedo: Color::new(0.5, 0.5, 0.5) });
    let ground = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material.clone());
    world.add(Box::new(ground));

    // // Random small spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utils::common::random();
            let center = Point3::new(
                a as f64 + 0.9 * utils::common::random(),
                0.2,
                b as f64 + 0.9 * utils::common::random(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material + Send + Sync> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    Arc::new(Lambertian { albedo })
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = utils::common::random_range(0.0, 0.5);
                    Arc::new(Metal { albedo, fuzz })
                } else {
                    // glass
                    Arc::new(Dielectric { ref_idx: 1.5 })
                };
                let sphere = Sphere::new(center, 0.2, sphere_material);
                world.add(Box::new(sphere));
            }
        }
    }

    // Three main spheres
    let material1 = Arc::new(Dielectric { ref_idx: 1.5 });
    world.add(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Arc::new(Lambertian { albedo: Color::new(0.4, 0.2, 0.1) });
    world.add(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(Metal { albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0 });
    world.add(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    let file = File::create(OUTPUT_FILE).unwrap();
    let mut writer = BufWriter::new(file);
    camera.render(&world, &mut writer).unwrap();
}
