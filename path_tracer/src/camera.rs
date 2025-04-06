use std::f32::consts::PI;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;

use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

use crate::ray_hit::Ray;
use crate::scene::Scene;
use crate::utils::packed_color;
use crate::utils::Float;
use crate::utils::Int;
use crate::vector::Vec3;
use crate::Buffer;

#[allow(dead_code)]
#[derive(Default)]
pub struct Camera {
    pub height: Int,
    pub width: Int,
    pub aspect_ratio: Float,
    pub fov: Float,
    pub focal_length: Float,

    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub world_up: Vec3,

    pub yaw: Float,
    pub pitch: Float,

    pub pixel_top_left: Vec3,
    pub pixel_du: Vec3,
    pub pixel_dv: Vec3,

    pub samples: Int,
    pub final_samples: Int,
    pub max_recursive_depth: Int,
}

impl Camera {
    pub fn build_default(width: Int, height: Int, samples: Int, final_samples: Int) -> Self {
        let aspect_ratio = width as Float / height as Float;
        let world_up = Vec3::build(0., 1., 0.);
        let look_at = Vec3::build(0., 0., -1.);

        let position = Vec3::build(0., 0., 1.);
        let front = look_at.norm();
        let up = world_up;

        let fov = 45.;
        let focal_length = 1.;
        let yaw = PI / 2.;

        let mut camera = Camera {
            height,
            width,
            aspect_ratio,
            fov,
            focal_length,

            position,
            front,
            up,
            world_up,

            yaw,

            samples,
            final_samples,
            max_recursive_depth: 30,
            ..Default::default()
        };
        camera.set_viewport();

        camera
    }

    pub fn set_viewport(&mut self) {
        self.update_vectors();
        let height_modifier = (self.fov.to_radians() * 0.5).tan();
        let viewport_height = 2. * height_modifier * self.focal_length;
        let viewport_width = viewport_height * self.aspect_ratio;

        let viewport_u = self.right * viewport_width;
        let viewport_v = -self.up * viewport_height;

        self.pixel_du = viewport_u / self.width as Float;
        self.pixel_dv = viewport_v / self.height as Float;

        let viewport_center = self.position - self.front * self.focal_length;
        let viewport_upper_left = viewport_center - (viewport_u + viewport_v) * 0.5;
        self.pixel_top_left = viewport_upper_left + (self.pixel_du + self.pixel_dv) * 0.5;
    }

    pub fn update_vectors(&mut self) {
        let front = Vec3::build(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        );
        self.front = front.norm();
        self.right = self.world_up.cross_product(&self.front).norm();
        self.up = self.front.cross_product(&self.right).norm();
    }

    pub fn render_to_file(&self, file: &mut File, scene: &Scene) -> io::Result<()> {
        let printerval = self.height / 100;

        let mut writer = BufWriter::with_capacity((self.width * self.height * 12) as usize, file);
        writeln!(writer, "P3\n{} {}\n255", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                let mut pixel_color = Vec3::zeros();
                (0..self.final_samples).for_each(|_| {
                    let ray = self.get_ray(x, y);
                    pixel_color += scene.get_color(&ray, self.max_recursive_depth);
                });
                self.write_pixel_gammcorr(&mut writer, pixel_color / self.final_samples as Float)?;
            }
            writeln!(writer)?;
            if y % printerval == 0 {
                println!("progress: {:.1}%", y as f32 / self.height as Float * 100.);
            }
        }
        writer.flush()?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn render_to_buffer(&self, buffer: &mut Buffer, scene: &Scene) {
        for y in 0..buffer.height {
            for x in 0..buffer.width {
                let mut pixel_color = Vec3::zeros();
                (0..self.samples).for_each(|_| {
                    let ray = self.get_ray(x as i32, y as i32);
                    pixel_color += scene.get_color(&ray, self.max_recursive_depth);
                });
                buffer.write_pixel(x, y, pixel_color / self.samples as Float);
            }
        }
    }

    pub fn render_to_buffer_par(&self, buffer: &mut Buffer, scene: &Scene) {
        buffer.pixels.par_iter_mut().enumerate().for_each(|(idx, pixel)| {
            let x = (idx % buffer.width) as i32;
            let y = (idx / buffer.width) as i32;

            let mut pixel_color = Vec3::zeros();
            (0..self.samples).for_each(|_| {
                let ray = self.get_ray(x, y);
                pixel_color += scene.get_color(&ray, self.max_recursive_depth);
            });

            *pixel = packed_color(pixel_color / self.samples as Float);
        });
    }

    pub fn get_ray(&self, x: Int, y: Int) -> Ray {
        let offset = Vec3::random();
        let pixel_sample = self.pixel_top_left
            + (self.pixel_du * (offset.x + x as Float))
            + (self.pixel_dv * (offset.y + y as Float));
        let ray_direction = pixel_sample - self.position;

        Ray::build(self.position, ray_direction)
    }

    pub fn write_pixel_gammcorr(&self, writer: &mut BufWriter<&mut File>, color: Vec3) -> io::Result<()> {
        let red = (color.x.sqrt() * 255.) as u8;
        let green = (color.y.sqrt() * 255.) as u8;
        let blue = (color.z.sqrt() * 255.) as u8;
        write!(writer, "{} {} {} ", red, green, blue)?;

        Ok(())
    }
}
