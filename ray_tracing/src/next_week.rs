use std::sync::Arc;

use ray_tracing::*;

fn main() {
    let world = random_scene();
    let camera = create_default_camera();

    render_world(world, camera)
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

    // light plane
    objects.push(zy_rect(
        -0.5,
        0.5,
        0.5,
        1.5,
        2.0,
        DiffuseLight::new(Color::new(1.0, 1.0, 1.0) * 4.0),
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
                } else if choose_mat < 1.2 {
                    // metal
                    // let albedo = Color::random_range(0.5, 1.0);
                    // let fuzz = random_f32_range(0.0, 0.5);
                    // Metal::new(albedo, fuzz)
                    DiffuseLight::new(Color::new(1.0, 1.0, 1.0))
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
