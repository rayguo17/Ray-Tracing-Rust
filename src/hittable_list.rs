use crate::hitable::Hittable;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}
