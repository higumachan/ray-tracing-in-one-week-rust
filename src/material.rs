use crate::ray::Ray;
use crate::vector3::Color;

struct ScatterResult {
    attenuation: Color,
    scattered: Ray,
}

trait Material {
    fn scatter() -> Option<ScatterResult>;
}
