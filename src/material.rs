use crate::{color::Color, hitable::HitRecord, ray::Ray, vec3::{random_unit_vector, reflect}};


pub struct Dummy;

impl Material for Dummy{
    fn scatter(&self,r_in : &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        return false;
    }
}

pub trait Material {
    fn scatter(&self,r_in : &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        return false; //default implementation.
    }
}

pub struct Metal{
    albedo: Color
}

impl Metal {
    pub fn new(albe: &Color)->Self {
        Metal { albedo: *albe }
    }
}

impl Material for Metal {
    fn scatter(&self,r_in : &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = reflect(r_in.direction(), &rec.normal);
        *scattered = Ray::from(&rec.p, &reflected);
        *attenuation = self.albedo;
        return true;
    }
}


pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(color: &Color) -> Self {
        Lambertian{
            albedo: *color
        }
    }
}

impl Material for Lambertian { // does the color valid?
    fn scatter(&self,r_in : &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();   //TODO: should this be random on hemi sphere vector????
        if scatter_direction.near_zero(){
            scatter_direction = rec.normal;
        }
        *scattered = Ray::from(&rec.p, &scatter_direction);
        *attenuation = self.albedo;
        return true;
    }   
}