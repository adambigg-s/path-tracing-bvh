mod camera;
mod geometry;
mod vector;

use std::{env, fs::File, path::Path};

use camera::*;

type Float = f32;
type Int = i32;

fn main() {
    let camera = Camera::build_default(600, 400);

    let envs: Vec<String> = env::args().collect();
    let path = if envs.len() > 1 {
        Path::new(&envs[1])
    } else {
        Path::new("../images/dump.ppm")
    };
    let mut file = File::create(path).expect("failed to create file");

    println!("beginning image processing");
    let start_time = std::time::Instant::now();
    camera
        .render(&mut file)
        .expect("failed rendering image: io failure");
    let end_time = start_time.elapsed().as_secs_f32();
    println!("processing time: {} seconds", end_time);
}
