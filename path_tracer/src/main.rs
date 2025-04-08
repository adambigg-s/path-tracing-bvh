mod camera;
mod debug_scenes;
mod geometry;
mod materials;
mod ray_hit;
mod realtime_buffer;
mod scene;
mod utils;
mod vector;

use std::env;
use std::fs::File;
use std::path::Path;

use camera::Camera;
use minifb::Window;
use minifb::WindowOptions;
use realtime_buffer::run_application;
use realtime_buffer::Buffer;
use scene::Scene;
use utils::Int;
use vector::Vec3;

const WIDTH: Int = 1920 / UPSCALE;
const HEIGHT: Int = 1080 / UPSCALE;
const UPSCALE: Int = 5;

fn main() {
    let mut camera = Camera::build_default(WIDTH, HEIGHT, 20, 1000, UPSCALE);
    let mut scene = Scene::new();
    debug_scenes::cornell_room(&mut scene);

    let envs: Vec<String> = env::args().collect();
    let path = match envs.get(1) {
        Some(env) => Path::new(env),
        None => Path::new("../images/dump.ppm"),
    };
    let mut file = File::create(path).expect("failed to create file");
    let mut window = Window::new(
        "cpu path traced",
        WIDTH as usize,
        HEIGHT as usize,
        WindowOptions { scale: minifb::Scale::X4, ..Default::default() },
    )
    .expect("failed to get minifb window handle");
    let mut buffer = Buffer::build(HEIGHT as usize, WIDTH as usize);

    run_application(&mut camera, &mut buffer, &scene, &mut window);

    println!("beginning image processing");
    let start_time = std::time::Instant::now();
    camera.render_to_file_par(&mut file, &scene, true, 1).expect("failed rendering image: io failure");
    let end_time = start_time.elapsed();
    println!("process time: {} seconds\n{} minutes", end_time.as_secs_f32(), end_time.as_secs_f32() / 60.);
}
