use crate::ray::Ray;
use crate::vector3::{Point3, Vector3};

pub struct HitRecord {
    point: Point3,
    normal: Vector3,
    t: f64,
}

impl HitRecord {
    pub fn new(point: Point3, normal: Vector3, t: f64) -> Self {
        HitRecord { point, normal, t }
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
