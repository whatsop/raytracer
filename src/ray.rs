use crate::color::Color;
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    // constructor
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    // return the color of the scene ray
    pub fn color(&self) -> Color {
        let unit_direction = self.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    pub fn at(&self, t: f32) -> Vec3 {
        let result = self.origin + t * self.direction;
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_at() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(Vec3::new(2.0, 0.0, 0.0), ray.at(2.0))
    }
}
