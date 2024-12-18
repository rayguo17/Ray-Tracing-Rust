use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

// maybe we want to print to different output streams....
pub fn write_color(color: &Color) {
    let r = color.x();
    let g = color.y();
    let b = color.z();

    let intensity = Interval::from(0.000, 0.999);
    let ir = (256.0 * intensity.clamp(r)) as i32;
    let ig = (256.0 * intensity.clamp(g)) as i32;
    let ib = (256.0 * intensity.clamp(b)) as i32;
    println!("{} {} {}", ir, ig, ib);
}
