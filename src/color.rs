use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

// maybe we want to print to different output streams....
pub fn write_color(color: &Color) {
    let mut  r = color.x();
    let mut g = color.y();
    let mut b = color.z();
    r= linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity = Interval::from(0.000, 0.999);
    let ir = (256.0 * intensity.clamp(r)) as i32;
    let ig = (256.0 * intensity.clamp(g)) as i32;
    let ib = (256.0 * intensity.clamp(b)) as i32;
    println!("{} {} {}", ir, ig, ib);
}

#[inline]
pub fn linear_to_gamma(linear_component: f64) -> f64{
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    return 0.0;
}
