use std::sync::Arc;

use rand::prelude::*;

use super::helpers::*;
use super::hittable::*;
use super::ray::*;
use super::vec3::*;

pub trait Material: Sync + Send {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Arc<Self> {
        Arc::new(Self { albedo })
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_in_unit_sphere().unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Debug)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Arc<Self> {
        Arc::new(Self { albedo, fuzz })
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = ray_in.dir.unit_vector().reflect(rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * random_in_unit_sphere().unit_vector(),
        );
        *attenuation = self.albedo;

        scattered.dir.dot(rec.normal) > 0.0
    }
}

#[derive(Debug)]
pub struct Dielectric {
    pub index_of_refraction: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Arc<Self> {
        Arc::new(Self {
            index_of_refraction,
        })
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray_in.dir.unit_vector();

        let cos_theta = clamp((-unit_direction).dot(rec.normal), -1.0, 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction;

        if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen() {
            direction = unit_direction.reflect(rec.normal);
        } else {
            direction = unit_direction.refract(rec.normal, refraction_ratio);
        }

        *scattered = Ray::new(rec.p, direction);
        return true;
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
}
