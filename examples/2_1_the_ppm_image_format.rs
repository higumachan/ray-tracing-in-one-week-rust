fn to_pixel_value(c: f64) -> u8 {
    (c * 255.999) as u8
}

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        for i in (0..image_width).rev() {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;

            println!(
                "{} {} {}",
                to_pixel_value(r),
                to_pixel_value(g),
                to_pixel_value(b)
            );
        }
    }
}
