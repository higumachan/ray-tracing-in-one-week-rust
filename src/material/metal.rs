use crate::hit::HitRecord;
use crate::material::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vector3::{Color, Vector3};
use rand::rngs::ThreadRng;
use rand::{thread_rng, RngCore};
use std::cell::RefCell;
use std::ops::DerefMut;

#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter<R: RngCore>(
        &self,
        rng: &mut R,
        input: &Ray,
        record: &HitRecord,
    ) -> Option<ScatterResult> {
        let reflected = input.direction().unit_vector().reflect(record.normal());
        let scattered = Ray::new(
            record.point().clone(),
            reflected + Vector3::random_in_unit_sphere(rng) * self.fuzz,
        );

        (scattered.direction().dot(record.normal()) > 0.0)
            .then(|| ScatterResult::new(self.albedo.clone(), scattered))
    }
}
