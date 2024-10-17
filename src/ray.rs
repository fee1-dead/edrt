use crate::vec3::Point3;
use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    // (1)
    pub fn at(self, t: f64) -> Point3 {
        todo!()
    }
}

// (1)
#[test]
fn test_ray_at() {
    let ray = Ray {
        origin: Point3(0.0, 0.0, 0.0),
        direction: Vec3(1.0, 1.0, 1.0),
    };

    assert_eq!(Vec3(0.5, 0.5, 0.5), ray.at(0.5));
}
