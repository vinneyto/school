use std::sync::Arc;

use super::attribute::*;
use super::bvh::*;
use super::material::*;
use super::triangle::*;
use super::vec2::*;
use super::vec3::*;

pub fn xy_rect(
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    material: Arc<dyn Material>,
) -> Arc<BVHNode> {
    let normal = Vec3::new(0.0, 0.0, 1.0);

    let t1 = Triangle::new(
        Attribute {
            a: Point3::new(x0, y0, k),
            b: Point3::new(x1, y1, k),
            c: Point3::new(x0, y1, k),
        },
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
    let normal = Vec3::new(1.0, 0.0, 0.0);

    let t1 = Triangle::new(
        Attribute {
            a: Point3::new(k, y0, z0),
            b: Point3::new(k, y1, z0),
            c: Point3::new(k, y1, z1),
        },
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
    let normal = Vec3::new(0.0, 1.0, 0.0);

    let t1 = Triangle::new(
        Attribute {
            a: Point3::new(x0, k, z0),
            b: Point3::new(x0, k, z1),
            c: Point3::new(x1, k, z1),
        },
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
