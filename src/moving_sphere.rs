use crate::bvh::aabb::AABB;
use crate::hit::{Hit, HitRecord};
use crate::material::material::Material;
use crate::ray::Ray;
use crate::sphere::hit_sphere;
use crate::vector3::Point3;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    radius: f64,
    time0: f64,
    time1: f64,
    material: Arc<dyn Material>,
}

impl Hit for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let center = self.center(ray.time());
        hit_sphere(&center, self.radius, &self.material, ray, t_min, t_max)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {}

    fn nearest_squared(&self, point: &Point3) -> f64 {
        unimplemented!()
    }

    fn farest_squared(&self, point: &Point3) -> f64 {
        unimplemented!()
    }
}

impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        radius: f64,
        time0: f64,
        time1: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        MovingSphere {
            center0,
            center1,
            radius,
            time0,
            time1,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        &self.center0
            + (&self.center1 - &self.center0) * ((time - self.time0) / (time - self.time1))
    }
}
