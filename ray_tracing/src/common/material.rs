use super::hittable::*;
use super::ray::*;
use super::vec3::*;

pub trait Material {
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
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn new_box(albedo: Color) -> Box<Self> {
        Box::new(Self::new(albedo))
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
        let mut scatter_direction = rec.normal + Point3::random_in_unit_sphere().unit_vector();

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
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn new_box(albedo: Color) -> Box<Self> {
        Box::new(Self::new(albedo))
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
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;

        scattered.dir.dot(rec.normal) > 0.0
    }
}
