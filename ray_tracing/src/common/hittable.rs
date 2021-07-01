use std::default::Default;
use std::sync::Arc;

use super::aabb::*;
use super::attribute::*;
use super::material::*;
use super::ray::*;
use super::vec2::*;
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

#[derive(Debug)]
pub enum GPUMaterialKind {
    Lambert,
    DiffuseLight,
}

impl GPUMaterialKind {
    pub fn to_f32(&self) -> f32 {
        match self {
            Self::Lambert => 0.0,
            Self::DiffuseLight => 1.0,
        }
    }
}

#[derive(Debug)]
pub enum GPUMaterialSide {
    Front,
    Back,
    Double,
}

impl GPUMaterialSide {
    pub fn to_f32(&self) -> f32 {
        match self {
            Self::Front => 0.0,
            Self::Back => 1.0,
            Self::Double => 2.0,
        }
    }
}

#[derive(Debug)]
pub struct GPUMaterial {
    pub kind: GPUMaterialKind,
    pub color: Vec3,
    pub side: GPUMaterialSide,
}

impl Default for GPUMaterial {
    fn default() -> Self {
        GPUMaterial {
            kind: GPUMaterialKind::Lambert,
            color: Color::new(0.0, 1.0, 0.0),
            side: GPUMaterialSide::Double,
        }
    }
}

#[derive(Debug)]
pub struct GPUPrimitive {
    pub position: Attribute<Vec3>,
    pub normal: Attribute<Vec3>,
    pub uv: Attribute<Vec2>,
    pub material: GPUMaterial,
}

#[derive(Debug)]
pub struct GPUBvhNode {
    pub aabb: AABB,
    pub left: usize,
    pub right: usize,
    pub primitive: Option<usize>,
}

#[derive(Debug, Default)]
pub struct GPUAcceleratedStructure {
    pub bvh: Vec<GPUBvhNode>,
    pub primitives: Vec<GPUPrimitive>,
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut AABB) -> bool;
    fn feed_gpu_bvh(&self, _acc: &mut GPUAcceleratedStructure) -> Option<usize> {
        None
    }
}

pub struct SkipGPUHittable {
    master: Arc<dyn Hittable>,
}

impl SkipGPUHittable {
    pub fn new(master: Arc<dyn Hittable>) -> Arc<Self> {
        Arc::new(SkipGPUHittable { master })
    }
}

impl Hittable for SkipGPUHittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        self.master.hit(ray, t_min, t_max, record)
    }

    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut AABB) -> bool {
        self.master.bounding_box(time0, time1, output_box)
    }

    fn feed_gpu_bvh(&self, _acc: &mut GPUAcceleratedStructure) -> Option<usize> {
        None
    }
}
