use std::ops::{Add, Div, Mul};

// (1)
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

// (6)
pub use Vec3 as Point3;
// (6)
pub use Vec3 as Color;

impl Vec3 {
    pub fn dot(self, other: Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
    // optional
    pub fn cross(self, other: Vec3) -> Vec3 {
        todo!()
    }
    // (2)
    pub fn length_squared(self) -> f64 {
        todo!()
    }
    // (3)
    pub fn length(self) -> f64 {
        todo!()
    }
    // (5)
    pub fn unit_vector(self) -> Vec3 {
        todo!()
    }
}

// (4)
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        todo!()
    }
}
// (4)
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
         todo!()
    }
}

// (4.5)
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        todo!()
    }
}

// (4.5)
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        todo!()
    }
}

#[test]
fn test_vec3() {
    let v1 = Vec3(1.0, 2.0, 3.0);
    let v2 = Vec3(4.0, 5.0, 6.0);

    assert_eq!(32.0, v1.dot(v2));
    // (2)
    assert_eq!(14.0, v1.length_squared());
    // (3)
    assert_eq!(13.0, Vec3(3.0, 4.0, 12.0).length());
    // (5)
    assert_eq!(Vec3(0.0, -1.0, 0.0), Vec3(0.0, -3.14, 0.0).unit_vector());
    // optional
    assert_eq!(Vec3(-3., 6., -3.), v1.cross(v2));
}

// (1)
#[test]
fn test_vec3_can_clone() {
    let v1 = Vec3(1.0, 1.0, 1.0);
    let v2 = v1.clone();
    assert_eq!(v1.0, v2.0);
}

// (1)
#[test]
fn test_vec3_can_copy() {
    let v1 = Vec3(1.0, 1.0, 1.0);
    let v2 = v1;
    assert_eq!(v1.0, v2.0);
}

// (1)
#[test]
fn test_vec3_can_debug() {
    let v = Vec3(1., 2., 3.);
    assert_eq!("Vec3(1.0, 2.0, 3.0)", format!("{v:?}"));
}

// (1)
#[test]
fn test_vec3_can_eq() {
    let v = Vec3(1.0, 2.0, 3.0);
    let v2 = Vec3(1.0, 2.0, 3.0);
    assert!(v == v2);
}