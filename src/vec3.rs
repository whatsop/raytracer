use std::ops::{self, Add, Div, Mul, Sub};

#[derive(Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    // initialize with default values
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    // initialize from values
    pub fn from(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    // get x value
    pub fn x(&self) -> f32 {
        self.x
    }

    // get y value
    pub fn y(&self) -> f32 {
        self.y
    }

    // get z value
    pub fn z(&self) -> f32 {
        self.z
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.x,
        }
    }
}
