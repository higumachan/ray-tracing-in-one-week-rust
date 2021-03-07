use crate::hit::HitRecord;
use crate::material::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vector3::{Color, Vector3};
use rand::rngs::ThreadRng;
use rand::{thread_rng, RngCore};
use std::cell::RefCell;
use std::ops::DerefMut;

#[derive(Debug)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        input: &Ray,
        record: &HitRecord,
    ) -> Option<ScatterResult> {
        let scatter_direction = record.normal() + &Vector3::random_unit_vector(rng);

        let scatter_direction = if scatter_direction.approx_zero() {
            record.normal().clone()
        } else {
            scatter_direction
        };

        Some(ScatterResult::new(
            self.albedo.clone(),
            Ray::new(record.point().clone(), scatter_direction),
        ))
    }
}
