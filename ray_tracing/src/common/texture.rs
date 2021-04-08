use std::sync::Arc;

use image::{open as open_image, ImageBuffer, Rgb};

use super::helpers::*;
use super::vec2::*;
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

pub enum TextureFiltering {
    Linear,
    Nearest,
}

pub enum TextureFlip {
    AsIs,
    FlipY,
}

pub struct ImageTexture {
    pub image: ImageBuffer<Rgb<u8>, Vec<u8>>,
    pub filtering: TextureFiltering,
    pub flip: TextureFlip,
    pub repeating: Vec2,
}

impl ImageTexture {
    pub fn new(
        path: &str,
        filtering: TextureFiltering,
        flip: TextureFlip,
        repeating: Vec2,
    ) -> Arc<Self> {
        let image = open_image(path).unwrap();
        let image = image.as_rgb8().unwrap().clone();

        Arc::new(ImageTexture {
            image,
            filtering,
            flip,
            repeating,
        })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _point: &Point3) -> Color {
        let w = self.image.width() as f32;
        let h = self.image.height() as f32;

        let mut u = (u * self.repeating.x) % 1.0;
        let mut v = (v * self.repeating.y) % 1.0;

        if u < 0.0 {
            u = u + 1.0;
        }

        if v < 0.0 {
            v = v + 1.0;
        }

        v = match self.flip {
            TextureFlip::AsIs => v,
            TextureFlip::FlipY => 1.0 - v,
        };

        let px = u * (w - 1.0);
        let py = v * (h - 1.0);

        match self.filtering {
            TextureFiltering::Nearest => {
                from_rgb(self.image.get_pixel(px.round() as u32, py.round() as u32))
            }
            TextureFiltering::Linear => {
                let px_from = px.floor();
                let py_from = py.floor();

                let px_to = px.ceil();
                let py_to = py.ceil();

                let c00 = from_rgb(self.image.get_pixel(px_from as u32, py_from as u32));
                let c10 = from_rgb(self.image.get_pixel(px_to as u32, py_from as u32));
                let c01 = from_rgb(self.image.get_pixel(px_from as u32, py_to as u32));
                let c11 = from_rgb(self.image.get_pixel(px_to as u32, py_to as u32));

                let a00 = (px_to - px) * (py_to - py);
                let a10 = (px - px_from) * (py_to - py);
                let a01 = (px_to - px) * (py - py_from);
                let a11 = (px - px_from) * (py - py_from);

                c00 * a00 + c10 * a10 + c01 * a01 + c11 * a11
            }
        }
    }
}
