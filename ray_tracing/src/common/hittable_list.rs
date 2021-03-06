use std::sync::Arc;

use super::aabb::*;
use super::hittable::*;
use super::ray::*;

#[derive(Default)]
pub struct HittableList {
    pub items: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn add(&mut self, item: Arc<dyn Hittable>) {
        self.items.push(item)
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for hittable in &self.items {
            if hittable.hit(ray, t_min, closest_so_far, record) {
                hit_anything = true;
                closest_so_far = record.t;
            }
        }

        return hit_anything;
    }

    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut AABB) -> bool {
        let mut temp_box = AABB::default();
        let mut first_box = true;

        for handle in &self.items {
            if !handle.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box
            } else {
                *output_box & temp_box
            };
            first_box = false;
        }

        true
    }
}
