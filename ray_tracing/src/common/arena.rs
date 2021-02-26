use generational_arena::{Arena, Index, Iter};
use std::sync::Arc;

use super::aabb::*;
use super::hittable::*;
use super::material::*;
use super::ray::*;

macro_rules! define_arenas {
    {$(
        $handle:ident,
        $arena:ident,
        $item:ty;
    )*} => {

$( // START Structure definitions

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
pub struct $handle {
    pub handle: Index
}

pub struct $arena {
    arena: Arena<$item>
}

impl $arena {
    pub fn new() -> Self {
        Self { arena: Arena::new() }
    }

    pub fn insert(&mut self, item: $item) -> $handle {
        let handle = self.arena.insert(item);

        $handle { handle }
    }

    pub fn get(&self, index: $handle) -> Option<&$item> {
        self.arena.get(index.handle)
    }

    pub fn get_mut(&mut self, index: $handle) -> Option<&mut $item> {
        self.arena.get_mut(index.handle)
    }

    pub fn remove(&mut self, index: $handle) -> Option<$item> {
        self.arena.remove(index.handle)
    }

    pub fn iter(&self) -> Iter<$item> {
        self.arena.iter()
    }
}
)* // END Structure definitions

    }
}

define_arenas! {
    MaterialHandle, MaterialArena, Arc<dyn Material>;
    HittableHandle, HittableArena, Arc<dyn Hittable>;
}

impl HittableArena {
    pub fn all_handles(&self) -> Vec<HittableHandle> {
        self.iter()
            .map(|(handle, _)| HittableHandle { handle })
            .collect()
    }

    pub fn hit(
        &self,
        handle: HittableHandle,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        record: &mut HitRecord,
    ) -> bool {
        let hittable = self.get(handle).unwrap();

        hittable.hit(self, ray, t_min, t_max, record)
    }

    pub fn bounding_box(
        &self,
        handle: HittableHandle,
        time0: f32,
        time1: f32,
        output_box: &mut AABB,
    ) -> bool {
        let hittable = self.get(handle).unwrap();

        hittable.bounding_box(self, time0, time1, output_box)
    }
}
