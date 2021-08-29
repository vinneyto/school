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
            let path = "rest_of_your_life_cornell.bmp";

            #[cfg(feature = "precise")]
            let path = "rest_of_your_life_cornell_precise.bmp";

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
        _ => panic!("unknown scene {}", scene_name),
    };
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

    let mut box1: Arc<dyn Hittable>;
    box1 = bake_box(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.6, 1.2, 0.6),
        white.clone(),
    );
    box1 = RotateY::new(box1, 15.0).arc();
    box1 = Translate::new(box1, Vec3::new(0.3, 0.0, 0.5)).arc();
    objects.push(box1);

    let mut box2: Arc<dyn Hittable>;
    box2 = bake_box(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.6, 0.6, 0.6),
        white.clone(),
    );
    box2 = RotateY::new(box2, -18.0).arc();
    box2 = Translate::new(box2, Vec3::new(1.1, 0.0, 0.9)).arc();
    objects.push(box2);

    BVHNode::new(&objects, 0.0, f32::MAX)
}
