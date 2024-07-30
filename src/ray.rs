use crate::point3::Point3;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    // constructor
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
        }
    }

    // pub fn point_at_parameter(t: )
}
