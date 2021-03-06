use ray_tracing_in_one_week_rust::to_pixel_value;
use ray_tracing_in_one_week_rust::vector3::Color;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {:3}", j);
        for i in (0..image_width).rev() {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let color = Color::new(r, g, b);

            println!("{}", color);
        }
        sleep(Duration::from_millis(10));
    }
    eprintln!("\nDone");
}
