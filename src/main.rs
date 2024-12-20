use std::rc::Rc;

use camera::Camera;
use color::Color;
use hittable_list::HittableList;
use material::{Dielectric, Dummy, Lambertian, Material, Metal};
use rtweekend::{random_double, random_double_range};
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
    final_world();
    
}

fn final_world(){
    let mut world = HittableList::empty_new();
    let ground_material = Rc::new(Lambertian::new(&Color::from(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(Point3::from(0.0, -1000.0, 0.0), 1000.0, Rc::clone(&ground_material) as Rc<dyn Material>)));

    for a in -11..11{
        for b in -11..11{
            let choose_mat = random_double();
            let center = Point3::from(a as f64 + 0.9 * random_double(), 0.2, b as f64 + 0.9 * random_double());

            if ((center - Point3::from(4.0, 0.2, 0.0)).length())>0.9{
                let sphere_material: Rc<dyn Material>;

                if choose_mat < 0.8{
                    let albedo = Color::random() * Color::random();
                    sphere_material = Rc::new(Lambertian::new(&albedo)); // initialize here
                }else if choose_mat < 0.95 {
                    let albedo = Color::random_rng(0.5,1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    sphere_material = Rc::new(Metal::new(&albedo,fuzz));
                }else{
                    sphere_material = Rc::new(Dielectric::new(1.5));
                }
                world.add(Rc::new(Sphere::new(center, 0.2, Rc::clone(&sphere_material)))); // Does it consider as move?
            }
        }
    }

    let material1:Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(Point3::from(0.0, 1.0, 0.0), 1.0, material1)));

    let material2: Rc<dyn Material> = Rc::new(Lambertian::new(&Color::from(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(Point3::from(-4.0, 1.0, 0.0),1.0,material2)));

    let material3: Rc<dyn Material> = Rc::new(Metal::new(&Vec3::from(0.7,0.6,0.6), 0.0));
    world.add(Rc::new(Sphere::new(Point3::from(4.0, 1.0,0.0),1.0, material3)));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::from(13.0, 2.0, 3.0);
    cam.lookat = Point3::from(0.0, 0.0, 0.0);
    cam.vup = Vec3::from(0.0, 1.0, 0.0);


    cam.render(&world);
}

fn camera_world() {
    let mut world = HittableList::empty_new();

    let material_ground = Rc::new(Lambertian::new(&Color::from(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(&Color::from(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.50));
    let material_bubble = Rc::new(Dielectric::new(1.00/1.50));
    let material_right = Rc::new(Metal::new(&Color::from(0.8, 0.6, 0.2),1.0));

    world.add(Rc::new(Sphere::new(Point3::from(0.0, -100.5, 1.0), 100.0, Rc::clone(&material_ground) as Rc<dyn Material>)));
    world.add(Rc::new(Sphere::new(Point3::from(0.0, 0.0, -1.2), 0.5, Rc::clone(&material_center) as Rc<dyn Material>)));
    world.add(Rc::new(Sphere::new(Point3::from(-1.0, 0.0, -1.0),0.5, Rc::clone(&material_left) as Rc<dyn Material>)));
    world.add(Rc::new(Sphere::new(Point3::from(-1.0, 0.0, -1.0), 0.4, Rc::clone(&material_bubble) as Rc<dyn Material>)));
    world.add(Rc::new(Sphere::new(Point3::from(1.0, 0.0, -1.0),0.5, Rc::clone(&material_right) as Rc<dyn Material>)));
    

    let mut cam:Camera = Default::default();
    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    
    cam.vfov = 20.0;
    cam.lookfrom = Point3::from(-2.0, 2.0, 1.0);
    cam.lookat = Point3::from(0.0, 0.0, -1.0);
    cam.vup = Vec3::from(0.0, 1.0, 0.0);

    cam.defocus_angle = 10.0;
    cam.focus_dist = 3.4;

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
