use std::time::Instant;

use image::{ImageBuffer, Rgb};
use rand::prelude::*;
use rayon::prelude::*;

use ray_tracing::*;

fn main() {
    // Image

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
    let samples_per_pixel = 1000;
    #[cfg(feature = "precise")]
    let max_depth = 50;

    let aspect_ratio = 16.0 / 9.0;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    // World
    let (world, materials) = random_scene();

    // Camera

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let v_up = Point3::new(0.0, 0.1, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

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

    println!(
        "rendered for {} s",
        now.elapsed().as_millis() as f32 / 1000.0
    );

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

fn random_scene() -> (HittableList, MaterialArena) {
    let mut world = HittableList::default();
    let mut materials = MaterialArena::new();

    let ground_material_handle = materials.insert(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material_handle,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f32();

            let center = Point3::new(
                a as f32 + 0.9 * random_f32(),
                0.2,
                b as f32 + 0.9 * random_f32(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material_handle = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    materials.insert(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_f32_range(0.0, 0.5);
                    materials.insert(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    materials.insert(Dielectric::new(1.5))
                };
                world.add(Sphere::new(center, 0.2, material_handle));
            }
        }
    }

    let m1 = materials.insert(Dielectric::new(1.5));
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, m1));

    let m2 = materials.insert(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, m2));

    let m3 = materials.insert(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, m3));

    (world, materials)
}
