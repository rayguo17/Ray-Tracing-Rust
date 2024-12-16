use crate::vec3::Vec3;

pub type Color = Vec3;

// maybe we want to print to different output streams....
pub fn write_color(color: &Color) {
    let r = color.x();
    let g = color.y();
    let b = color.z();
    let ir = (255.999 * r) as i32;
    let ig = (255.999 * g) as i32;
    let ib = (255.999 * b) as i32;
    println!("{} {} {}", ir, ig, ib);
}
