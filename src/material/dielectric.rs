use crate::hit::HitRecord;
use crate::material::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vector3::{Color, Vector3};
use rand::rngs::ThreadRng;
use rand::{Rng, RngCore};

#[derive(Debug, Clone)]
pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Dielectric {
            index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        input: &Ray,
        record: &HitRecord,
    ) -> Option<ScatterResult> {
        let refraction_ratio = if record.front_face() {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = input.direction().unit_vector();
        let cos_theta = f64::min(Vector3::dot(&(-(&unit_direction)), record.normal()), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>() {
                unit_direction.reflect(record.normal())
            } else {
                unit_direction.refract(record.normal(), refraction_ratio)
            };

        Some(ScatterResult::new(
            Color::white(),
            Ray::new(record.point().clone(), direction, input.time()),
        ))
    }
}

fn reflectance(cos: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}
