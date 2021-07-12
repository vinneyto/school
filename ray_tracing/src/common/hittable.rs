use partial_min_max::{max as fmax, min as fmin};

use std::default::Default;
use std::sync::Arc;

use super::aabb::*;
use super::helpers::*;
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

pub struct Translate {
    pub hittable: Arc<dyn Hittable>,
    pub displacement: Vec3,
}

impl Translate {
    pub fn new(hittable: Arc<dyn Hittable>, displacement: Vec3) -> Self {
        Self {
            hittable,
            displacement,
        }
    }

    pub fn arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let moved_r = Ray::new(ray.orig - self.displacement, ray.dir);

        if !self.hittable.hit(&moved_r, t_min, t_max, rec) {
            return false;
        }

        rec.p += self.displacement;
        rec.set_face_normal(&moved_r, rec.normal);

        true
    }

    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut AABB) -> bool {
        if !self.hittable.bounding_box(time0, time1, output_box) {
            return false;
        }

        *output_box = AABB::new(
            output_box.minimum + self.displacement,
            output_box.maximum + self.displacement,
        );

        true
    }
}

pub struct RotateY {
    pub hittable: Arc<dyn Hittable>,
    pub sin_theta: f32,
    pub cos_theta: f32,
    pub has_box: bool,
    pub bbox: AABB,
}

impl RotateY {
    pub fn new(hittable: Arc<dyn Hittable>, angle: f32) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = AABB::default();
        let has_box = hittable.bounding_box(0.0, 1.0, &mut bbox);

        let mut min = Point3::new(std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY);
        let mut max = Point3::new(
            -std::f32::INFINITY,
            -std::f32::INFINITY,
            -std::f32::INFINITY,
        );

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * bbox.maximum.x + bbox.minimum.x * (1 - i) as f32;
                    let y = j as f32 * bbox.maximum.y + bbox.minimum.y * (1 - j) as f32;
                    let z = k as f32 * bbox.maximum.z + bbox.minimum.z * (1 - k) as f32;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = fmin(min[c], tester[c]);
                        max[c] = fmax(max[c], tester[c]);
                    }
                }
            }
        }

        let bbox = AABB::new(min, max);

        Self {
            hittable,
            sin_theta,
            cos_theta,
            has_box,
            bbox,
        }
    }

    pub fn arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut origin = ray.orig;
        let mut direction = ray.dir;

        origin[0] = self.cos_theta * ray.orig[0] - self.sin_theta * ray.orig[2];
        origin[2] = self.sin_theta * ray.orig[0] + self.cos_theta * ray.orig[2];

        direction[0] = self.cos_theta * ray.dir[0] - self.sin_theta * ray.dir[2];
        direction[2] = self.sin_theta * ray.dir[0] + self.cos_theta * ray.dir[2];

        let rotated_r = Ray::new(origin, direction);

        if !self.hittable.hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.p;
        let mut normal = rec.normal;

        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;
        rec.set_face_normal(&rotated_r, normal);

        return true;
    }

    fn bounding_box(&self, _time0: f32, _time1: f32, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        self.has_box
    }
}
