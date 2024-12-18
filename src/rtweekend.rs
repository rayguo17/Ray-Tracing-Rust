
const INFINITY: f64 = std::f64::INFINITY;
const PI:f64 = std::f64::consts::PI;

#[inline]
pub fn degrees_to_radians(degrees:f64)->f64{
    return degrees * PI / 180.0;
}

#[inline]
pub fn random_double() -> f64 {
    return rand::random(); 
}

#[inline]
pub fn random_double_range(min:f64, max:f64) -> f64 {
    return min + (max-min)*random_double();
}