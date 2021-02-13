use image::{ImageBuffer, Rgba};

use ray_tracing::*;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 600;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let img = ImageBuffer::from_fn(image_width, image_height, |x, y| {
        let u = x as f32 / (image_width - 1) as f32;
        let mut v = y as f32 / (image_height - 1) as f32;
        v = 1.0 - v;
        let ray = Ray::new(
            origin,
            lower_left_corner + u * horizontal + v * vertical - origin,
        );
        let c = ray_color(&ray, &world);
        Rgba([
            (c.x * 255.999) as u8,
            (c.y * 255.999) as u8,
            (c.z * 255.999) as u8,
            0u8,
        ])
    });

    img.save("one_weekend.jpg").unwrap();
}

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(ray, 0.0, f32::MAX, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}
