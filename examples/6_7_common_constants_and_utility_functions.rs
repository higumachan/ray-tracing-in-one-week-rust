use ray_tracing_in_one_week_rust::hit::Hit;
use ray_tracing_in_one_week_rust::hit_objects::HitObjects;
use ray_tracing_in_one_week_rust::ray::Ray;
use ray_tracing_in_one_week_rust::sphere::Sphere;
use ray_tracing_in_one_week_rust::vector3::{Color, Point3, Vector3};
use std::f64::INFINITY;
use std::thread::sleep;
use std::time::Duration;

fn ray_color(ray: &Ray, world: &HitObjects) -> Color {
    let rec = world.hit(ray, 0.0, INFINITY);
    rec.map(|r| Color::from((r.normal() + &Vector3::one()) * 0.5))
        .unwrap_or_else(|| {
            let unit_direction = ray.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            Color::from(&Vector3::one() * (1.0 - t) + &Vector3::new(0.5, 0.7, 1.0) * t)
        })
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = ray.origin() - center;
    let a = ray.direction().length_squared();
    let half_b = oc.dot(ray.direction());
    let c = oc.length_squared() - radius.powi(2);
    let discriminant = half_b.powi(2) - a * c;

    (discriminant >= 0.0).then(|| (-half_b - discriminant.sqrt()) / a)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400usize;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    // World
    let mut world = HitObjects::new();
    world.add(Box::new(Sphere::new(Point3::new_z(-1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

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

            let pixel_color = ray_color(&r, &world);

            println!("{}", pixel_color);
        }
    }
    eprintln!("\nDone");
}
