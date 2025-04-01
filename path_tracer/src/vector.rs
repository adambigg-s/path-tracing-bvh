#![allow(dead_code)]

use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::Float;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Vec3 {
    pub const TOLERANCE: f32 = 1e-4;

    pub fn build(x: Float, y: Float, z: Float) -> Self {
        Vec3 { x, y, z }
    }

    pub fn inner_product(&self, other: &Self) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross_product(&self, other: &Self) -> Self {
        Vec3::build(
            self.y * other.z - self.z * other.y,
            self.x * other.z - self.z * other.x,
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
        self.x.abs() < Self::TOLERANCE
            && self.y.abs() < Self::TOLERANCE
            && self.z.abs() < Self::TOLERANCE
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Vec3::build(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Vec3::build(self.x - other.x, self.y - other.y, self.z - other.z)
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
