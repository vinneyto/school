use std::ops;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub type Point2 = Vec2;

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn from_array(array: &[f32], index: usize) -> Self {
        let x = array[index];
        let y = array[index + 1];

        Self { x, y }
    }
}

impl ops::Add for Vec2 {
    type Output = Self;

    fn add(self, other: Vec2) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl ops::Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Self::Output {
        Vec2::new(self * other.x, self * other.y)
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, t: f32) -> Self::Output {
        t * self
    }
}
