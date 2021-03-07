use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use ray_tracing_in_one_week_rust::camera::Camera;
use ray_tracing_in_one_week_rust::hit::Hit;
use ray_tracing_in_one_week_rust::hit_objects::HitObjects;
use ray_tracing_in_one_week_rust::lambertian::Lambertian;
use ray_tracing_in_one_week_rust::metal::Metal;
use ray_tracing_in_one_week_rust::ray::Ray;
use ray_tracing_in_one_week_rust::sphere::Sphere;
use ray_tracing_in_one_week_rust::vector3::{Color, Point3, Vector3};
use std::collections::hash_map::RandomState;
use std::f64::INFINITY;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;

fn ray_color(rng: &mut ThreadRng, ray: &Ray, world: &HitObjects, depth: usize) -> Color {
    if depth == 0 {
        return Color::black();
    }

    let rec = world.hit(ray, 0.001, INFINITY);
    rec.map(|r| {
        Color::from(
            r.material()
                .scatter(ray, &r)
                .map(|result| {
                    Vector3::from(result.attenuation).hadamard_product(&Vector3::from(ray_color(
                        rng,
                        &result.scattered,
                        world,
                        depth - 1,
                    )))
                })
                .unwrap_or_else(Vector3::zero),
        )
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
    let max_depth = 50;
    let mut rng = thread_rng();

    // World
    let mut world = HitObjects::new();
    let material_ground = Rc::new(Lambertian::new(Color::new_all(0.8)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Color::new_all(0.8), 0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new_z(-1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

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

                    Vector3::from(ray_color(&mut rng, &r, &world, max_depth))
                        / samples_per_pixel as f64
                })
                .sum::<Vector3>();
            let pixel_color = Color::from(pixel_color.sqrt());
            println!("{}", pixel_color);
        }
    }
    eprintln!("\nDone");
}
