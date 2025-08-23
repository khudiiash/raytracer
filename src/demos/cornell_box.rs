use crate::core::camera::Camera;
use crate::core::hittable::{Translate, RotateY};
use crate::core::hittable_list::HittableList;
use crate::core::material::Material;
use crate::materials::lambertian::Lambertian;
use crate::materials::diffuse_light::DiffuseLight;
use crate::math::bvh_node::BvhNode;
use crate::math::color::Color;
use crate::math::vec3::{Point3, Vec3};
use crate::sdf::quad::{make_box, Quad};
use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;

const OUTPUT_FILE: &str = "renders/cornell_box.ppm";
const RED: Color = Color::new(0.65, 0.05, 0.05);
const WHITE: Color = Color::new(0.73, 0.73, 0.73);
const GREEN: Color = Color::new(0.12, 0.45, 0.15);
const LIGHT: Color = Color::new(14.352, 9.828, 6.24);
static red: Lambertian = Lambertian { albedo: RED };
static white: Lambertian = Lambertian { albedo: WHITE };
static green:Lambertian = Lambertian { albedo: GREEN };
static light:DiffuseLight = DiffuseLight { emit: LIGHT };

pub fn cornell_box() {
    let mut camera = Camera::new();
    let mut world = HittableList::new();

    // Materials

    // Light
    world.add(Quad::new(
        Point3::new(343.0, 548.8, 227.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 104.0),
        &light
    ));
    // Geometry
    world.add(Quad::new(Point3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), &red as &dyn Material));
    world.add(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), &green as &dyn Material));
    world.add(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), &white as &dyn Material));
    world.add(Quad::new(Point3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), &white as &dyn Material));
    world.add(Quad::new(Point3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), &white as &dyn Material));

    // Boxes
    let box1 = make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        &white
    );
    let box1 = RotateY::new(Arc::new(box1),15.0);
    let box1 = Translate::new(Arc::new(box1), Vec3::new(265.0, 0.0, 295.0));

    let box2 = make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        &white
    );
    let box2 = RotateY::new(Arc::new(box2), -18.0);
    let box2 = Translate::new(Arc::new(box2), Vec3::new(130.0, 0.0, 65.0));

    world.add(box1);
    world.add(box2);

    // Camera settings
    camera.aspect_ratio = 1.0;
    camera.image_width = 600;
    camera.samples_per_pixel = 1000;
    camera.background = Color::new(0.0, 0.0, 0.0);

    camera.vfov = 40.0;
    camera.eye = Point3::new(278.0, 273.0, -800.0);
    camera.look_at = Point3::new(278.0, 273.0, 0.0);
    camera.up = Vec3::new(0.0, 1.0, 0.0);

    // File
    let file = File::create(OUTPUT_FILE).unwrap();
    let mut writer = BufWriter::new(file);

    let bvh_node = BvhNode::new_from_list(world.objects.clone());
    let world = HittableList::from_bvh(bvh_node);

    // Render
    camera.render(&world, &mut writer).unwrap();
}
