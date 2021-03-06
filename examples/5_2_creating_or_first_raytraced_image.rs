use ray_tracing_in_one_week_rust::ray::Ray;
use ray_tracing_in_one_week_rust::vector3::{Color, Point3, Vector3};
use std::thread::sleep;
use std::time::Duration;

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(&Point3::new_z(-1.0), 0.5, ray) {
        Color::red()
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::from(&Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + &Vector3::new(0.5, 0.7, 1.0) * t)
    }
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction().length_squared();
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.length_squared() - radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * a * c;

    discriminant > 0.0
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400usize;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::zero();
    let horizontal = Vector3::new_x(viewport_width);
    let vertical = Vector3::new_y(viewport_height);
    let lower_left_corner = Vector3::from(origin.clone())
        - &horizontal / 2.0
        - &vertical / 2.0
        - Vector3::new_z(focal_length);

    // Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {:3}", j);
        for i in (0..image_width).rev() {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let hv = &horizontal * u;
            let vv = &vertical * v;
            let r = Ray::new(
                origin.clone(),
                &lower_left_corner + &hv + vv - Vector3::from(origin.clone()),
            );

            let pixel_color = ray_color(&r);

            println!("{}", pixel_color);
        }
    }
    eprintln!("\nDone");
}
