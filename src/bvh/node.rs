use crate::bvh::aabb::AABB;
use crate::hit::{Hit, HitRecord};
use crate::hit_objects::HitObject;
use crate::ray::Ray;
use crate::vector3::Point3;
use rand::{Rng, RngCore};
use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Debug)]
pub enum Tree {
    Leaf(HitObject),
    Node(Box<Node>),
}

impl Hit for Tree {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Self::Leaf(ho) => ho.hit(ray, t_min, t_max),
            Self::Node(node) => node.hit(ray, t_min, t_max),
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        match self {
            Self::Leaf(ho) => ho.bounding_box(time0, time1),
            Self::Node(node) => node.bounding_box(time0, time1),
        }
    }

    fn nearest_squared(&self, point: &Point3) -> f64 {
        unimplemented!()
    }

    fn farest_squared(&self, point: &Point3) -> f64 {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct Node {
    left: Tree,
    right: Tree,
    bbox: AABB,
}

fn bbox_compare(axis: usize, a: &HitObject, b: &HitObject) -> Ordering {
    let a_bbox = a.bounding_box(0.0, 0.0).unwrap();
    let b_bbox = b.bounding_box(0.0, 0.0).unwrap();

    a_bbox
        .minimum()
        .element(axis)
        .partial_cmp(&b_bbox.minimum().element(axis))
        .unwrap()
}

impl Node {
    pub fn new<R: RngCore>(
        rng: &mut R,
        src_objects: &Vec<HitObject>,
        time0: f64,
        time1: f64,
    ) -> Option<Self> {
        Self::new_inner(rng, src_objects, 0, src_objects.len(), time0, time1)
    }

    fn new_inner<R: RngCore>(
        rng: &mut R,
        src_objects: &Vec<HitObject>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Option<Self> {
        let mut objects = src_objects.clone();
        let axis = rng.gen_range(0..3);

        let object_span = end - start;

        let (left, right) = match object_span {
            1 => (
                Tree::Leaf(objects[start].clone()),
                Tree::Leaf(objects[start].clone()),
            ),
            2 => {
                if bbox_compare(axis, &objects[start], &objects[start + 1]) == Ordering::Less {
                    (
                        Tree::Leaf(objects[start].clone()),
                        Tree::Leaf(objects[start + 1].clone()),
                    )
                } else {
                    (
                        Tree::Leaf(objects[start + 1].clone()),
                        Tree::Leaf(objects[start].clone()),
                    )
                }
            }
            _ => {
                objects.sort_by(|a, b| bbox_compare(axis, a, b));

                let mid = start + object_span / 2;
                (
                    Tree::Node(Box::new(Node::new_inner(
                        rng, &objects, start, mid, time0, time1,
                    )?)),
                    Tree::Node(Box::new(Node::new_inner(
                        rng, &objects, mid, end, time0, time1,
                    )?)),
                )
            }
        };

        let left_box = left.bounding_box(time0, time1)?;
        let right_box = right.bounding_box(time0, time1)?;

        Some(Self {
            left,
            right,
            bbox: left_box.surrounding_box(&right_box),
        })
    }
}

impl Hit for Node {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bbox.hit(ray, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(ray, t_min, t_max);
        let hit_right = self.right.hit(
            ray,
            t_min,
            hit_left.as_ref().map(|r| r.t()).unwrap_or(t_max),
        );

        hit_right.or(hit_left)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.bbox.clone())
    }

    fn nearest_squared(&self, point: &Point3) -> f64 {
        unimplemented!()
    }

    fn farest_squared(&self, point: &Point3) -> f64 {
        unimplemented!()
    }
}
