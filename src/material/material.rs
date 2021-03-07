use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vector3::Color;
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
    fn scatter<R: RngCore>(
        &self,
        rng: &mut R,
        input: &Ray,
        record: &HitRecord,
    ) -> Option<ScatterResult>;
}
