use std::time::Instant;

use image::{ImageBuffer, Rgba};
use rand::prelude::*;

use ray_tracing::*;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let samples_per_pixel = 50;
    let max_depth = 50;

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let camera = Camera::default();

    let now = Instant::now();

    println!("begin rendering...");

    let mut rnd = rand::thread_rng();

    let img = ImageBuffer::from_fn(image_width, image_height, |x, y| {
        let mut pixel_color = Color::default();

        for _ in 0..samples_per_pixel {
            let u = (x as f32 + rnd.gen::<f32>()) / (image_width - 1) as f32;
            let vv = (y as f32 + rnd.gen::<f32>()) / (image_height - 1) as f32;
            let v = 1.0 - vv;
            let ray = camera.get_ray(u, v);
            pixel_color += ray_color(&ray, &world, max_depth);
        }

        Rgba(to_rgba(&pixel_color, samples_per_pixel))
    });

    println!("rendered for {} ms", now.elapsed().as_millis());

    img.save("one_weekend.bmp").unwrap();
}

fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color {
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(ray, 0.001, f32::MAX, &mut rec) {
        let target = rec.p + rec.normal + Point3::random_in_unit_sphere().unit_vector();
        let next_ray = Ray::new(rec.p, target - rec.p);
        return 0.5 * ray_color(&next_ray, world, depth - 1);
    }

    let unit_direction = ray.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}
