use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Add<Color> for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: rhs.r * self,
            g: rhs.g * self,
            b: rhs.b * self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_add_color() {
        assert_eq!(
            Color::new(1.0, 0.0, 0.0),
            Color::new(0.5, 0.0, 0.0) + Color::new(0.5, 0.0, 0.0)
        )
    }

    #[test]
    fn color_mul_f32() {
        assert_eq!(Color::new(2.0, 2.0, 2.0), Color::new(1.0, 1.0, 1.0) * 2.0)
    }

    #[test]
    fn f32_mul_color() {
        assert_eq!(Color::new(2.0, 2.0, 2.0), 2.0 * Color::new(1.0, 1.0, 1.0))
    }
}
