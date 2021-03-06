use crate::hit::{Hit, HitRecord};
use crate::ray::Ray;

pub struct HitObjects(Vec<Box<dyn Hit>>);

impl HitObjects {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn new_one(object: Box<dyn Hit>) -> Self {
        Self(vec![object])
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn add(&mut self, object: Box<dyn Hit>) {
        self.0.push(object)
    }
}

impl Hit for HitObjects {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.0.iter().fold(None, |acc, obj| {
            let c = acc.clone();
            obj.hit(ray, t_min, acc.map(|x| x.t()).unwrap_or(t_max))
                .or(c)
        })
    }
}
