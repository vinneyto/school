use std::cmp::Ordering;
use std::sync::Arc;

use rand::prelude::*;

use super::aabb::*;
use super::arena::*;
use super::hittable::*;
use super::ray::*;

#[derive(Debug)]
pub struct BVHNode {
    pub left: HittableHandle,
    pub right: HittableHandle,
    pub aabb: AABB,
}

impl BVHNode {
    pub fn new(
        arena: &mut HittableArena,
        objects: &[HittableHandle],
        time0: f32,
        time1: f32,
    ) -> Self {
        let axis: usize = rand::thread_rng().gen_range(0..3);
        let comparator = |a, b| box_compare(arena, a, b, axis);

        let size = objects.len();
        let left;
        let right;

        match size {
            1 => {
                left = objects[0];
                right = objects[0];
            }
            2 => match comparator(objects[0], objects[1]) {
                Ordering::Greater => {
                    left = objects[0];
                    right = objects[1]
                }
                _ => {
                    left = objects[1];
                    right = objects[0]
                }
            },
            _ => {
                let mut sorted = objects
                    .iter()
                    .map(|a| a.clone())
                    .collect::<Vec<HittableHandle>>();

                sorted.sort_by(|a, b| comparator(*a, *b));

                let mid = size / 2;
                let start = 0;

                let left_node = Arc::new(BVHNode::new(arena, &sorted[start..mid], time0, time1));
                let right_node = Arc::new(BVHNode::new(arena, &sorted[mid..size], time0, time1));

                left = arena.insert(left_node);
                right = arena.insert(right_node);
            }
        };

        let mut box_left = AABB::default();
        let mut box_right = AABB::default();

        let has_box_a = arena.bounding_box(left, time0, time1, &mut box_left);
        let has_box_b = arena.bounding_box(right, time0, time1, &mut box_right);

        if !has_box_a || !has_box_b {
            panic!("No bounding box in bvh_node constructor")
        }

        let aabb = box_left & box_right;

        Self { left, right, aabb }
    }
}

impl Hittable for BVHNode {
    fn hit(
        &self,
        arena: &HittableArena,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        record: &mut HitRecord,
    ) -> bool {
        if !self.aabb.hit(ray, t_min, t_max) {
            return false;
        }

        let hit_left = arena.hit(self.left, ray, t_min, t_max, record);
        let t_max_right = if hit_left { record.t } else { t_max };
        let hit_right = arena.hit(self.right, ray, t_min, t_max_right, record);

        hit_left || hit_right
    }

    fn bounding_box(
        &self,
        _arena: &HittableArena,
        _time0: f32,
        _time1: f32,
        output_box: &mut AABB,
    ) -> bool {
        *output_box = self.aabb;

        true
    }
}

fn box_compare(
    arena: &HittableArena,
    a: HittableHandle,
    b: HittableHandle,
    axis: usize,
) -> Ordering {
    let mut box_a = AABB::default();
    let mut box_b = AABB::default();

    let has_box_a = arena.bounding_box(a, 0.0, 0.0, &mut box_a);
    let has_box_b = arena.bounding_box(b, 0.0, 0.0, &mut box_b);

    if !has_box_a || !has_box_b {
        panic!("No bounding box in bvh_node constructor.");
    };

    if box_a.minimum[axis] < box_b.minimum[axis] {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}
