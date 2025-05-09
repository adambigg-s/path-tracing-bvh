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
    let mut camera = Camera {
        height: HEIGHT,
        width: WIDTH,
        position: Vec3::build(0., 2., 7.),
        fov: 55.,
        focal_length: 1.,
        yaw: 3.141934 / 2.,
        pitch: 0.,
        move_speed: 0.15,
        rotation_speed: 0.03,
        max_recursive_depth: 4,
        denoise_iters: 1,
        samples: 1,
        ..Default::default()
    };
    camera.build_params();
    let mut scene = Scene::new();
    debug_scenes::cornell_basic(&mut scene);

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

    camera.height *= UPSCALE;
    camera.width *= UPSCALE;
    camera.samples = 30;
    camera.max_recursive_depth = 30;
    camera.build_params();
    println!("beginning image processing");
    let start_time = std::time::Instant::now();
    camera.render_to_file_par(&mut file, &scene).expect("failed rendering image: io failure");
    let end_time = start_time.elapsed();
    println!("process time: {} seconds\n{} minutes", end_time.as_secs_f32(), end_time.as_secs_f32() / 60.);
}
