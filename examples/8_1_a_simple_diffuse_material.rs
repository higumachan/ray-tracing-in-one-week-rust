use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use ray_tracing_in_one_week_rust::camera::Camera;
use ray_tracing_in_one_week_rust::hit::Hit;
use ray_tracing_in_one_week_rust::hit_objects::HitObjects;
use ray_tracing_in_one_week_rust::ray::Ray;
use ray_tracing_in_one_week_rust::sphere::Sphere;
use ray_tracing_in_one_week_rust::vector3::{Color, Point3, Vector3};
use std::collections::hash_map::RandomState;
use std::f64::INFINITY;
use std::thread::sleep;
use std::time::Duration;

fn ray_color(rng: &mut ThreadRng, ray: &Ray, world: &HitObjects) -> Color {
    let rec = world.hit(ray, 0.0, INFINITY);
    rec.map(|r| {
        let target = r.point() + r.normal() + Vector3::random_in_unit_sphere(rng);
        let c = Vector3::from(ray_color(
            rng,
            &Ray::new(r.point().clone(), &target - r.point()),
            world,
        ));
        Color::from(c * 0.5)
    })
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
    let samples_per_pixel = 100;
    let mut rng = thread_rng();

    // World
    let mut world = HitObjects::new();
    world.add(Box::new(Sphere::new(Point3::new_z(-1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::default();

    // Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {:3}", j);
        for i in (0..image_width).rev() {
            let pixel_color: Vector3 = (0..samples_per_pixel)
                .map(|_| {
                    let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                    let r = camera.ray(u, v);

                    Vector3::from(ray_color(&mut rng, &r, &world)) / samples_per_pixel as f64
                })
                .sum::<Vector3>();
            let pixel_color = Color::from(pixel_color);

            println!("{}", pixel_color);
        }
    }
    eprintln!("\nDone");
}
