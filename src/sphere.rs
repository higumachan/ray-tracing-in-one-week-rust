use crate::bvh::aabb::AABB;
use crate::hit::{Hit, HitRecord};
use crate::material::material::Material;
use crate::ray::Ray;
use crate::vector3::{Point3, Vector3};
use std::f64::consts::PI;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

pub(crate) fn hit_sphere(
    center: &Point3,
    radius: f64,
    material: &Arc<dyn Material>,
    ray: &Ray,
    t_min: f64,
    t_max: f64,
) -> Option<HitRecord> {
    let oc = ray.origin() - center;
    let a = ray.direction().length_squared();
    let half_b = oc.dot(ray.direction());
    let c = oc.length_squared() - radius.powi(2);
    let discriminant = half_b.powi(2) - a * c;

    if discriminant < 0.0 {
        return None;
    }

    let sqrtd = discriminant.sqrt();

    let root = Some((-half_b - sqrtd) / a)
        .filter(|x| t_min <= *x && *x <= t_max)
        .or_else(|| Some((-half_b + sqrtd) / a).filter(|x| t_min <= *x && *x <= t_max));

    root.map(|r| {
        let p = ray.at(r);
        let outward_normal = (&p - center) / radius;
        let uv = get_sphere_uv(&Point3::from(outward_normal.clone()));
        HitRecord::new(p, r, uv.0, uv.1, outward_normal, ray, Arc::clone(material))
    })
}

pub(crate) fn sphere_bounding_box(center: &Point3, radius: f64) -> AABB {
    AABB::new(
        center - &Vector3::new(radius, radius, radius),
        center + &Vector3::new(radius, radius, radius),
    )
}

pub(crate) fn get_sphere_uv(p: &Point3) -> (f64, f64) {
    let theta = -p.y().acos();
    let phi = f64::atan2(-p.z(), p.x()) + PI;

    (phi / 2.0 * PI, theta / PI)
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        hit_sphere(&self.center, self.radius, &self.material, ray, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(sphere_bounding_box(&self.center, self.radius))
    }

    fn nearest_squared(&self, point: &Point3) -> f64 {
        (&self.center - point).length_squared() - self.radius.powi(2)
    }

    fn farest_squared(&self, point: &Point3) -> f64 {
        (&self.center - point).length_squared() + self.radius.powi(2)
    }
}
