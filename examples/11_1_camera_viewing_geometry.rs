use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use ray_tracing_in_one_week_rust::camera::Camera;
use ray_tracing_in_one_week_rust::hit::Hit;
use ray_tracing_in_one_week_rust::hit_objects::HitObjects;
use ray_tracing_in_one_week_rust::material::dielectric::Dielectric;
use ray_tracing_in_one_week_rust::material::lambertian::Lambertian;
use ray_tracing_in_one_week_rust::material::metal::Metal;
use ray_tracing_in_one_week_rust::ray::Ray;
use ray_tracing_in_one_week_rust::sphere::Sphere;
use ray_tracing_in_one_week_rust::vector3::{Color, Point3, Vector3};
use std::collections::hash_map::RandomState;
use std::f64::consts::PI;
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
                .scatter(rng, ray, &r)
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

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400usize;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let mut rng = thread_rng();

    // World
    let R = f64::cos(PI / 4.0);

    let mut world = HitObjects::new();
    let material_left = Rc::new(Lambertian::new(Color::blue()));
    let material_right = Rc::new(Lambertian::new(Color::red()));

    world.add(Box::new(Sphere::new(
        Point3::new(-R, 0.0, -1.0),
        R,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(R, 0.0, -1.0),
        R,
        material_right,
    )));

    // Camera
    let camera = Camera::new(90.0, aspect_ratio);

    // Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {:3}", j);
        for i in (0..image_width) {
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
