use crate::vector3::{Point3, Vector3};

#[derive(Debug, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vector3,
    inv_direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        let inv_direction = direction.invert();
        Ray {
            origin,
            direction,
            inv_direction,
        }
    }
    pub fn origin(&self) -> &Point3 {
        &self.origin
    }
    pub fn direction(&self) -> &Vector3 {
        &self.direction
    }
    pub fn inv_direction(&self) -> &Vector3 {
        &self.inv_direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        let v = &self.direction * t;
        &self.origin + v
    }
}
