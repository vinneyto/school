use std::time::Instant;

use image::{ImageBuffer, Rgb};
use rand::prelude::*;
use rayon::prelude::*;

use ray_tracing::*;

fn main() {
    // Image

    // fast
    #[cfg(not(feature = "precise"))]
    let image_width = 400;
    #[cfg(not(feature = "precise"))]
    let samples_per_pixel = 10;
    #[cfg(not(feature = "precise"))]
    let max_depth = 10;

    // precise
    #[cfg(feature = "precise")]
    let image_width = 1920;
    #[cfg(feature = "precise")]
    let samples_per_pixel = 700;
    #[cfg(feature = "precise")]
    let max_depth = 200;

    let aspect_ratio = 16.0 / 9.0;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    // World
    let mut materials = MaterialArena::new();

    let material_ground_handle = materials.insert(Lambertian::new_arc(Color::new(0.8, 0.8, 0.0)));
    let material_center_handle = materials.insert(Lambertian::new_arc(Color::new(0.7, 0.3, 0.3)));
    let material_left_handle = materials.insert(Metal::new_arc(Color::new(0.8, 0.8, 0.8)));
    let material_right_handle = materials.insert(Metal::new_arc(Color::new(0.8, 0.6, 0.2)));

    let mut world = HittableList::default();
    world.add(Sphere::new_arc(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_ground_handle,
    ));
    world.add(Sphere::new_arc(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_center_handle,
    ));
    world.add(Sphere::new_arc(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left_handle,
    ));
    world.add(Sphere::new_arc(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right_handle,
    ));

    // Camera

    let camera = Camera::default();

    let now = Instant::now();

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
                pixel_color += ray_color(&ray, &world, &materials, max_depth);
            }

            to_rgb(&pixel_color, samples_per_pixel)
        })
        .flatten()
        .collect::<Vec<u8>>();

    let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_vec(image_width, image_height, pixels).unwrap();

    println!("rendered for {} ms", now.elapsed().as_millis());

    #[cfg(not(feature = "precise"))]
    let path = "one_weekend.bmp";

    #[cfg(feature = "precise")]
    let path = "one_weekend_precise.bmp";

    img.save(path).unwrap();
}

fn ray_color(ray: &Ray, world: &HittableList, materials: &MaterialArena, depth: i32) -> Color {
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(ray, 0.001, f32::MAX, &mut rec) && rec.material_handle.is_some() {
        let material = materials.get(rec.material_handle.unwrap()).unwrap();
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();

        if material.scatter(&ray, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, materials, depth - 1);
        }

        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = ray.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}
