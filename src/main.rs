use color::{write_color, Color};
use ray::Ray;
use vec3::{dot, unit_vector, Point3, Vec3};

pub mod color;
pub mod hitable;
pub mod hittable_list;
pub mod ray;
pub mod sphere;
pub mod vec3;

#[allow(dead_code)]
fn is_clone<T: Clone>() {}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *center - r.origin();
    let a = r.direction().length_squared();
    let h = dot(r.direction(), &oc);
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: &Ray) -> Color {
    // depends on the ray direction return diff color.
    let t = hit_sphere(&Point3::from(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        // the distance between viewport and point on sphere.
        let N = unit_vector(&(r.at(t) - Vec3::from(0.0, 0.0, -1.0)));
        return 0.5 * Color::from(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0);
    }

    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    let white = Color::from(1.0, 1.0, 1.0);
    let blue = Color::from(0.5, 0.7, 1.0);
    return (1.0 - a) * white + a * blue;
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    image_height = image_height.max(1);

    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    //gen_ppm();

    let focal_length = 1.0;
    let camera_center = Point3::from(0.0, 0.0, 0.0);

    let viewport_u = Vec3::from(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::from(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3::from(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    println!("P3\n{} {}\n255", image_width, image_height);
    for j in 0..image_height {
        eprintln!("Scanlines remaining: {}\r", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + i as f64 * pixel_delta_u + j as f64 * pixel_delta_v;
            let ray_direction = pixel_center - camera_center; // direction should be unit vector.
            let r = Ray::from(&camera_center, &ray_direction);
            let pixel_color = ray_color(&r); // no reflection, directly the color of ray.
            write_color(&pixel_color);
        }
    }
}

#[allow(dead_code)]
fn gen_ppm() {
    let image_width = 256;
    let image_height = 256;
    println!("P3\n{} {}\n255", image_width, image_height);
    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let pixel_color = Vec3::from(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );
            color::write_color(&pixel_color);
        }
    }
}
