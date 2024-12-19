use std::rc::Rc;

use camera::Camera;
use color::Color;
use hittable_list::HittableList;
use material::{Lambertian, Material, Metal};
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
pub mod material;

#[allow(dead_code)]
fn is_clone<T: Clone>() {}



fn main() {
    let mut world = HittableList::empty_new();

    let material_ground = Rc::new(Lambertian::new(&Color::from(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(&Color::from(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Metal::new(&Color::from(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(&Color::from(0.8, 0.6, 0.2)));

    world.add(Rc::new(Sphere::new(Point3::from(0.0, -100.5, 1.0), 100.0, Rc::clone(&material_ground) as Rc<dyn Material>)));
    world.add(Rc::new(Sphere::new(Point3::from(0.0, 0.0, -1.2), 0.5, Rc::clone(&material_center) as Rc<dyn Material>)));
    world.add(Rc::new(Sphere::new(Point3::from(-1.0, 0.0, -1.0),0.5, Rc::clone(&material_left) as Rc<dyn Material>)));
    world.add(Rc::new(Sphere::new(Point3::from(1.0, 0.0, -1.0),0.5, Rc::clone(&material_right) as Rc<dyn Material>)));
    

    let mut cam:Camera = Default::default();
    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.render(&world);
    
}

fn test_color_main(){
    let mut col = Color::new();
    color_modi(&mut col);
}

fn color_modi(col : &mut Color){

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
