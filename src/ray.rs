use crate::vec3::Point3;
use crate::vec3::Vec3;

// (8)
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

// (9)
impl Ray {
    pub fn at(self, t: f64) -> Point3 {
        todo!()
    }
}