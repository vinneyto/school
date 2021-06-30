use std::cmp::Ordering;
use std::sync::Arc;

use rand::prelude::*;

use super::aabb::*;
use super::hittable::*;
use super::ray::*;

pub struct BVHNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub aabb: AABB,
}

impl BVHNode {
    pub fn new(objects: &[Arc<dyn Hittable>], time0: f32, time1: f32) -> Self {
        let axis: usize = rand::thread_rng().gen_range(0..3);

        let size = objects.len();
        let left;
        let right;

        match size {
            1 => {
                left = objects[0].clone();
                right = objects[0].clone();
            }
            2 => match box_compare(&objects[0], &objects[1], axis) {
                Ordering::Greater => {
                    left = objects[0].clone();
                    right = objects[1].clone();
                }
                _ => {
                    left = objects[1].clone();
                    right = objects[0].clone();
                }
            },
            _ => {
                let mut sorted = objects
                    .iter()
                    .map(|a| a.clone())
                    .collect::<Vec<Arc<dyn Hittable>>>();

                sorted.sort_by(|a, b| box_compare(a, b, axis));

                let mid = size / 2;
                let start = 0;

                left = Arc::new(BVHNode::new(&sorted[start..mid], time0, time1));
                right = Arc::new(BVHNode::new(&sorted[mid..size], time0, time1));
            }
        };

        let mut box_left = AABB::default();
        let mut box_right = AABB::default();

        let has_box_a = left.bounding_box(time0, time1, &mut box_left);
        let has_box_b = right.bounding_box(time0, time1, &mut box_right);

        if !has_box_a || !has_box_b {
            panic!("No bounding box in bvh_node constructor")
        }

        let aabb = box_left & box_right;

        Self { left, right, aabb }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        if !self.aabb.hit(ray, t_min, t_max) {
            return false;
        }

        let hit_left = self.left.hit(ray, t_min, t_max, record);
        let t_max_right = if hit_left { record.t } else { t_max };
        let hit_right = self.right.hit(ray, t_min, t_max_right, record);

        hit_left || hit_right
    }

    fn bounding_box(&self, _time0: f32, _time1: f32, output_box: &mut AABB) -> bool {
        *output_box = self.aabb;

        true
    }

    fn feed_gpu_bvh(&self, acc: &mut GPUAcceleratedStructure) -> Option<usize> {
        let left_index = self.left.feed_gpu_bvh(acc);
        let right_index = self.right.feed_gpu_bvh(acc);

        if left_index.is_some() && right_index.is_some() {
            acc.bvh.push(GPUBvhNode {
                aabb: self.aabb,
                left: left_index.unwrap(),
                right: right_index.unwrap(),
                primitive: None,
            });
            let bvh_node_index = acc.bvh.len() - 1;

            return Some(bvh_node_index);
        }

        return None;
    }
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
    let mut box_a = AABB::default();
    let mut box_b = AABB::default();

    let has_box_a = a.bounding_box(0.0, 0.0, &mut box_a);
    let has_box_b = b.bounding_box(0.0, 0.0, &mut box_b);

    if !has_box_a || !has_box_b {
        panic!("No bounding box in bvh_node constructor.");
    };

    if box_a.minimum[axis] < box_b.minimum[axis] {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}
