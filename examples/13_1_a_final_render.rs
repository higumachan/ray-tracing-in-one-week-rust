use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use ray_tracing_in_one_week_rust::camera::Camera;
use ray_tracing_in_one_week_rust::hit::{Hit, HitRecord};
use ray_tracing_in_one_week_rust::hit_objects::{HitObject, HitObjects};
use ray_tracing_in_one_week_rust::material::dielectric::Dielectric;
use ray_tracing_in_one_week_rust::material::lambertian::Lambertian;
use ray_tracing_in_one_week_rust::material::material::Material;
use ray_tracing_in_one_week_rust::material::metal::Metal;
use ray_tracing_in_one_week_rust::ray::Ray;
use ray_tracing_in_one_week_rust::sphere::Sphere;
use ray_tracing_in_one_week_rust::vector3::{Color, Point3, Vector3};
use std::collections::hash_map::RandomState;
use std::f64::INFINITY;
use std::sync::Arc;
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

fn random_scene(rng: &mut ThreadRng) -> HitObjects {
    let mut world = HitObjects::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(HitObject::Sphere(Sphere::new(
        Point3::new_y(-1000.0),
        1000.0,
        ground_material,
    )));

    for a in -11..12 {
        for b in -11..12 {
            let choose_mat = rng.gen::<f64>();
            let center = Point3::new(
                (a as f64) + 0.9 * rng.gen::<f64>(),
                0.2,
                (b as f64) + 0.9 * rng.gen::<f64>(),
            );

            if (&center - &Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Arc<dyn Material> = if choose_mat < 0.8 {
                    let albedo =
                        Color::from(Vector3::random(rng).hadamard_product(&Vector3::random(rng)));
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(rng, 0.5..1.0);
                    let fuzz = rng.gen();
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    Arc::new(Dielectric::new(1.5))
                };
                world.add(HitObject::Sphere(Sphere::new(center, 0.2, material)));
            }
        }
    }

    let material = Arc::new(Dielectric::new(1.5));
    world.add(HitObject::Sphere(Sphere::new(
        Point3::new_y(1.0),
        1.0,
        material,
    )));

    let material = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(HitObject::Sphere(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(HitObject::Sphere(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    world
}

fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 400usize;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let mut rng = thread_rng();

    // World
    let world = random_scene(&mut rng);

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::zero();
    let vup = Vector3::new_y(1.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

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
                    let r = camera.ray(&mut rng, u, v);

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
