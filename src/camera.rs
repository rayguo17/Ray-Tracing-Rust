use std::f64::INFINITY;

use crate::{color::{write_color, Color}, hitable::{HitRecord, Hittable}, hittable_list::HittableList, interval::Interval, ray::Ray, vec3::{unit_vector, Point3, Vec3}};


pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3
}

impl Default for Camera{
    fn default() -> Self {
        Camera { aspect_ratio: 1.0, image_width: 100, image_height: 0, center: Point3::new(), pixel00_loc: Point3::new(), pixel_delta_u: Vec3::new(), pixel_delta_v: Vec3::new() }
    }
}

impl Camera {
    fn ray_color(r :&Ray, world: &HittableList) -> Color{
        let mut rec = HitRecord::new();
        if world.hit(r, &Interval::from(0.0, INFINITY), &mut rec){
            return 0.5 * (rec.normal + Color::from(1.0, 1.0, 1.0));
        }
        let unit_direction = unit_vector(r.direction());
        let white = Color::from(1.0, 1.0, 1.0);
        let blue = Color::from(0.5, 0.7, 1.0);
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0-a) * white + a * blue;
    }

    pub fn render(&mut self, world: &HittableList){
        self.initialize();
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for j in 0..self.image_height{
            eprintln!("Scanlines remaining: {}\r", self.image_height - j);
            for i in 0..self.image_width{
                let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::from(&self.center, &ray_direction); // taking reference but turn into copy.
                let pixel_color = Self::ray_color(&r, world);
                write_color(&pixel_color);
            }
        }
        eprintln!("\rDone.         \n");
    }

    fn initialize(&mut self){
        self.image_height = ((self.image_width as f64) / self.aspect_ratio) as i32;
        self.image_height = self.image_height.max(1);
        self.center = Point3::new();
        
        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * ((self.image_width as f64)/self.image_height as f64);
        
        // Calculate the vectors across the horizontal and down the vertical viewpoint edges.
        let viewport_u = Vec3::from(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::from(0.0, -viewport_height, 0.0);
        
        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.center - Vec3::from(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

}
