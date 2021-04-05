use std::sync::Arc;

use super::vec3::*;

pub trait Texture: Sync + Send {
    fn value(&self, u: f32, v: f32, point: &Point3) -> Color;
}

pub struct SolidColor {
    pub color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Arc<Self> {
        Arc::new(SolidColor { color })
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _point: &Point3) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    pub even: Arc<dyn Texture>,
    pub odd: Arc<dyn Texture>,
    pub size: f32,
}

impl CheckerTexture {
    pub fn new(even: Arc<dyn Texture>, odd: Arc<dyn Texture>, size: f32) -> Arc<Self> {
        Arc::new(CheckerTexture { even, odd, size })
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, point: &Point3) -> Color {
        let sines = (self.size * u).sin() * (self.size * v).sin();

        if sines < 0.0 {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)
        }
    }
}

pub struct DebugUVTexture;

impl DebugUVTexture {
    pub fn new() -> Arc<Self> {
        Arc::new(DebugUVTexture)
    }
}

impl Texture for DebugUVTexture {
    fn value(&self, u: f32, v: f32, _point: &Point3) -> Color {
        Color::new(u, v, 0.0)
    }
}
