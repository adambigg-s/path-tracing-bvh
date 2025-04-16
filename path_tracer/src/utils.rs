use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;

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
    const TOLERANCE: Float = SMALL;

    pub const fn build(min: Float, max: Float) -> Self {
        Interval { min, max }
    }

    pub fn near_zero() -> Self {
        Interval::build(-Self::TOLERANCE, Self::TOLERANCE)
    }

    pub fn new_real_valued() -> Self {
        Interval::build(Self::TOLERANCE, INFIN)
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
    let unit = ray.direction.normalized();
    let alpha = 0.5 * (unit.y + 1.);
    Vec3::build(1., 1., 1.) * (1. - alpha) + BACKGROUND_COLOR * alpha
}

pub fn random() -> Float {
    let mut rng = rand::rng();
    rng.random::<Float>()
}

pub fn packed_color(color: Vec3) -> u32 {
    let red = ((color.x * 255.) as u8) as u32;
    let green = ((color.y * 255.) as u8) as u32;
    let blue = ((color.z * 255.) as u8) as u32;

    (red << 16) | (green << 8) | blue
}

pub fn unpack_color(color: u32) -> Vec3 {
    let red = ((color >> 16) & 0xff) as f32 / 255.;
    let green = ((color >> 8) & 0xff) as f32 / 255.;
    let blue = ((color) & 0xff) as f32 / 255.;

    Vec3::build(red, green, blue)
}

pub fn write_pixel_gammcorr(writer: &mut BufWriter<&mut File>, color: Vec3) -> io::Result<()> {
    let red = (color.x.sqrt() * 255.999) as u8;
    let green = (color.y.sqrt() * 255.999) as u8;
    let blue = (color.z.sqrt() * 255.999) as u8;
    write!(writer, "{} {} {} ", red, green, blue)?;

    Ok(())
}
