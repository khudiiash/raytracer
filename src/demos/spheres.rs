use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::lambertian::Lambertian;
use crate::math::vec3::{Point3, Vec3, Vec3Ext};
use crate::sdf::sphere::Sphere;
use crate::core::hittable_list::HittableList;
use crate::core::camera::Camera;
use crate::math::color::Color;
use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;

const OUTPUT_FILE: &str = "renders/spheres.ppm";

pub fn spheres() {
    use crate::materials::metal::Metal;
    use crate::materials::dielectric::Dielectric;
    use crate::utils::common::{random, random_range};

    let mut world = HittableList::new();

    // Ground
    let ground_material = Arc::new(Lambertian { albedo: Color::new(0.5, 0.5, 0.5) });
    world.add(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material.clone())));

    // Random small spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Point3::new(
                a as f32 + 0.9 * random(),
                0.2,
                b as f32 + 0.9 * random(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian { albedo });
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal { albedo, fuzz });
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric { ref_idx: 1.5 });
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    // Three big spheres
    let material1 = Arc::new(Dielectric { ref_idx: 1.5 });
    world.add(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Arc::new(Lambertian { albedo: Color::new(0.4, 0.2, 0.1) });
    world.add(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(Metal { albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0 });
    world.add(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    // Camera
    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;
    camera.vfov = 20.0;
    camera.eye = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.up = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.6;
    camera.focus_distance = 10.0;
    camera.background = Color::new(0.70, 0.80, 1.00);

    let file = File::create(OUTPUT_FILE).unwrap();
    let mut writer = BufWriter::new(file);
    camera.render(&world, &mut writer).unwrap();
}