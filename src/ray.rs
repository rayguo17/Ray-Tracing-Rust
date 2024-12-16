use crate::vec3::{Point3, Vec3};
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new() -> Self {
        Ray {
            orig: Point3::new(),
            dir: Vec3::new(), // are there auto init way?
        }
    }

    // just borrow
    pub fn from(origin: &Point3, direction: &Vec3) -> Self {
        Ray {
            orig: *origin, // deref to copy
            dir: *direction,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}
