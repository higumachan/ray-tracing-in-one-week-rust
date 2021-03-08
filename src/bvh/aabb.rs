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
        let v1 = self
            .minimum
            .as_vector()
            .zip_elements(ray.origin().as_vector())
            .map(|(a, b)| a - b)
            .zip(ray.inv_direction().iter_elements())
            .map(|(a, b)| a * b);
        let v2 = self
            .maximum
            .as_vector()
            .zip_elements(ray.origin().as_vector())
            .map(|(a, b)| a - b)
            .zip(ray.inv_direction().iter_elements())
            .map(|(a, b)| a * b);

        v1.zip(v2)
            .zip(ray.inv_direction().iter_elements())
            .all(|((a, b), inv_d)| {
                let (t0, t1) = if *inv_d < 0.0 { (b, a) } else { (a, b) };

                t_min = f64::max(t_min, t0);
                t_max = f64::min(t_max, t1);

                t_min < t_max
            })
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
