use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use ray_tracing_in_one_week_rust::bvh::node::Node;
use ray_tracing_in_one_week_rust::camera::Camera;
use ray_tracing_in_one_week_rust::hit::Hit;
use ray_tracing_in_one_week_rust::hit_objects::{HitObject, HitObjects};
use ray_tracing_in_one_week_rust::material::dielectric::Dielectric;
use ray_tracing_in_one_week_rust::material::lambertian::Lambertian;
use ray_tracing_in_one_week_rust::material::material::Material;
use ray_tracing_in_one_week_rust::material::metal::Metal;
use ray_tracing_in_one_week_rust::sphere::Sphere;
use ray_tracing_in_one_week_rust::vector3::{Color, Point3, Vector3};
use std::f64::INFINITY;
use std::sync::Arc;

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

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();
    let world = random_scene(&mut rng);
    let bvh = Node::new(&mut rng, &world.0, 0.0, 0.0).unwrap();

    let aspect_ratio = 3.0 / 2.0;
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

    let rays: Vec<_> = (0..1000)
        .into_iter()
        .map(|_| {
            let u = rng.gen::<f64>();
            let v = rng.gen::<f64>();
            camera.ray(&mut rng, u, v)
        })
        .collect();

    c.bench_function("world_hit normal", |b| {
        b.iter(|| {
            let mut hit = 0;
            for ray in &rays {
                hit += world.hit(ray, 0.001, INFINITY).is_some() as i32;
            }
        })
    });

    c.bench_function("world_hit bvh", |b| {
        b.iter(|| {
            let mut hit = 0;
            for ray in &rays {
                hit += bvh.hit(ray, 0.001, INFINITY).is_some() as i32;
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
