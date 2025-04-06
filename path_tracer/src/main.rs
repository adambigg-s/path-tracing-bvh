mod camera;
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
use geometry::Sphere;
use geometry::Triangle;
use materials::Glass;
use materials::Lambertian;
use materials::Material;
use materials::Metal;
use materials::Source;
use minifb::Window;
use minifb::WindowOptions;
use realtime_buffer::run_application;
use realtime_buffer::Buffer;
use scene::Scene;
use utils::Int;
use vector::Vec3;

const HEIGHT: Int = 400;
const WIDTH: Int = 600;

fn main() {
    let mut camera = Camera::build_default(WIDTH, HEIGHT, 7, 150);
    let mut scene = Scene::new();

    let lambertian = Material::Lambertian(Lambertian::build(Vec3::build(0.2, 0.2, 0.3)));
    let glass = Material::Glass(Glass::build(Vec3::build(0.95, 1., 0.97), 1.7));
    let lambertian_green = Material::Lambertian(Lambertian::build(Vec3::build(0.2, 0.9, 0.2)));
    let source = Material::Source(Source::build(Vec3::build(10., 10., 9.)));
    let metal = Material::Metal(Metal::build(Vec3::build(0.5, 0.6, 0.2), 0.3));

    scene.add_sphere(Sphere::build(Vec3::build(0., 0., -1.), 0.5, glass));
    scene.add_sphere(Sphere::build(Vec3::build(0., -100.5, -1.), 100., metal));
    scene.add_sphere(Sphere::build(Vec3::build(2.5, 2.5, 2.5), 0.1, source));
    scene.add_triangle(Triangle::build(
        Vec3::build(0., 1., -2.),
        Vec3::build(0.5, 0., -2.),
        Vec3::build(-0.5, 0., -2.),
        glass,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(0., 0., 0.),
        Vec3::build(5., 5., 0.),
        Vec3::build(0., 5., 0.),
        lambertian,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(0., 0., 0.),
        Vec3::build(5., 5., 0.),
        Vec3::build(5., 0., 0.),
        lambertian,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(0., 0., 5.),
        Vec3::build(5., 5., 5.),
        Vec3::build(0., 5., 5.),
        lambertian_green,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(0., 0., 5.),
        Vec3::build(5., 5., 5.),
        Vec3::build(5., 0., 5.),
        lambertian_green,
    ));

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
        WindowOptions { scale: minifb::Scale::X2, ..Default::default() },
    )
    .expect("failed to get minifb window handle");

    let mut buffer = Buffer::build(HEIGHT as usize, WIDTH as usize);
    run_application(&mut camera, &mut buffer, &scene, &mut window);

    println!("beginning image processing");
    let start_time = std::time::Instant::now();
    camera.render_to_file(&mut file, &scene).expect("failed rendering image: io failure");
    let end_time = start_time.elapsed().as_secs_f32();
    println!("processing time: {} seconds", end_time);
}
