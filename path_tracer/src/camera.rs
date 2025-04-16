use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSliceMut;

use crate::ray_hit::Ray;
use crate::scene::Scene;
use crate::utils::packed_color;
use crate::utils::unpack_color;
use crate::utils::write_pixel_gammcorr;
use crate::utils::Float;
use crate::utils::Int;
use crate::vector::Vec3;
use crate::Buffer;

#[derive(Default, Debug)]
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

    pub move_speed: Float,
    pub rotation_speed: Float,

    pub pixel_top_left: Vec3,
    pub pixel_du: Vec3,
    pub pixel_dv: Vec3,

    pub samples: Int,
    pub max_recursive_depth: Int,
    pub denoise_iters: Int,
}

impl Camera {
    pub fn build_params(&mut self) {
        self.aspect_ratio = self.width as Float / self.height as Float;
        self.world_up = Vec3::build(0., 1., 0.);
        self.front = Vec3::build(0., 0., 1.).normalized();
        self.set_viewport();
    }

    pub fn set_viewport(&mut self) {
        self.update_vectors();
        self.aspect_ratio = self.width as Float / self.height as Float;
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
        self.front = front.normalized();
        self.right = self.world_up.cross_product(&self.front).normalized();
        self.up = self.front.cross_product(&self.right).normalized();
    }

    pub fn get_ray(&self, x: Int, y: Int) -> Ray {
        let offset = Vec3::random();
        let pixel_sample = self.pixel_top_left
            + (self.pixel_du * (offset.x + x as Float))
            + (self.pixel_dv * (offset.y + y as Float));
        let ray_direction = pixel_sample - self.position;

        Ray::build(self.position, ray_direction)
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

    pub fn render_to_file_par(&mut self, file: &mut File, scene: &Scene) -> io::Result<()> {
        let total_pixels = (self.width * self.height) as usize;
        let progress_counter = Arc::new(AtomicUsize::new(0));
        let progress_clone = Arc::clone(&progress_counter);
        let progress_thread = thread::spawn(move || loop {
            let done = progress_clone.load(Ordering::Relaxed);
            let percent = done as Float / total_pixels as Float * 100.;
            println!("progress: {:.2}%", percent);

            if done >= total_pixels {
                break;
            }

            thread::sleep(Duration::from_millis(1000));
        });

        let mut buffer = Buffer::build(self.height as usize, self.width as usize);
        let chunk_size = buffer.width;
        buffer.pixels.par_chunks_mut(chunk_size).enumerate().for_each(|(chunk_idx, chunk)| {
            let start_idx = chunk_idx * chunk_size;
            let chunk_length = chunk.len();
            for (i, pixel) in chunk.iter_mut().enumerate() {
                let idx = start_idx + i;
                let x = (idx % buffer.width) as i32;
                let y = (idx / buffer.width) as i32;
                let mut pixel_color = Vec3::zeros();
                (0..self.samples).for_each(|_| {
                    let ray = self.get_ray(x, y);
                    pixel_color += scene.get_color(&ray, self.max_recursive_depth);
                });

                *pixel = packed_color(pixel_color / self.samples as Float);
            }
            progress_counter.fetch_add(chunk_length, Ordering::Relaxed);
        });

        progress_thread.join().unwrap();

        buffer.bilateral_denoise(self.denoise_iters);

        let mut writer = BufWriter::with_capacity((self.width * self.height * 12) as usize, file);
        writeln!(writer, "P3\n{} {}\n255", self.width, self.height)?;
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                let packed_color = buffer.pixels[y * buffer.width + x];
                let pixel_color = unpack_color(packed_color);
                write_pixel_gammcorr(&mut writer, pixel_color)?;
            }
        }

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

    #[allow(dead_code)]
    pub fn render_to_file(&self, file: &mut File, scene: &Scene) -> io::Result<()> {
        let printerval = self.height / 100;

        let mut writer = BufWriter::with_capacity((self.width * self.height * 12) as usize, file);
        writeln!(writer, "P3\n{} {}\n255", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                let mut pixel_color = Vec3::zeros();
                (0..self.samples).for_each(|_| {
                    let ray = self.get_ray(x, y);
                    pixel_color += scene.get_color(&ray, self.max_recursive_depth);
                });
                write_pixel_gammcorr(&mut writer, pixel_color / self.samples as Float)?;
            }
            writeln!(writer)?;
            if y % printerval == 0 {
                println!("progress: {:.1}%", y as f32 / self.height as Float * 100.);
            }
        }
        writer.flush()?;

        Ok(())
    }
}

// let yaw = -1.559;
// let pitch = 0.1459;
// let look_at = Vec3::build(-0.0279, 0.1455, -0.9889);
// let position = Vec3::build(2.009, 9.556, -20.757);
// let front = look_at.normalized();
