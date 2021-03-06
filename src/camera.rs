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
        origin: Point3,
        lower_left_corner: Point3,
        horizontal: Vector3,
        vertical: Vector3,
    ) -> Self {
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let hv = &self.horizontal * u;
        let vv = &self.vertical * v;
        Ray::new(
            self.origin.clone(),
            &Vector3::from(self.lower_left_corner.clone()) + &hv + vv
                - Vector3::from(self.origin.clone()),
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::zero();
        let horizontal = Vector3::new_x(viewport_width);
        let vertical = Vector3::new_y(viewport_height);
        let lower_left_corner =
            origin.clone() - &horizontal / 2.0 - &vertical / 2.0 - Vector3::new_z(focal_length);

        Self::new(origin, lower_left_corner, horizontal, vertical)
    }
}
