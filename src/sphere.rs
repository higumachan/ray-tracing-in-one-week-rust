use crate::hit::{Hit, HitRecord};
use crate::ray::Ray;
use crate::vector3::Point3;

struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - &self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            return None;
        }

        let p = ray.at(root);
        let normal = (&p - &self.center) / self.radius;
        Some(HitRecord::new(p, normal, root))
    }
}