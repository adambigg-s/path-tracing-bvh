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

#[allow(dead_code)]
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

    pub fn normalized(&self) -> Self {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        self.x.abs() < Self::TOLERANCE && self.y.abs() < Self::TOLERANCE && self.z.abs() < Self::TOLERANCE
    }

    pub fn mul_component(&self, other: &Self) -> Self {
        Vec3::build(self.x * other.x, self.y * other.y, self.z * other.z)
    }

    pub fn rotate_x_inplace(&mut self, angle: Float) {
        let Vec3 { x, y, z } = *self;
        let (sin, cos) = angle.sin_cos();
        self.x = x;
        self.y = y * cos + z * -sin;
        self.z = y * sin + z * cos;
    }

    pub fn rotate_y_inplace(&mut self, angle: Float) {
        let Vec3 { x, y, z } = *self;
        let (sin, cos) = angle.sin_cos();
        self.x = x * cos + z * sin;
        self.y = y;
        self.z = x * -sin + z * cos;
    }

    pub fn rotate_z_inplace(&mut self, angle: Float) {
        let Vec3 { x, y, z } = *self;
        let (sin, cos) = angle.sin_cos();
        self.x = x * cos + y * -sin;
        self.y = x * sin + y * cos;
        self.z = z;
    }

    pub fn rotate_x(&self, angle: Float) -> Self {
        let Vec3 { x, y, z } = *self;
        let (sin, cos) = angle.sin_cos();
        Vec3::build(x, y * cos + z * -sin, y * sin + z * cos)
    }

    pub fn rotate_y(&self, angle: Float) -> Self {
        let Vec3 { x, y, z } = *self;
        let (sin, cos) = angle.sin_cos();
        Vec3::build(x * cos + z * sin, y, x * -sin + z * cos)
    }

    pub fn rotate_z(&self, angle: Float) -> Self {
        let Vec3 { x, y, z } = *self;
        let (sin, cos) = angle.sin_cos();
        Vec3::build(x * cos + y * -sin, x * sin + y * cos, z)
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

    pub fn random_on_hemisphere(norm: &Vec3) -> Self {
        let candidate = Self::random_unit_vector();
        if candidate.inner_product(norm) > 0. {
            candidate
        }
        else {
            -candidate
        }
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
