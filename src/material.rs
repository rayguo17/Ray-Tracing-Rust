use crate::{color::Color, hitable::HitRecord, ray::Ray, rtweekend::random_double, vec3::{dot, random_unit_vector, reflect, refract, unit_vector, Vec3}};


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
    albedo: Color,
    fuzz: f64
}

impl Metal {
    pub fn new(albe: &Color,fuzz: f64)->Self { // fuzz should be smaller than 1
        Metal { albedo: *albe, fuzz: fuzz.min(1.0) }
    }
}

impl Material for Metal {
    fn scatter(&self,r_in : &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut reflected = reflect(r_in.direction(), &rec.normal);
        reflected = unit_vector(&reflected) + (self.fuzz * random_unit_vector());
        *scattered = Ray::from(&rec.p, &reflected);
        
        *attenuation = self.albedo;
        return true;
    }
}

pub struct Dielectric {
    refraction_index: f64
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        return Dielectric{
            refraction_index
        }
    }
    pub fn reflectance(cosine:f64, refraction_index: f64)->f64{
        let mut  r0 = (1.0-refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        return r0 + (1.0-r0) * (1.0-cosine).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(&self,r_in : &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::from(1.0, 1.0, 1.0);
        let ri = match rec.front_face {
            true => 1.0/self.refraction_index,
            false => self.refraction_index
        };
        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0-cos_theta*cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random_double() {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, ri)
        };
        
        *scattered = Ray::from(&rec.p, &direction);
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