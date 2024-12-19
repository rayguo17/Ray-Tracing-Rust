use std::f64::INFINITY;

use crate::{color::{write_color, Color}, hitable::{HitRecord, Hittable}, hittable_list::HittableList, interval::Interval, ray::Ray, rtweekend::random_double, vec3::{random_on_hemisphere, unit_vector, Point3, Vec3}};


pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth:i32,
    image_height: i32,
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    center: Point3,  // Camera center
    pixel00_loc: Point3, // Location of pixel 0,0
    pixel_delta_u: Vec3, 
    pixel_delta_v: Vec3
}

impl Default for Camera{
    fn default() -> Self {
        Camera { max_depth:10,aspect_ratio: 1.0, image_width: 100, image_height: 0, center: Point3::new(), pixel00_loc: Point3::new(), pixel_delta_u: Vec3::new(), pixel_delta_v: Vec3::new(), samples_per_pixel: 10, pixel_samples_scale: 1.0 }
    }
}

impl Camera {
    fn ray_color(r :&Ray, depth: i32, world: &HittableList) -> Color{
        if depth <=0{
            return Color::new(); // zero by default
        }
        let mut rec = HitRecord::new();
        if world.hit(r, &Interval::from(0.001, INFINITY), &mut rec){
            let mut scattered = Ray::new();
            let mut attenuation = Color::new();
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered){
                return attenuation * Self::ray_color(&scattered, depth-1, world);
            }
            return Color::new();
            // let direction = random_on_hemisphere(&rec.normal) + rec.normal;
            // return 0.5 * Self::ray_color(&Ray::from(&rec.p, &direction),depth-1, world);
            //return 0.5 * (rec.normal + Color::from(1.0, 1.0, 1.0));
        }
        let unit_direction = unit_vector(r.direction());
        let white = Color::from(1.0, 1.0, 1.0);
        let blue = Color::from(0.5, 0.7, 1.0);
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0-a) * white + a * blue;
    }
    fn get_ray(&self,i:i32, j:i32)->Ray{ // get ray based on pixel index. || we will get multiple ray samples for the same pixel index.
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc + ((i as f64 + offset.x()) * self.pixel_delta_u) + ((j as f64 + offset.y()) * self.pixel_delta_v );
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        return Ray::from(&ray_origin, &ray_direction);
    }

    fn sample_square() -> Vec3{
        return Vec3::from(random_double() - 0.5, random_double()-0.5, 0.0);
    }

    pub fn render(&mut self, world: &HittableList){
        self.initialize();
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for j in 0..self.image_height{
            eprintln!("Scanlines remaining: {}\r", self.image_height - j);
            for i in 0..self.image_width{
                let mut pixel_color = Color::new();
                for _ in 0..self.samples_per_pixel{
                    let r = self.get_ray(i, j);
                    pixel_color += &Self::ray_color(&r, self.max_depth,world);
                }
                write_color(&(pixel_color * self.pixel_samples_scale));
            }
        }
        eprintln!("\rDone.         \n");
    }

    
    fn initialize(&mut self){
        self.image_height = ((self.image_width as f64) / self.aspect_ratio) as i32;
        self.image_height = self.image_height.max(1);
        self.pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);
        
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
