use crate::material::material::Material;
use crate::ray::Ray;
use crate::vector3::{Point3, Vector3};
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct HitRecord {
    point: Point3,
    normal: Vector3,
    material: Arc<dyn Material>,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn point(&self) -> &Point3 {
        &self.point
    }
    pub fn normal(&self) -> &Vector3 {
        &self.normal
    }
    pub fn t(&self) -> f64 {
        self.t
    }
    pub fn front_face(&self) -> bool {
        self.front_face
    }
    pub fn material(&self) -> &Arc<dyn Material> {
        &self.material
    }
}

impl HitRecord {
    pub fn new(
        point: Point3,
        t: f64,
        outward_normal: Vector3,
        ray: &Ray,
        material: Arc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hit: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn distance_squared(&self, point: &Point3) -> f64;
}
