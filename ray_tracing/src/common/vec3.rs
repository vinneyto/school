use std::ops;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        *self *= 1.0 / t;
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self::Output {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3::new(self * other.x, self * other.y, self * other.z)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, t: f32) -> Self::Output {
        t * self
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, t: f32) -> Self::Output {
        (1.0 / t) * self
    }
}

impl Vec3 {
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }
}
