use crate::degrees_to_radians;
use crate::ray::Ray;
use crate::vector3::{Point3, Vector3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        up_vector: Vector3,
        vertical_fov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = degrees_to_radians(vertical_fov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (&look_from - &look_at).unit_vector();
        let u = up_vector.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin.clone() - &horizontal / 2.0 - &vertical / 2.0 - w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn ray(&self, s: f64, t: f64) -> Ray {
        let hv = &self.horizontal * s;
        let vv = &self.vertical * t;
        Ray::new(
            self.origin.clone(),
            &Vector3::from(self.lower_left_corner.clone()) + &hv + vv
                - Vector3::from(self.origin.clone()),
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Point3::zero(),
            Point3::new_z(-1.0),
            Vector3::new_y(1.0),
            2.0,
            16.0 / 9.0,
        )
    }
}
