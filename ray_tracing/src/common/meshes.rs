use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json;

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
