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
    pub na: Vec3,
    pub nb: Vec3,
    pub nc: Vec3,
    pub face_normal: Vec3,
    pub material: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(
        a: Point3,
        b: Point3,
        c: Point3,
        na: Vec3,
        nb: Vec3,
        nc: Vec3,
        material: Arc<dyn Material>,
    ) -> Arc<Self> {
        let face_normal = (b - a).cross(c - a).unit_vector();

        Arc::new(Self {
            a,
            b,
            c,
            na,
            nb,
            nc,
            face_normal,
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
        let outward_normal = (self.na * w + self.nb * u + self.nc * v).unit_vector();
        let front_face = ray.dir.dot(self.face_normal) < 0.0;
        record.set_front_face_and_normal(front_face, outward_normal);
        record.material = Some(self.material.clone());
        record.override_color = None;

        // if w < 0.01 || u < 0.01 || v < 0.05 {
        //         // record.override_color = Some(Color::new(0.0, 0.0, 0.0));
        //     record.override_color = Some(record.normal);
        // }

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
