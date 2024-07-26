use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            origin: Vec3::new(),
            direction: Vec3::new(),
        }
    }

    pub fn from(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
        }
    }

    // pub fn point_at_parameter(t: )
}
