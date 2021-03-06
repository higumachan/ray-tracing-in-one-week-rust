use crate::ray::Ray;
use crate::vector3::Color;
use std::fmt::Debug;

struct ScatterResult {
    attenuation: Color,
    scattered: Ray,
}

pub trait Material: Debug {
    fn scatter(&self) -> Option<ScatterResult>;
}
