use std::f64::consts::PI;

pub mod camera;
pub mod hit;
pub mod hit_objects;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vector3;

pub fn to_pixel_value(c: f64) -> u8 {
    (256.0 * c.clamp(0.0, 0.999)) as u8
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
