use std::sync::Arc;

use super::aabb::*;
use super::material::*;
use super::ray::*;
use super::vec3::*;

#[derive(Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
    pub material: Option<Arc<dyn Material>>,
    pub override_color: Option<Color>,
}

impl HitRecord {
    pub fn set_front_face_and_normal(&mut self, front_face: bool, outward_normal: Vec3) {
        self.front_face = front_face;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        let front_face = ray.dir.dot(outward_normal) < 0.0;
        self.set_front_face_and_normal(front_face, outward_normal);
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut AABB) -> bool;
}
