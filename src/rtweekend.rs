
const INFINITY: f64 = std::f64::INFINITY;
const PI:f64 = std::f64::consts::PI;

#[inline]
pub fn degrees_to_radians(degrees:f64)->f64{
    return degrees * PI / 180.0;
}