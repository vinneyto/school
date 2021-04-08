use std::sync::Arc;
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
    let max_depth = 100;

    let aspect_ratio = 16.0 / 9.0;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    let world = random_scene();

    // Camera

    let look_from = Point3::new(0.0, 4.0, 10.0);
    let look_at = Point3::new(0.0, 0.0, -3.0);
    let v_up = Point3::new(0.0, 1.0, 0.0);
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
                pixel_color += ray_color(&ray, &world, max_depth);
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
    let path = "next_week.bmp";

    #[cfg(feature = "precise")]
    let path = "next_week_precise.bmp";

    img.save(path).unwrap();
}

fn ray_color<T: Hittable>(ray: &Ray, world: &T, depth: i32) -> Color {
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

        if material.scatter(&ray, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }

        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = ray.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn random_scene() -> BVHNode {
    let mut objects: Vec<Arc<dyn Hittable>> = vec![];
    let image_texture = ImageTexture::new(
        "./assets/bricks.jpeg",
        TextureFiltering::Linear,
        TextureFlip::FlipY,
        Vec2::new(0.5, 0.5),
    );

    let ground_texture = SolidColor::new(Color::new(0.5, 0.5, 0.5));
    let ground_material = Lambertian::new(ground_texture);

    objects.push(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f32();

            let mut center = Point3::new(
                a as f32 + 0.9 * random_f32(),
                0.2,
                b as f32 + 0.9 * random_f32(),
            );

            if center.length() < 1.0 {
                center.x *= 2.0;
                center.z *= 2.0;
            }

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Arc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    // let albedo = Color::random() * Color::random();
                    // let texture = SolidColor::new(albedo);
                    Lambertian::new(image_texture.clone())
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_f32_range(0.0, 0.5);
                    Metal::new(albedo, fuzz)
                } else {
                    // glass
                    Dielectric::new(1.5)
                };
                objects.push(Sphere::new(center, 0.2, material));
            }
        }
    }

    // objects.push(Sphere::new(
    //     Point3::new(0.0, 1.0, 0.0),
    //     1.0,
    //     Dielectric::new(1.5),
    // ));

    // objects.push(Sphere::new(
    //     Point3::new(-4.0, 1.0, 0.0),
    //     1.0,
    //     Lambertian::new(Color::new(0.4, 0.2, 0.1)),
    // ));

    // objects.push(Sphere::new(
    //     Point3::new(4.0, 1.0, 0.0),
    //     1.0,
    //     Metal::new(Color::new(0.7, 0.6, 0.5), 0.0),
    // ));

    let cup_color1 = SolidColor::new(Color::new(0.3, 0.8, 0.6));
    let cup_color2 = SolidColor::new(Color::new(0.9, 0.9, 0.9));

    let cup_texture = CheckerTexture::new(cup_color1, cup_color2, 200.0);
    let cup_material = Lambertian::new(cup_texture);
    // let cup_material = Dielectric::new(1.5);

    objects.push(bake_monkey_mesh(cup_material));

    BVHNode::new(&objects, 0.0, f32::MAX)
}
