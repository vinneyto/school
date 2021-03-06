use std::sync::Arc;

use super::aabb::*;
use super::hittable::*;
use super::material::*;
use super::ray::*;
use super::vec3::*;

pub struct Triangle {
    pub a: Point3,
    pub b: Point3,
    pub c: Point3,
    pub na: Point3,
    pub nb: Point3,
    pub nc: Point3,
    pub material: Arc<dyn Material>,
}

impl Triangle {
    pub fn new_auto_normal(
        a: Point3,
        b: Point3,
        c: Point3,
        material: Arc<dyn Material>,
    ) -> Arc<Self> {
        let normal = (b - a).cross(c - a).unit_vector();

        Arc::new(Self {
            a,
            b,
            c,
            na: normal,
            nb: normal,
            nc: normal,
            material,
        })
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let e1 = self.b - self.a;
        let e2 = self.c - self.a;
        let x = ray.dir.cross(e2);
        let d = e1.dot(x);
        let eps = 1e-6;

        if d > -eps && d < eps {
            return false;
        }

        let f = 1.0 / d;
        let s = ray.orig - self.a;
        let y = s.cross(e1);
        let t = f * e2.dot(y);

        if t < t_min || t_max < t {
            return false;
        }

        let u = f * s.dot(x);
        if u < 0.0 || u > 1.0 {
            return false;
        }

        let v = f * ray.dir.dot(y);
        if v < 0.0 || v > 1.0 || u + v > 1.0 {
            return false;
        }

        let w = 1.0 - u - v;

        record.t = t;
        record.p = ray.at(record.t);
        let outward_normal = (self.na * u + self.nb * v + self.nc * w).unit_vector();
        record.set_face_normal(ray, outward_normal);
        record.material = Some(self.material.clone());

        true
    }

    fn bounding_box(&self, _time0: f32, _time1: f32, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            self.a.min(self.b).min(self.c),
            self.a.max(self.b).max(self.c),
        );

        true
    }
}
