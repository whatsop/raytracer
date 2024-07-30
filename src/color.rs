use std::ops::{self, Add};

#[derive(Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    // constructor
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    // convert values to bytes
    pub fn as_bytes(&self) -> [i32; 3] {
        [
            (255.0 * self.r) as i32,
            (255.0 * self.g) as i32,
            (255.0 * self.b) as i32,
        ]
    }
}

impl ops::Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}
