use super::ray::*;
use super::vec3::*;

const DEG_TO_RAD: f32 = 0.017453292519943295769236907684886;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: Point3, look_at: Point3, v_up: Vec3, vfov: f32, aspect: f32) -> Self {
        let theta = DEG_TO_RAD * vfov;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = v_up.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
