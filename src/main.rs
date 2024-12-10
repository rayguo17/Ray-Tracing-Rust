use vec3::Vec3;

pub mod vec3;

#[allow(dead_code)]
fn is_clone<T: Clone>() {}
fn main() {
    let mut vec = Vec3::from(-0.1, -0.2, -0.3);
    let vec2 = -vec;
    vec[1] = 2.0;
    vec *= 2.0;
    println!("{:?}", vec);
    vec /= 2.0;
    println!("{:?}", vec);
    println!("Hello, world!");
}

#[allow(dead_code)]
fn gen_ppm() {
    let image_width = 256;
    let image_height = 256;
    println!("P3\n{} {}\n255", image_width, image_height);
    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.00;
            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
