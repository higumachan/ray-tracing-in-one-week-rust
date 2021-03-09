use crate::ray::Ray;
use crate::vector3::{Point3, Vector3};
use std::cmp::min;
use std::mem::swap;
use std::ops::{Div, Sub};

#[derive(Debug, Clone)]
pub struct AABB {
    minimum: Point3,
    maximum: Point3,
}

impl AABB {
    pub fn minimum(&self) -> &Point3 {
        &self.minimum
    }
    pub fn maximum(&self) -> &Point3 {
        &self.maximum
    }
}

impl AABB {
    pub fn new(minimum: Point3, maximum: Point3) -> Self {
        AABB { minimum, maximum }
    }

    pub(crate) fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;

        for a in 0..3 {
            let t0 = (self.minimum().element(a) - ray.origin().element(a))
                * ray.inv_direction().element(a);
            let t1 = (self.maximum().element(a) - ray.origin().element(a))
                * ray.inv_direction().element(a);

            let (t0, t1) = if ray.inv_direction().element(a) < 0.0 {
                (t1, t0)
            } else {
                (t0, t1)
            };

            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(&self, other: &Self) -> Self {
        let small = Vector3::new_from_iter(
            self.minimum
                .as_vector()
                .zip_elements(other.minimum.as_vector())
                .map(|(a, b)| f64::min(*a, *b)),
        );
        let big = Vector3::new_from_iter(
            self.maximum
                .as_vector()
                .zip_elements(other.maximum.as_vector())
                .map(|(a, b)| f64::max(*a, *b)),
        );

        AABB::new(Point3::from(small), Point3::from(big))
    }
}
