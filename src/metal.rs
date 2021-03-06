use crate::hit::HitRecord;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vector3::Color;

#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, input: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let reflected = input.direction().unit_vector().reflect(record.normal());
        let scattered = Ray::new(record.point().clone(), reflected);

        (scattered.direction().dot(record.normal()) > 0.0)
            .then(|| ScatterResult::new(self.albedo.clone(), scattered))
    }
}
