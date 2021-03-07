use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vector3::Color;
use rand::rngs::ThreadRng;
use rand::RngCore;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Ray,
}

impl ScatterResult {
    pub fn new(attenuation: Color, scattered: Ray) -> Self {
        ScatterResult {
            attenuation,
            scattered,
        }
    }
}

pub trait Material: Debug {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        input: &Ray,
        record: &HitRecord,
    ) -> Option<ScatterResult>;
}
