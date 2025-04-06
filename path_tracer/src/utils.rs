use rand::Rng;

use crate::ray_hit::Ray;
use crate::vector::Vec3;

pub const INFIN: Float = f32::INFINITY;
pub const SMALL: Float = 1e-4;
pub const BACKGROUND_COLOR: Vec3 = Vec3::build(0.5, 0.7, 1.);

pub type Float = f32;
pub type Int = i32;

#[derive(Clone, Copy)]
pub struct Interval {
    pub min: Float,
    pub max: Float,
}

impl Interval {
    pub const fn build(min: Float, max: Float) -> Self {
        Interval { min, max }
    }

    pub fn new_real_valued() -> Interval {
        Interval::build(SMALL, INFIN)
    }

    pub fn contains(&self, value: Float) -> bool {
        self.min < value && value < self.max
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval::new_real_valued()
    }
}

pub fn sky_gradient(ray: &Ray) -> Vec3 {
    let unit = ray.direction.norm();
    let alpha = 0.5 * (unit.y + 1.);
    Vec3::build(1., 1., 1.) * (1. - alpha) + BACKGROUND_COLOR * alpha
}

pub fn random() -> Float {
    let mut rng = rand::rng();
    rng.random::<Float>()
}

pub fn packed_color(color: Vec3) -> u32 {
    let red = (color.x * 255.) as u32;
    let green = (color.y * 255.) as u32;
    let blue = (color.z * 255.) as u32;

    (red << 16) | (green << 8) | blue
}
