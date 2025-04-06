use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

use crate::utils::random;
use crate::utils::Float;
use crate::utils::SMALL;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Vec3 {
    pub const TOLERANCE: f32 = SMALL;

    pub const fn build(x: Float, y: Float, z: Float) -> Self {
        Vec3 { x, y, z }
    }

    pub fn zeros() -> Self {
        Vec3::build(0., 0., 0.)
    }

    pub fn random() -> Self {
        Vec3::build(random() - 0.5, random() - 0.5, random() - 0.5)
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let candidate = Vec3::random();
            let squared_magnitude = candidate.inner_product(&candidate);
            if Self::TOLERANCE < squared_magnitude {
                return candidate / squared_magnitude.sqrt();
            }
        }
    }

    pub fn inner_product(&self, other: &Self) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross_product(&self, other: &Self) -> Self {
        Vec3::build(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length(&self) -> Float {
        self.inner_product(self).sqrt()
    }

    pub fn norm(&self) -> Self {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        self.x.abs() < Self::TOLERANCE && self.y.abs() < Self::TOLERANCE && self.z.abs() < Self::TOLERANCE
    }

    pub fn mul_component(&self, other: &Self) -> Self {
        Vec3::build(self.x * other.x, self.y * other.y, self.z * other.z)
    }

    pub fn reflect_around(&self, axis: &Vec3) -> Self {
        *self - *axis * (2. * self.inner_product(axis))
    }

    pub fn refract_around(&self, axis: &Vec3, ratio: Float) -> Self {
        let cos = (-(*self)).inner_product(axis).min(1.);
        let perp = (*self + *axis * cos) * ratio;
        let par = *axis * -(1. - perp.inner_product(&perp)).abs().sqrt();
        perp + par
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Vec3::build(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Vec3::build(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3::build(-self.x, -self.y, -self.z)
    }
}

impl Mul<Float> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Float) -> Self::Output {
        Vec3::build(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<Float> for Vec3 {
    type Output = Self;
    fn div(self, rhs: Float) -> Self::Output {
        Vec3::build(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
