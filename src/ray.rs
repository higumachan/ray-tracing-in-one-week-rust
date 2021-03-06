use crate::vector3::{Point3, Vector3};

struct Ray {
    origin: Point3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Ray { origin, direction }
    }
    pub fn origin(&self) -> &Point3 {
        &self.origin
    }
    pub fn direction(&self) -> &Vector3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        let v = &self.direction * t;
        &self.origin + v
    }
}
