use super::vec3::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn at(self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }
}
