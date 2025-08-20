use crate::core::camera::Camera;
use crate::core::hittable_list::HittableList;
use crate::materials::lambertian::Lambertian;
use crate::materials::diffuse_light::DiffuseLight;
use crate::math::color::Color;
use crate::math::vec3::{Point3, Vec3};
use crate::sdf::sphere::Sphere;
use crate::sdf::Quad;
use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;
use crate::sdf::cuboid::cuboid;

const OUTPUT_FILE: &str = "renders/cornell_box.ppm";

pub fn cornell_box() {
    let mut camera = Camera::new();
    let mut world = HittableList::new();

    // Materials
    let red   = Arc::new(Lambertian { albedo: Color::new(0.65, 0.05, 0.05) });
    let white = Arc::new(Lambertian { albedo: Color::new(0.73, 0.73, 0.73) });
    let green = Arc::new(Lambertian { albedo: Color::new(0.12, 0.45, 0.15) });
    let light = Arc::new(DiffuseLight { emit: Color::new(15.0, 15.0, 15.0) });

    // Geometry
    world.add(Box::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    world.add(Box::new(cuboid(Point3::new(130.0, 0.0, 65.0), Point3::new(295.0, 165.0, 230.0), white.clone())));
    world.add(Box::new(cuboid(Point3::new(265.0, 0.0, 295.0), Point3::new(430.0, 330.0, 460.0), white.clone())));

    // Camera settings
    camera.aspect_ratio = 1.0;
    camera.image_width = 600;
    camera.samples_per_pixel = 256;
    camera.max_depth = 50;
    camera.background = Color::new(0.0, 0.0, 0.0);

    camera.vfov = 40.0;
    camera.eye = Point3::new(278.0, 278.0, -800.0);
    camera.look_at = Point3::new(278.0, 278.0, 0.0);
    camera.up = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;

    // File
    let file = File::create(OUTPUT_FILE).unwrap();
    let mut writer = BufWriter::new(file);

    // test sphere
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, white.clone())));

    // Render
    camera.render(&world, &mut writer).unwrap();
}