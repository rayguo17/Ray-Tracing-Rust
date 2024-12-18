use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
    pub fn new()->Self{
        HitRecord{
            p:Point3::new(),
            normal: Vec3::new(),
            t:0.0,
            front_face:false // default false.
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t :&Interval, rec: &mut HitRecord) -> bool;
}
