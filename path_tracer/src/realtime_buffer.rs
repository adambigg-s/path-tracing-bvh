use minifb::Key;
use minifb::Window;

use crate::camera::Camera;
use crate::scene::Scene;
use crate::utils::packed_color;
use crate::vector::Vec3;

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
}

pub fn run_application(camera: &mut Camera, buffer: &mut Buffer, scene: &Scene, window: &mut Window) {
    while !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::W) {
            camera.position -= camera.front;
        }
        if window.is_key_down(Key::S) {
            camera.position += camera.front;
        }
        if window.is_key_down(Key::A) {
            camera.position -= camera.right;
        }
        if window.is_key_down(Key::D) {
            camera.position += camera.right;
        }
        if window.is_key_down(Key::Q) {
            camera.yaw -= 0.1;
        }
        if window.is_key_down(Key::E) {
            camera.yaw += 0.1;
        }
        if window.is_key_down(Key::R) {
            camera.pitch -= 0.1;
        }
        if window.is_key_down(Key::F) {
            camera.pitch += 0.1;
        }
        if window.is_key_down(Key::Key1) {
            camera.position += camera.world_up;
        }
        if window.is_key_down(Key::Key2) {
            camera.position -= camera.world_up;
        }
        camera.set_viewport();
        camera.render_to_buffer_par(buffer, scene);
        window
            .update_with_buffer(&buffer.pixels, buffer.width, buffer.height)
            .expect("error updating window");
    }
}
