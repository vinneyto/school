use std::mem;
use std::ops;

use super::ray::*;
use super::vec3::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct AABB {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl AABB {
    pub fn new(minimum: Point3, maximum: Point3) -> Self {
        Self { minimum, maximum }
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f32, mut t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.dir[a];
            let mut t0 = (self.minimum[a] - ray.orig[a]) * inv_d;
            let mut t1 = (self.maximum[a] - ray.orig[a]) * inv_d;
            if inv_d < 0.0 {
                mem::swap(&mut t0, &mut t1);
            }
            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}

impl ops::BitAnd for AABB {
    type Output = Self;

    fn bitand(self, other: AABB) -> Self::Output {
        let minimum = self.minimum.min(other.minimum);
        let maximum = self.maximum.max(other.maximum);

        Self::new(minimum, maximum)
    }
}
