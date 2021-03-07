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
    let mut world = HitObjects::new();
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

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
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
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
