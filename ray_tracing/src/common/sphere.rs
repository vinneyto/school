use std::sync::Arc;

use generational_arena::Index;

use super::hittable::*;
use super::ray::Ray;
use super::vec3::*;

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material_handle: Index,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material_handle: Index) -> Self {
        Sphere {
            center,
            radius,
            material_handle,
        }
    }

    pub fn new_arc(center: Point3, radius: f32, material_handle: Index) -> Arc<Self> {
        Arc::new(Self::new(center, radius, material_handle))
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let oc = ray.orig - self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        };
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        record.t = root;
        record.p = ray.at(record.t);
        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);
        record.material_handle = Some(self.material_handle);

        true
    }
}
