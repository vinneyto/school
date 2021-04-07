use image::{Pixel, Rgb};
use rand::prelude::*;

use super::vec3::*;

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

pub fn to_rgb(color: &Color, samples_per_pixel: u32) -> Vec<u8> {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / samples_per_pixel as f32;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    vec![
        (256.0 * clamp(r, 0.0, 0.999)) as u8,
        (256.0 * clamp(g, 0.0, 0.999)) as u8,
        (256.0 * clamp(b, 0.0, 0.999)) as u8,
    ]
}

pub fn from_rgb(pixel: &Rgb<u8>) -> Color {
    let channels = pixel.channels();

    let r = channels[0] as f32 / 255.0;
    let g = channels[1] as f32 / 255.0;
    let b = channels[2] as f32 / 255.0;
    Color::new(r, g, b)
}

pub fn random_f32() -> f32 {
    let mut rnd = rand::thread_rng();

    rnd.gen()
}

pub fn random_f32_range(from: f32, to: f32) -> f32 {
    let mut rnd = rand::thread_rng();

    rnd.gen_range(from..to)
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_disc() -> Vec3 {
    let mut rnd = rand::thread_rng();

    loop {
        let p = Vec3::new(rnd.gen_range(-1.0..1.0), rnd.gen_range(-1.0..1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
