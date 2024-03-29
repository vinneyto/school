use std::sync::Arc;

use rand::prelude::*;

use super::helpers::*;
use super::hittable::*;
use super::ray::*;
use super::texture::*;
use super::vec3::*;

pub enum Side {
    Front,
    Back,
    Double,
}

pub trait Material: Sync + Send {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        alb: &mut Color,
        scattered: &mut Ray,
        pdf: &mut f32,
    ) -> bool;

    fn scattering_pdf(&self, _ray_in: &Ray, _hit_record: &HitRecord, _scattered: &mut Ray) -> f32 {
        0.0
    }

    fn emitted(&self, _rec: &HitRecord) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Arc<Self> {
        Arc::new(Self { albedo })
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _: &Ray,
        rec: &HitRecord,
        alb: &mut Color,
        scattered: &mut Ray,
        pdf: &mut f32,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction.unit_vector());
        *alb = self.albedo.value(rec.u, rec.v, &rec.p);
        *pdf = rec.normal.dot(scattered.dir) / std::f32::consts::PI;
        true
    }

    fn scattering_pdf(&self, _ray_in: &Ray, rec: &HitRecord, scattered: &mut Ray) -> f32 {
        let cosine = rec.normal.dot(scattered.dir.unit_vector());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / std::f32::consts::PI
        }
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
        _pdf: &mut f32,
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
        _pdf: &mut f32,
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

pub struct DiffuseLight {
    pub color: Color,
    pub side: Side,
}

impl DiffuseLight {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            side: Side::Double,
        }
    }

    pub fn set_side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    pub fn arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
        _pdf: &mut f32,
    ) -> bool {
        false
    }

    fn emitted(&self, rec: &HitRecord) -> Color {
        let should_emit = match self.side {
            Side::Front => rec.front_face,
            Side::Back => !rec.front_face,
            Side::Double => true,
        };

        if should_emit {
            self.color
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }
}

pub enum DebugTarget {
    Normal,
    Face,
}

pub struct DebugMaterial {
    target: DebugTarget,
}

impl DebugMaterial {
    pub fn new(target: DebugTarget) -> Self {
        Self { target }
    }

    pub fn arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}

impl Material for DebugMaterial {
    fn scatter(
        &self,
        _: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
        _pdf: &mut f32,
    ) -> bool {
        false
    }

    fn emitted(&self, rec: &HitRecord) -> Color {
        match self.target {
            DebugTarget::Normal => rec.normal,
            DebugTarget::Face => {
                if rec.front_face {
                    Vec3::new(1.0, 1.0, 1.0)
                } else {
                    Vec3::zero()
                }
            }
        }
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
}
