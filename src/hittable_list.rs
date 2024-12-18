use std::rc::Rc;

use crate::{hitable::{HitRecord, Hittable}, interval::Interval};

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>, // i will just assume the reference will not change, try to refactor when there are needs.
}

impl HittableList {
    pub fn empty_new() -> Self{
        HittableList { objects: vec![] }
    }
    pub fn one_new(object: Rc<dyn Hittable>)->Self{
        let mut new_empty = Self::empty_new();
        new_empty.add(object);
        return new_empty;
    }
    pub fn add(&mut self, object: Rc<dyn Hittable>){ // do we handle the clone inside or outside?
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, ray_t: &Interval, rec: &mut crate::hitable::HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut  closest_so_far = ray_t.max;
        for obj in self.objects.iter(){
            if obj.hit(r, &Interval::from(ray_t.min, closest_so_far), &mut temp_rec){
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;  // because of copy, this would be copied.
                //temp_rec = HitRecord::new(); // it is moved, so we need to reallocate
            }
        }
        return hit_anything;
    }
}