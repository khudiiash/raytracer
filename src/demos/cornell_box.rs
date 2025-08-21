use crate::core::camera::Camera;
use crate::core::hittable::{Translate, RotateY};
use crate::core::hittable_list::HittableList;
use crate::materials::lambertian::Lambertian;
use crate::materials::diffuse_light::DiffuseLight;
use crate::math::bvh_node::BvhNode;
use crate::math::color::Color;
use crate::math::vec3::{Point3, Vec3};
use crate::sdf::sphere::Sphere;
use crate::sdf::quad::{make_box, Quad};
use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;

const OUTPUT_FILE: &str = "renders/cornell_box.ppm";

pub fn cornell_box() {
    let mut camera = Camera::new();
    let mut world = HittableList::new();

    // Materials
    let red   = Lambertian { albedo: Color::new(0.65, 0.05, 0.05) };
    let white = Lambertian { albedo: Color::new(0.73, 0.73, 0.73) };
    let green = Lambertian { albedo: Color::new(0.12, 0.45, 0.15) };
    let light = DiffuseLight { emit: Color::new(15.0, 15.0, 15.0) };

    // Geometry
    world.add(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Arc::new(green.clone()),
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Arc::new(red.clone()),
    ));
    world.add(Quad::new(
        Point3::new(343.0, 555.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        Arc::new(light),
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Arc::new(white.clone()),
    ));
    world.add(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        Arc::new(white.clone()),
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Arc::new(white.clone()),
    ));

    let box1 = make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        Arc::new(white.clone()),
    );
    let box1 = RotateY::new(Arc::new(box1),15.0);
    let box1 = Translate::new(Arc::new(box1), Vec3::new(265.0, 0.0, 295.0));

    let box2 = make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        Arc::new(white.clone()),
    );
    let box2 = RotateY::new(Arc::new(box2), -18.0);
    let box2 = Translate::new(Arc::new(box2), Vec3::new(130.0, 0.0, 65.0));

    world.add(box1);
    world.add(box2);

    // Camera settings
    camera.aspect_ratio = 1.0;
    camera.image_width = 300;
    camera.samples_per_pixel = 100;
    camera.max_depth = 5;
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
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Arc::new(white)));

    let bvh_node = BvhNode::new_from_list(world.objects.clone());
    let world = HittableList::from_bvh(bvh_node);

    // Render
    camera.render(&world, &mut writer).unwrap();
}