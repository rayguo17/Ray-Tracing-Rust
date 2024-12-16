use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

//geometric clarity
#[allow(dead_code)]
pub type Point3 = Vec3;

impl Vec3 {
    pub fn from(e1: f64, e2: f64, e3: f64) -> Vec3 {
        Vec3 { e: [e1, e2, e3] }
    }
    pub fn new() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }
    pub fn x(&self) -> f64 {
        self.e[0] // primitive type copy
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub const fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        // so here self is copy
        Vec3::from(-self.x(), -self.y(), -self.z())
    }
}

// right value
impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        &self.e[i] // what's the behaviour if out of bound? will panic
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
        // match index {
        //     0 => &mut self.e[0],
        //     1 => &mut self.e[1],
        //     2 => &mut self.e[2],
        //     _ => panic!("out of bound!"),
        // }
    }
}

impl ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::from(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        self + &rhs
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::from(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        self - &rhs
    }
}

impl ops::Mul<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::from(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::from(self * rhs.e[0], self * rhs.e[1], self * rhs.e[2])
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

// what does const do here?
#[inline]
pub const fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
}

// cross product with right hand rules.
#[inline]
pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::from(
        v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
        v1.e[2] * v2.e[0] - v1.e[0] * v2.e[2],
        v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0],
    )
}

#[inline] // if clone trait implemented, *v deref would be copy?
pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}
