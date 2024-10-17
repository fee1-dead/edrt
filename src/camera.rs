use crate::vec3::{Vec3, Point3};

// (4)
#[derive(PartialEq, Debug)]
pub struct Camera {
    image_width: u64,
    image_height: u64,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    center: Point3,
}

// (5)
pub struct CameraBuilder {
    pub image_width: u64,
    pub aspect_ratio: f64,
    pub focal_length: f64,
    pub center: Point3,
}

// (6)
impl CameraBuilder {
    pub fn build(self) -> Camera {
        todo!()
    }
}

// (6)
#[test]
fn test_camera_builder() {
    let expected_camera = Camera {
        image_width: 16,
        image_height: 8,
        pixel00_loc: Point3(-1.875, 0.875, -1.0),
        pixel_delta_u: Vec3(0.25, 0.0, 0.0),
        pixel_delta_v: Vec3(0.0, -0.25, 0.0),
        center: Vec3(0.0, 0.0, 0.0),
    };

    let builder = CameraBuilder {
        image_width: 16,
        aspect_ratio: 2.0,
        focal_length: 1.0,
        center: Vec3(0.0, 0.0, 0.0),
    };

    let camera = builder.build();
    assert_eq!(expected_camera, camera);
}
