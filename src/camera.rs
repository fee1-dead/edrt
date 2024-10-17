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

// (5)
impl CameraBuilder {
    pub fn build(self) -> Camera {
        let CameraBuilder { image_width, aspect_ratio, focal_length, center } = self;

        let image_height = (image_width as f64 / aspect_ratio) as u64;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let viewport_u = Vec3(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        let viewport_upper_left = center - Vec3(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
        let pixel00_loc = viewport_upper_left + 0.5*(pixel_delta_u + pixel_delta_v);
        Camera {
            image_width,
            image_height,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            center,
        }
    }
}

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
