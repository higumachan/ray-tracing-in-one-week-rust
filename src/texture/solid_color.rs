use crate::texture::texture::Texture;
use crate::vector3::{Color, Point3};

#[derive(Debug)]
pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color_value: Color) -> Self {
        SolidColor { color_value }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.color_value.clone()
    }
}
