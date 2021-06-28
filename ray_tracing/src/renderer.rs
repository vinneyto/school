use std::env;
use std::time::Instant;

use image::{ImageBuffer, Rgb};
use rand::prelude::*;
use rayon::prelude::*;

use crate::common::*;

pub fn render_world<T: Hittable>(world: T, camera: Camera) {
    // fast
    #[cfg(not(feature = "precise"))]
    let image_width = 800;
    #[cfg(not(feature = "precise"))]
    let samples_per_pixel = 30;
    #[cfg(not(feature = "precise"))]
    let max_depth = 30;

    // precise
    #[cfg(feature = "precise")]
    let image_width = 1920;
    #[cfg(feature = "precise")]
    let samples_per_pixel = 2000;
    #[cfg(feature = "precise")]
    let max_depth = 100;

    let aspect_ratio = 16.0 / 9.0;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    let now = Instant::now();
    let background = Color::new(0.0, 0.0, 0.0);

    println!("begin rendering...");

    let pixels = (0..image_height * image_width)
        .into_par_iter()
        .map(|i| {
            let x = i % image_width;
            let y = (i - x) / image_width;
            let mut rnd = rand::thread_rng();
            let mut pixel_color = Color::default();

            for _ in 0..samples_per_pixel {
                let u = (x as f32 + rnd.gen::<f32>()) / (image_width - 1) as f32;
                let vv = (y as f32 + rnd.gen::<f32>()) / (image_height - 1) as f32;
                let v = 1.0 - vv;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &background, &world, max_depth);
            }

            to_rgb(&pixel_color, samples_per_pixel)
        })
        .flatten()
        .collect::<Vec<u8>>();

    let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_vec(image_width, image_height, pixels).unwrap();

    println!(
        "rendered for {} s",
        now.elapsed().as_millis() as f32 / 1000.0
    );

    let p = env::current_exe().unwrap();
    let name = p.file_name().unwrap();
    let s_name = name.to_str().unwrap();

    #[cfg(not(feature = "precise"))]
    let path = format!("{}.bmp", s_name);

    #[cfg(feature = "precise")]
    let path = format!("{}_precise.bmp", s_name);

    println!("saving {}", path);

    img.save(path).unwrap();
}

pub fn ray_color<T: Hittable>(ray: &Ray, background: &Color, world: &T, depth: i32) -> Color {
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(ray, 0.001, f32::MAX, &mut rec) && rec.material.is_some() {
        if let Some(override_color) = rec.override_color {
            return override_color;
        }

        let material = rec.material.clone().unwrap();
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        let emitted = material.emitted(rec.u, rec.v, &rec.p);
        if !material.scatter(&ray, &rec, &mut attenuation, &mut scattered) {
            return emitted;
        }

        return emitted + attenuation * ray_color(&scattered, &background, world, depth - 1);
    }

    *background
}

pub fn create_default_camera() -> Camera {
    let aspect_ratio = 16.0 / 9.0;
    let look_from = Point3::new(0.0, 4.0, 10.0);
    let look_at = Point3::new(0.0, 0.0, -3.0);
    let v_up = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    Camera::new(
        look_from,
        look_at,
        v_up,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    )
}
