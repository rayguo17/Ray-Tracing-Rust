use std::f64::INFINITY;


pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Interval {
    pub fn new() -> Self {
        Interval { min: INFINITY, max: -INFINITY }
    }
    pub fn from(min:f64, max:f64)->Self{
        Interval {min: min,max:max}
    }
    pub fn size(&self) -> f64 {
        self.max-self.min
    }
    pub fn contains(&self, x: f64)->bool{
        self.min<=x && x<=self.max
    }
    pub fn surrounds(&self, x: f64)->bool{
        self.min<x && x < self.max
    }
    pub fn clamp(&self, x: f64)->f64 {
        if x<self.min {
            self.min
        }else if x>self.max {
            self.max
        }else{
            x
        }
    }
}

pub const EMPTY:Interval = Interval{
    min: INFINITY,
    max: -INFINITY
};

pub const UNIVERSE:Interval = Interval {
    min: -INFINITY,
    max: INFINITY
};