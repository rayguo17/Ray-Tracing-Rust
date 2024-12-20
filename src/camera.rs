use std::{arch::x86_64::_CMP_FALSE_OQ, f64::INFINITY};

use crate::{color::{write_color, Color}, hitable::{HitRecord, Hittable}, hittable_list::HittableList, interval::Interval, ray::Ray, rtweekend::{degrees_to_radians, random_double}, vec3::{cross, random_in_unit_disk, unit_vector, Point3, Vec3}};


pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth:i32,
    pub vfov : f64,
    pub lookfrom : Point3,
    pub lookat : Point3,
    pub vup : Vec3,
    pub defocus_angle : f64,
    pub focus_dist:f64,
    image_height: i32,
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    center: Point3,  // Camera center
    pixel00_loc: Point3, // Location of pixel 0,0
    pixel_delta_u: Vec3, 
    pixel_delta_v: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    u: Vec3,
    v: Vec3,
    w : Vec3
}

impl Default for Camera{
    fn default() -> Self {
        Camera { focus_dist:10.0,defocus_angle:0.0,lookfrom: Point3::new(), lookat: Point3::from(0.0, 0.0, -1.0), vup: Vec3::from(0.0, 1.0, 0.0),vfov: 90.0,max_depth:10,aspect_ratio: 1.0, image_width: 100, image_height: 0, center: Point3::new(), pixel00_loc: Point3::new(), pixel_delta_u: Vec3::new(), pixel_delta_v: Vec3::new(), samples_per_pixel: 10, pixel_samples_scale: 1.0, u:Vec3::new(), v: Vec3::new(), w: Vec3::new(), defocus_disk_u: Vec3::new(),defocus_disk_v:Vec3::new()}
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
        let ray_origin = match self.defocus_angle<=0.0 {
            true=> self.center,
            false => self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        return Ray::from(&ray_origin, &ray_direction);
    }
    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        return self.center + (p.x() * self.defocus_disk_u) + p.y() * self.defocus_disk_v;
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
        
        self.center = self.lookfrom;
        
        // Determine viewport dimensions.
        let focal_length = (self.lookfrom - self.lookat).length();
        let theta = degrees_to_radians(self.vfov);
        let h = (theta/2.0).tan();

        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * ((self.image_width as f64)/self.image_height as f64);

        self.w = unit_vector(&(self.lookfrom - self.lookat));
        self.u = unit_vector(&cross(&self.vup,& self.w));
        self.v = cross(&self.w, &self.u);
        // Calculate the vectors across the horizontal and down the vertical viewpoint edges.
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * (-self.v);
        
        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.center - self.focus_dist* self.w - viewport_u/2.0 - viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
        
        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle/2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

}
