use crate::vector3::{Color, Point3};
use std::fmt::Debug;

pub trait Texture: Debug + Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
