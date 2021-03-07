use crate::hit::HitRecord;
use crate::material::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vector3::Color;

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
    fn scatter(&self, input: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let refraction_ratio = if record.front_face() {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = input.direction().unit_vector();
        let refracted = unit_direction.refract(record.normal(), refraction_ratio);

        Some(ScatterResult::new(
            Color::white(),
            Ray::new(record.point().clone(), refracted),
        ))
    }
}
