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

pub fn random() -> Vec3 {
    let mut rnd = rand::thread_rng();

    Vec3::new(rnd.gen(), rnd.gen(), rnd.gen())
}

pub fn random_range(from: f32, to: f32) -> Vec3 {
    let mut rnd = rand::thread_rng();

    Vec3::new(
        rnd.gen_range(from..to),
        rnd.gen_range(from..to),
        rnd.gen_range(from..to),
    )
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_range(-1.0, 1.0);
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
