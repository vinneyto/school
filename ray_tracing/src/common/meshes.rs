use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json;

use super::attribute::*;
use super::bvh::*;
use super::hittable::*;
use super::material::*;
use super::triangle::*;
use super::vec2::*;
use super::vec3::*;

#[derive(Serialize, Deserialize, Debug)]
struct Mesh {
    position: Vec<f32>,
    normal: Vec<f32>,
    uv: Vec<f32>,
    index: Vec<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Donut {
    pub cup: Mesh,
    pub plate: Mesh,
}

pub fn bake_cup_mesh(material: Arc<dyn Material>) -> Arc<dyn Hittable> {
    let data = include_str!("../../assets/donut.json");

    let donut: Donut = serde_json::from_str(data).unwrap();

    let mut trinagles: Vec<Arc<dyn Hittable>> = vec![];

    let shift = Point3::new(0.0, 0.9, 0.0);

    for i in (0..donut.cup.index.len()).step_by(3) {
        let ai = donut.cup.index[i] as usize;
        let bi = donut.cup.index[i + 1] as usize;
        let ci = donut.cup.index[i + 2] as usize;

        let a = Point3::from_array(&donut.cup.position, ai * 3) * 20.0 + shift;
        let b = Point3::from_array(&donut.cup.position, bi * 3) * 20.0 + shift;
        let c = Point3::from_array(&donut.cup.position, ci * 3) * 20.0 + shift;

        let na = Point3::from_array(&donut.cup.normal, ai * 3);
        let nb = Point3::from_array(&donut.cup.normal, bi * 3);
        let nc = Point3::from_array(&donut.cup.normal, ci * 3);

        let ta = Point2::from_array(&donut.cup.uv, ai * 2);
        let tb = Point2::from_array(&donut.cup.uv, bi * 2);
        let tc = Point2::from_array(&donut.cup.uv, ci * 2);

        let position = Attribute::new(a, b, c);
        let normal = Attribute::new(na, nb, nc);
        let uv = Attribute::new(ta, tb, tc);

        let triangle = Triangle::new(position, normal, uv, material.clone());

        trinagles.push(triangle);
    }

    Arc::new(BVHNode::new(&trinagles, 0.0, f32::MAX))
}

pub fn bake_monkey_mesh(material: Arc<dyn Material>) -> Arc<dyn Hittable> {
    let data = include_str!("../../assets/monkey.json");

    let monkey: Mesh = serde_json::from_str(data).unwrap();

    let mut trinagles: Vec<Arc<dyn Hittable>> = vec![];

    let shift = Point3::new(0.0, 1.0, 0.0);

    for i in (0..monkey.index.len()).step_by(3) {
        let ai = monkey.index[i] as usize;
        let bi = monkey.index[i + 1] as usize;
        let ci = monkey.index[i + 2] as usize;

        let a = Point3::from_array(&monkey.position, ai * 3) + shift;
        let b = Point3::from_array(&monkey.position, bi * 3) + shift;
        let c = Point3::from_array(&monkey.position, ci * 3) + shift;

        let na = Point3::from_array(&monkey.normal, ai * 3);
        let nb = Point3::from_array(&monkey.normal, bi * 3);
        let nc = Point3::from_array(&monkey.normal, ci * 3);

        let ta = Point2::from_array(&monkey.uv, ai * 2);
        let tb = Point2::from_array(&monkey.uv, bi * 2);
        let tc = Point2::from_array(&monkey.uv, ci * 2);

        let position = Attribute::new(a, b, c);
        let normal = Attribute::new(na, nb, nc);
        let uv = Attribute::new(ta, tb, tc);

        let triangle = Triangle::new(position, normal, uv, material.clone());

        trinagles.push(triangle);
    }

    Arc::new(BVHNode::new(&trinagles, 0.0, f32::MAX))
}

pub fn xy_rect(
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    material: Arc<dyn Material>,
) -> Arc<BVHNode> {
    let a = Point3::new(x0, y0, k);
    let b = Point3::new(x1, y1, k);
    let c = Point3::new(x0, y1, k);
    let normal = (b - a).cross(c - a).unit_vector();

    let t1 = Triangle::new(
        Attribute { a, b, c },
        Attribute {
            a: normal,
            b: normal,
            c: normal,
        },
        Attribute {
            a: Vec2::new(0.0, 0.0),
            b: Vec2::new(1.0, 1.0),
            c: Vec2::new(0.0, 1.0),
        },
        material.clone(),
    );

    let t2 = Triangle::new(
        Attribute {
            a: Point3::new(x0, y0, k),
            b: Point3::new(x1, y0, k),
            c: Point3::new(x1, y1, k),
        },
        Attribute {
            a: normal,
            b: normal,
            c: normal,
        },
        Attribute {
            a: Vec2::new(0.0, 0.0),
            b: Vec2::new(1.0, 0.0),
            c: Vec2::new(1.0, 1.0),
        },
        material.clone(),
    );

    Arc::new(BVHNode::new(&[t1, t2], 0.0, f32::MAX))
}

pub fn yz_rect(
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: Arc<dyn Material>,
) -> Arc<BVHNode> {
    let a = Point3::new(k, y0, z0);
    let b = Point3::new(k, y1, z0);
    let c = Point3::new(k, y1, z1);
    let normal = (b - a).cross(c - a).unit_vector();

    let t1 = Triangle::new(
        Attribute { a, b, c },
        Attribute {
            a: normal,
            b: normal,
            c: normal,
        },
        Attribute {
            a: Vec2::new(0.0, 0.0),
            b: Vec2::new(0.0, 1.0),
            c: Vec2::new(1.0, 1.0),
        },
        material.clone(),
    );

    let t2 = Triangle::new(
        Attribute {
            a: Point3::new(k, y0, z0),
            b: Point3::new(k, y1, z1),
            c: Point3::new(k, y0, z1),
        },
        Attribute {
            a: normal,
            b: normal,
            c: normal,
        },
        Attribute {
            a: Vec2::new(0.0, 0.0),
            b: Vec2::new(1.0, 1.0),
            c: Vec2::new(1.0, 0.0),
        },
        material.clone(),
    );

    Arc::new(BVHNode::new(&[t1, t2], 0.0, f32::MAX))
}

pub fn xz_rect(
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: Arc<dyn Material>,
) -> Arc<BVHNode> {
    let a = Point3::new(x0, k, z0);
    let b = Point3::new(x0, k, z1);
    let c = Point3::new(x1, k, z1);
    let normal = (b - a).cross(c - a).unit_vector();

    let t1 = Triangle::new(
        Attribute { a, b, c },
        Attribute {
            a: normal,
            b: normal,
            c: normal,
        },
        Attribute {
            a: Vec2::new(0.0, 0.0),
            b: Vec2::new(1.0, 1.0),
            c: Vec2::new(0.0, 1.0),
        },
        material.clone(),
    );

    let t2 = Triangle::new(
        Attribute {
            a: Point3::new(x0, k, z0),
            b: Point3::new(x1, k, z1),
            c: Point3::new(x1, k, z0),
        },
        Attribute {
            a: normal,
            b: normal,
            c: normal,
        },
        Attribute {
            a: Vec2::new(0.0, 0.0),
            b: Vec2::new(1.0, 0.0),
            c: Vec2::new(1.0, 1.0),
        },
        material.clone(),
    );

    Arc::new(BVHNode::new(&[t1, t2], 0.0, f32::MAX))
}

pub fn bake_box(p0: Vec3, p1: Vec3, material: Arc<dyn Material>) -> Arc<dyn Hittable> {
    let sides: Vec<Arc<dyn Hittable>> = vec![
        xy_rect(p0.x, p1.x, p0.y, p1.y, p1.z, material.clone()),
        xy_rect(p1.x, p0.x, p0.y, p1.y, p0.z, material.clone()),
        xz_rect(p0.x, p1.x, p0.z, p1.z, p1.y, material.clone()),
        xz_rect(p1.x, p0.x, p0.z, p1.z, p0.y, material.clone()),
        yz_rect(p0.y, p1.y, p0.z, p1.z, p1.x, material.clone()),
        yz_rect(p1.y, p0.y, p0.z, p1.z, p0.x, material.clone()),
    ];

    Arc::new(BVHNode::new(&sides, 0.0, f32::MAX))
}
