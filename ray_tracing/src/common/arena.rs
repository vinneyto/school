use generational_arena::{Arena, Index};
use std::sync::Arc;

use super::hittable::Hittable;
use super::material::Material;

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
}
)* // END Structure definitions

    }
}

define_arenas! {
    MaterialHandle, MaterialArena, Arc<dyn Material>;
    HittableHandle, HittableArena, Arc<dyn Hittable>;
}
