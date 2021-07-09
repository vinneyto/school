use std::env;
use std::sync::Arc;

use ray_tracing::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let scene_name = if args.len() > 1 {
        args[1].clone()
    } else {
        String::from("default")
    };

    match scene_name.as_str() {
        "cornell" => {
            //fast
            #[cfg(not(feature = "precise"))]
            let image_width = 400;
            #[cfg(not(feature = "precise"))]
            let samples_per_pixel = 200;
            #[cfg(not(feature = "precise"))]
            let max_depth = 30;

            // precise
            #[cfg(feature = "precise")]
            let image_width = 1920;
            #[cfg(feature = "precise")]
            let samples_per_pixel = 10000;
            #[cfg(feature = "precise")]
            let max_depth = 100;
            let aspect_ratio = 1.0;

            #[cfg(not(feature = "precise"))]
            let path = "next_week_cornell.bmp";

            #[cfg(feature = "precise")]
            let path = "next_week_cornell_precise.bmp";

            let look_from = Point3::new(1.0, 1.0, 8.0);
            let look_at = Point3::new(1.0, 1.0, -3.0);
            let v_up = Point3::new(0.0, 1.0, 0.0);
            let dist_to_focus = 10.0;
            let aperture = 0.0;
            let background = Color::new(0.0, 0.0, 0.0);

            let world = cornell_box();

            println!("rendering -> cornell");

            let params = CPURenderingParams {
                world,
                camera: Camera::new(
                    look_from,
                    look_at,
                    v_up,
                    20.0,
                    aspect_ratio,
                    aperture,
                    dist_to_focus,
                ),
                image_width,
                samples_per_pixel,
                max_depth,
                aspect_ratio,
                background,
                path: String::from(path),
            };

            render_world_cpu(params);
        }
        "default" => {
            //fast
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

            #[cfg(not(feature = "precise"))]
            let path = "next_week.bmp";

            #[cfg(feature = "precise")]
            let path = "next_week_precise.bmp";

            let look_from = Point3::new(0.0, 4.0, 10.0);
            let look_at = Point3::new(0.0, 0.0, -3.0);
            let v_up = Point3::new(0.0, 1.0, 0.0);
            let dist_to_focus = 10.0;
            let aperture = 0.1;
            let background = Color::new(0.0, 0.0, 0.0);

            println!("rendering -> default");

            let params = CPURenderingParams {
                world: random_scene(),
                camera: Camera::new(
                    look_from,
                    look_at,
                    v_up,
                    20.0,
                    aspect_ratio,
                    aperture,
                    dist_to_focus,
                ),
                image_width,
                samples_per_pixel,
                max_depth,
                aspect_ratio,
                background,
                path: String::from(path),
            };

            render_world_cpu(params);
        }
        _ => panic!("unknown scene {}", scene_name),
    };
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
    objects.push(yz_rect(
        0.5,
        1.5,
        -0.5,
        0.5,
        2.0,
        DiffuseLight::new(Color::new(1.0, 1.0, 1.0) * 4.0).arc(),
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
                    DiffuseLight::new(Color::new(1.0, 1.0, 1.0)).arc()
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

fn cornell_box() -> BVHNode {
    let red = Lambertian::new(SolidColor::new(Color::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(SolidColor::new(Color::new(0.12, 0.45, 0.15)));
    let light = DiffuseLight::new(Color::new(15.0, 15.0, 15.0))
        .set_side(Side::Back)
        .arc();

    let mut objects: Vec<Arc<dyn Hittable>> = vec![];

    objects.push(yz_rect(0.0, 2.0, 0.0, 2.0, 0.0, green));
    objects.push(yz_rect(2.0, 0.0, 0.0, 2.0, 2.0, red));
    objects.push(xz_rect(0.8, 1.2, 0.8, 1.2, 1.99, light.clone()));
    objects.push(xz_rect(0.0, 2.0, 0.0, 2.0, 0.0, white.clone()));
    objects.push(xz_rect(2.0, 0.0, 0.0, 2.0, 2.0, white.clone()));
    objects.push(xy_rect(0.0, 2.0, 0.0, 2.0, 0.0, white.clone()));

    // let dm = DebugMaterial::new(DebugTarget::Face).arc();
    // objects.push(yz_rect(0.0, 2.0, 0.0, 2.0, 0.0, dm.clone()));
    // objects.push(yz_rect(2.0, 0.0, 0.0, 2.0, 2.0, dm.clone()));
    // objects.push(xz_rect(0.0, 2.0, 0.0, 2.0, 0.0, dm.clone()));
    // objects.push(xz_rect(2.0, 0.0, 0.0, 2.0, 2.0, dm.clone()));
    // objects.push(xy_rect(0.0, 2.0, 0.0, 2.0, 0.0, dm.clone()));

    objects.push(bake_box(
        Vec3::new(0.0 + 0.3, 0.0, 0.0 + 0.3),
        Vec3::new(0.6 + 0.3, 1.2, 0.6 + 0.3),
        white.clone(),
    ));

    objects.push(bake_box(
        Vec3::new(0.0 + 1.0, 0.0, 0.0 + 1.0),
        Vec3::new(0.6 + 1.0, 0.6, 0.6 + 1.0),
        white.clone(),
    ));

    BVHNode::new(&objects, 0.0, f32::MAX)
}
