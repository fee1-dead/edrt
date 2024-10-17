pub mod camera;
pub mod vec3;
// (2)
pub mod ray;

use vec3::Color;

// (3)
fn write_color(out: &mut impl std::io::Write, c: Color) {
    todo!()
}

// (3)
#[test]
fn test_write_color() {
    let mut writer = Vec::new();
    write_color(&mut writer, Color(0.5, 1.0, 0.5));
    assert_eq!("127 255 127\n", String::from_utf8_lossy(&writer));
}

fn main() {
    let image_height = 256;
    let image_width = 256;

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");
    for j in 0..image_height {
        for i in 0..image_width {
            // (3)
            let r = j as f64 / (image_height - 1) as f64;
            let g = i as f64 / (image_width - 1) as f64;
            let b = 0.0;

            write_color(&mut std::io::stdout(), Color(r, g, b));
        }
    }
}
