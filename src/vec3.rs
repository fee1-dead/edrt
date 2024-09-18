pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn dot(self, other: Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
}

#[test]
fn test_vec3() {
    let v1 = Vec3(1.0, 2.0, 3.0);
    let v2 = Vec3(4.0, 5.0, 6.0);

    assert_eq!(32.0, v1.dot(v2));
}