use crate::{
    hitable::{HitRecord, Hittable},
    ray::Ray,
    vec3::{dot, Point3},
};
pub struct Sphere {
    center: Point3,
    radius: f64,
}

pub fn maxnumf64(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        let radius = maxnumf64(0.0, radius);
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), &oc);
        let c = dot(&oc,&oc) - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let srtd = discriminant.sqrt();
        let mut root = (h - srtd) / a;
        // if root < t_min || t_max <= root {
        //     root = (h + srtd) / a; // try the other root
        //     if root < t_min || t_max <= root {
        //         return false; // no root in ranges.
        //     }
        // }
        rec.t = root;
        rec.p = r.at(rec.t);
        
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.normal = outward_normal;
        //rec.set_face_normal(r, &outward_normal);
        return true;
    }
}
