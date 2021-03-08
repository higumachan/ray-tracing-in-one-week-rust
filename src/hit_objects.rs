use crate::bvh::aabb::AABB;
use crate::camera::Camera;
use crate::hit::{Hit, HitRecord};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector3::Point3;
use unwrap_ord::UnwrapOrd;

#[derive(Clone)]
pub enum HitObject {
    Sphere(Sphere),
}

impl Hit for HitObject {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Self::Sphere(s) => s.hit(ray, t_min, t_max),
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        match self {
            Self::Sphere(s) => s.bounding_box(time0, time1),
        }
    }

    fn nearest_squared(&self, point: &Point3) -> f64 {
        match self {
            Self::Sphere(s) => s.nearest_squared(point),
        }
    }

    fn farest_squared(&self, point: &Point3) -> f64 {
        match self {
            Self::Sphere(s) => s.farest_squared(point),
        }
    }
}

pub struct HitObjects(Vec<HitObject>);

impl HitObjects {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn new_one(object: HitObject) -> Self {
        Self(vec![object])
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn add(&mut self, object: HitObject) {
        self.0.push(object)
    }

    pub fn indexing_from_camera(&mut self, camera: &Camera) {
        let origin = camera.origin();

        self.0
            .sort_by_cached_key(|x| UnwrapOrd(x.nearest_squared(origin)));
    }
}

impl Hit for HitObjects {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut record: Option<HitRecord> = None;
        for obj in &self.0 {
            let t = record.as_ref().map(|x| x.t()).unwrap_or(t_max);
            let distance_sq = obj.farest_squared(ray.origin());

            record = obj.hit(ray, t_min, t).or(record.clone());
        }
        record
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if self.0.is_empty() {
            return None;
        }

        let mut result_box: Option<AABB> = None;

        for obj in &self.0 {
            if let Some(bbox) = obj.bounding_box(time0, time1) {
                result_box = result_box.map(|x| x.surrounding_box(&bbox)).or(Some(bbox));
            } else {
                return None;
            }
        }

        result_box
    }

    fn nearest_squared(&self, point: &Point3) -> f64 {
        unimplemented!()
    }

    fn farest_squared(&self, point: &Point3) -> f64 {
        unimplemented!()
    }
}
