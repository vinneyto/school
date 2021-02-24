use super::aabb::*;
use super::arena::*;
use super::hittable::*;
use super::ray::*;

#[derive(Default)]
pub struct HittableList {
    pub handles: Vec<HittableHandle>,
}

impl HittableList {
    pub fn new(arena: &HittableArena) -> Self {
        let handles = arena
            .iter()
            .map(|(handle, _)| HittableHandle { handle })
            .collect();

        Self { handles }
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        arena: &HittableArena,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        record: &mut HitRecord,
    ) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for handle in &self.handles {
            if arena
                .get(*handle)
                .unwrap()
                .hit(arena, ray, t_min, closest_so_far, record)
            {
                hit_anything = true;
                closest_so_far = record.t;
            }
        }

        return hit_anything;
    }

    fn bounding_box(
        &self,
        arena: &HittableArena,
        time0: f32,
        time1: f32,
        output_box: &mut AABB,
    ) -> bool {
        let mut temp_box = AABB::default();
        let mut first_box = true;

        for handle in &self.handles {
            if !arena
                .get(*handle)
                .unwrap()
                .bounding_box(arena, time0, time1, &mut temp_box)
            {
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
