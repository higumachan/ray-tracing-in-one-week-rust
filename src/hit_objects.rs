use crate::hit::{Hit, HitRecord};
use crate::ray::Ray;

pub struct HitObjects(Vec<Box<dyn Hit>>);

impl HitObjects {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn new_one(object: Box<dyn Hit>) -> Self {
        Self(vec![object])
    }

    fn clear(&mut self) {
        self.0.clear()
    }

    fn add(&mut self, object: Box<dyn Hit>) {
        self.0.push(object)
    }
}

impl Hit for HitObjects {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.0.iter().fold(None, |acc, obj| {
            obj.hit(ray, t_min, acc.map(|x| x.t()).unwrap_or(t_max))
                .or(acc)
        })
    }
}
