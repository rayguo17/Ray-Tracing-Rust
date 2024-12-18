use std::rc::Rc;

use camera::Camera;
use hittable_list::HittableList;
use sphere::Sphere;
use vec3::{ Point3, Vec3};

pub mod color;
pub mod hitable;
pub mod hittable_list;
pub mod ray;
pub mod sphere;
pub mod vec3;
pub mod rtweekend;
pub mod interval;
pub mod camera;

#[allow(dead_code)]
fn is_clone<T: Clone>() {}



fn main() {
    let mut world = HittableList::empty_new();
    world.add(Rc::new(Sphere::new(Point3::from(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::from(0.0, -100.5, 1.0), 100.0)));

    let mut cam:Camera = Default::default();
    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 400;
    cam.render(&world);
    
}

#[allow(dead_code)]
fn gen_ppm() {
    let image_width = 256;
    let image_height = 256;
    println!("P3\n{} {}\n255", image_width, image_height);
    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let pixel_color = Vec3::from(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );
            color::write_color(&pixel_color);
        }
    }
}
