use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    // constructor
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn normalize(&self) -> Self {
        Self {
            x: self.x / self.length(),
            y: self.y / self.length(),
            z: self.z / self.length(),
        }
    }

    pub fn dot(vector_a: &Vec3, vector_b: &Vec3) -> f32 {
        vector_a.x * vector_b.x + vector_a.y * vector_b.y + vector_a.z * vector_b.z
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_add_vec3() {
        assert_eq!(
            Vec3::new(2.0, 2.0, 2.0),
            Vec3::new(1.0, 1.0, 1.0) + Vec3::new(1.0, 1.0, 1.0)
        );
    }

    #[test]
    fn vec3_sub_vec3() {
        assert_eq!(
            Vec3::new(2.0, 2.0, 2.0),
            Vec3::new(4.0, 4.0, 4.0) - Vec3::new(2.0, 2.0, 2.0)
        )
    }

    #[test]
    fn vec3_mul_f32() {
        assert_eq!(Vec3::new(2.0, 2.0, 2.0), Vec3::new(1.0, 1.0, 1.0) * 2.0)
    }

    #[test]
    fn f32_mul_vec3() {
        assert_eq!(Vec3::new(2.0, 2.0, 2.0), 2.0 * Vec3::new(1.0, 1.0, 1.0))
    }

    #[test]
    fn vec3_div_f32() {
        assert_eq!(Vec3::new(2.0, 2.0, 2.0), Vec3::new(4.0, 4.0, 4.0) / 2.0)
    }

    #[test]
    fn f32_div_vec3() {
        assert_eq!(Vec3::new(2.0, 2.0, 2.0), 8.0 / Vec3::new(4.0, 4.0, 4.0))
    }
}
