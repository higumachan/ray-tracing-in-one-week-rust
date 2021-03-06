use std::f64::consts::PI;

pub mod hit;
pub mod hit_objects;
pub mod ray;
pub mod sphere;
pub mod vector3;

pub fn to_pixel_value(c: f64) -> u8 {
    (c * 255.999) as u8
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
