use minifb::Key;
use minifb::Window;

use crate::camera::Camera;
use crate::scene::Scene;
use crate::utils::packed_color;
use crate::utils::unpack_color;
use crate::utils::Float;
use crate::utils::SMALL;
use crate::vector::Vec3;

#[derive(Clone)]
pub struct Buffer {
    pub pixels: Vec<u32>,
    pub height: usize,
    pub width: usize,
}

impl Buffer {
    pub fn build(height: usize, width: usize) -> Buffer {
        Buffer { height, width, pixels: vec![0; width * height] }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Vec3) {
        self.pixels[y * self.width + x] = packed_color(color);
    }

    // quite terrible and needs to be refactored but it is working well now
    pub fn bilateral_denoise(&mut self, iters: usize) {
        let radius = 3;
        let mut sigma_spatial = 2.0;
        let mut sigma_color = 0.1;

        (0..iters).for_each(|_| {
            let mut guidance_buffer = self.clone();
            guidance_buffer.box_blur(radius);
            let mut result = self.pixels.clone();

            for y in radius..self.height - radius {
                for x in radius..self.width - radius {
                    let center_idx = y * self.width + x;

                    let center_color = unpack_color(guidance_buffer.pixels[center_idx]);
                    let mut color = Vec3::zeros();
                    let mut weight_sum = 0.;

                    (-(radius as isize)..=radius as isize).for_each(|dy| {
                        (-(radius as isize)..=radius as isize).for_each(|dx| {
                            let ny = (y as isize + dy) as usize;
                            let nx = (x as isize + dx) as usize;
                            let idx = ny * self.width + nx;

                            let neighbor_color_input = unpack_color(self.pixels[idx]);
                            let neighbor_color_guidance = unpack_color(guidance_buffer.pixels[idx]);

                            let spatial_dist = (dx * dx + dy * dy) as Float;
                            let color_dist = (neighbor_color_guidance - center_color)
                                .inner_product(&(neighbor_color_guidance - center_color));

                            let spatial_weight = (-spatial_dist / (2. * sigma_spatial * sigma_spatial)).exp();
                            let color_weight = (-color_dist / (2. * sigma_color * sigma_color)).exp();
                            let weight = spatial_weight * color_weight;

                            color += neighbor_color_input * weight;
                            weight_sum += weight;
                        });
                    });
                    let filtered_color = color / weight_sum.max(SMALL);
                    result[center_idx] = packed_color(filtered_color);
                }
            }

            self.pixels = result;
            sigma_color /= 1.2;
            sigma_spatial *= 1.2;
        });
    }

    pub fn box_blur(&mut self, radius: usize) {
        let mut result = self.pixels.clone();
        for y in radius..self.height - radius {
            for x in radius..self.width - radius {
                let mut sum = Vec3::zeros();
                let mut count = 0.;
                for dy in -(radius as isize)..=(radius as isize) {
                    for dx in -(radius as isize)..=(radius as isize) {
                        let ny = (y as isize + dy) as usize;
                        let nx = (x as isize + dx) as usize;
                        let idx = ny * self.width + nx;
                        sum += unpack_color(self.pixels[idx]);
                        count += 1.;
                    }
                }

                let idx = y * self.width + x;
                result[idx] = packed_color(sum / count);
            }
        }

        self.pixels = result;
    }
}

pub fn run_application(camera: &mut Camera, buffer: &mut Buffer, scene: &Scene, window: &mut Window) {
    while !window.is_key_down(Key::Escape) && window.is_open() {
        if window.is_key_down(Key::W) {
            camera.position -= camera.front * camera.move_speed;
        }
        if window.is_key_down(Key::S) {
            camera.position += camera.front * camera.move_speed;
        }
        if window.is_key_down(Key::A) {
            camera.position -= camera.right * camera.move_speed;
        }
        if window.is_key_down(Key::D) {
            camera.position += camera.right * camera.move_speed;
        }
        if window.is_key_down(Key::Q) {
            camera.yaw -= camera.rotation_speed;
        }
        if window.is_key_down(Key::E) {
            camera.yaw += camera.rotation_speed;
        }
        if window.is_key_down(Key::R) {
            camera.pitch -= camera.rotation_speed;
        }
        if window.is_key_down(Key::F) {
            camera.pitch += camera.rotation_speed;
        }
        if window.is_key_down(Key::Key1) {
            camera.position += camera.world_up * camera.move_speed;
        }
        if window.is_key_down(Key::Key2) {
            camera.position -= camera.world_up * camera.move_speed;
        }
        camera.set_viewport();
        camera.render_to_buffer_par(buffer, scene);
        println!(
            "position: {:?} front: {:?}, yaw: {}, pitch: {}",
            camera.position, camera.front, camera.yaw, camera.pitch
        );
        window
            .update_with_buffer(&buffer.pixels, buffer.width, buffer.height)
            .expect("error updating window");
    }
}
