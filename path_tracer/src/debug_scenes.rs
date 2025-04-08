#![allow(dead_code)]

use crate::geometry::Sphere;
use crate::geometry::Triangle;
use crate::materials::Glass;
use crate::materials::Lambertian;
use crate::materials::Material;
use crate::materials::Metal;
use crate::materials::Source;
use crate::scene::Scene;
use crate::vector::Vec3;

// i really need to make an ergonomic scene editor
// goddamn this sucks
// it is so much boilerplate tho to make a system better than this
pub fn cornell_room(scene: &mut Scene) {
    let lambertian_red = Material::Lambertian(Lambertian::build(Vec3::build(0.65, 0.05, 0.05)));
    let lambertian_green = Material::Lambertian(Lambertian::build(Vec3::build(0.12, 0.45, 0.15)));
    let lambertian_white = Material::Lambertian(Lambertian::build(Vec3::build(0.73, 0.73, 0.73)));
    let source = Material::Source(Source::build(Vec3::build(15., 13., 5.)));
    let mirror = Material::Metal(Metal::build(Vec3::build(0.82, 0.95, 0.93), 0.005));
    let ideal_mirror = Material::Metal(Metal::build(Vec3::build(1., 1., 1.), 0.));
    let glass = Material::Glass(Glass::build(Vec3::build(1., 1., 1.), 1.5));
    let blue_glass = Material::Glass(Glass::build(Vec3::build(0.95, 0.95, 1.), 1.9));

    let scaling = 3.;
    let ceiling_y = 6.;
    let z_min = -8.;
    let z_max = 8.;

    // floor
    scene.add_triangle(Triangle::build(
        Vec3::build(-5., 0., z_min) * scaling,
        Vec3::build(5., 0., z_min) * scaling,
        Vec3::build(5., 0., z_max) * scaling,
        lambertian_white,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(-5., 0., z_min) * scaling,
        Vec3::build(5., 0., z_max) * scaling,
        Vec3::build(-5., 0., z_max) * scaling,
        lambertian_white,
    ));
    // ceiling
    scene.add_triangle(Triangle::build(
        Vec3::build(-5., ceiling_y, z_min) * scaling,
        Vec3::build(5., ceiling_y, z_min) * scaling,
        Vec3::build(5., ceiling_y, z_max) * scaling,
        lambertian_white,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(-5., ceiling_y, z_min) * scaling,
        Vec3::build(5., ceiling_y, z_max) * scaling,
        Vec3::build(-5., ceiling_y, z_max) * scaling,
        lambertian_white,
    ));
    // back wall
    scene.add_triangle(Triangle::build(
        Vec3::build(-5., 0., z_min) * scaling,
        Vec3::build(5., 0., z_min) * scaling,
        Vec3::build(5., ceiling_y, z_min) * scaling,
        lambertian_white,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(-5., 0., z_min) * scaling,
        Vec3::build(5., ceiling_y, z_min) * scaling,
        Vec3::build(-5., ceiling_y, z_min) * scaling,
        lambertian_white,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(-3.8, 0.5, z_min + 0.01) * scaling,
        Vec3::build(3.8, 0.5, z_min + 0.01) * scaling,
        Vec3::build(3.8, ceiling_y - 0.2, z_min + 0.01) * scaling,
        mirror,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(-3.8, 0.5, z_min + 0.01) * scaling,
        Vec3::build(3.8, ceiling_y - 0.2, z_min + 0.01) * scaling,
        Vec3::build(-3.8, ceiling_y - 0.2, z_min + 0.01) * scaling,
        mirror,
    ));
    // front wall
    scene.add_triangle(Triangle::build(
        Vec3::build(-5., 0., z_max) * scaling,
        Vec3::build(5., 0., z_max) * scaling,
        Vec3::build(5., ceiling_y, z_max) * scaling,
        lambertian_white,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(-5., 0., z_max) * scaling,
        Vec3::build(5., ceiling_y, z_max) * scaling,
        Vec3::build(-5., ceiling_y, z_max) * scaling,
        lambertian_white,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(-4.1, 0.5, z_max - 0.01) * scaling,
        Vec3::build(4.1, 0.5, z_max - 0.01) * scaling,
        Vec3::build(4.1, ceiling_y - 0.2, z_max - 0.01) * scaling,
        mirror,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(-4.1, 0.5, z_max - 0.01) * scaling,
        Vec3::build(4.1, ceiling_y - 0.2, z_max - 0.01) * scaling,
        Vec3::build(-4.1, ceiling_y - 0.2, z_max - 0.01) * scaling,
        mirror,
    ));
    // left wall
    scene.add_triangle(Triangle::build(
        Vec3::build(-5., 0., z_min) * scaling,
        Vec3::build(-5., 0., z_max) * scaling,
        Vec3::build(-5., ceiling_y, z_max) * scaling,
        lambertian_red,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(-5., 0., z_min) * scaling,
        Vec3::build(-5., ceiling_y, z_max) * scaling,
        Vec3::build(-5., ceiling_y, z_min) * scaling,
        lambertian_red,
    ));
    // right wall
    scene.add_triangle(Triangle::build(
        Vec3::build(5., 0., z_min) * scaling,
        Vec3::build(5., 0., z_max) * scaling,
        Vec3::build(5., ceiling_y, z_max) * scaling,
        lambertian_green,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(5., 0., z_min) * scaling,
        Vec3::build(5., ceiling_y, z_max) * scaling,
        Vec3::build(5., ceiling_y, z_min) * scaling,
        lambertian_green,
    ));
    // light on the ceiling
    scene.add_triangle(Triangle::build(
        Vec3::build(-1.5, ceiling_y - 0.01, -1.5) * scaling,
        Vec3::build(1.5, ceiling_y - 0.01, -1.5) * scaling,
        Vec3::build(1.5, ceiling_y - 0.01, 1.5) * scaling,
        source,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(-1.5, ceiling_y - 0.01, -1.5) * scaling,
        Vec3::build(1.5, ceiling_y - 0.01, 1.5) * scaling,
        Vec3::build(-1.5, ceiling_y - 0.01, 1.5) * scaling,
        source,
    ));

    // prism - small
    let base = Vec3::build(2.2, 0., -2.) * scaling;
    let width = 1.5 * scaling;
    let height = 2.5 * scaling;
    let depth = 1.5 * scaling;
    let p0 = base;
    let p1 = base + Vec3::build(width, 0., 0.).rotate_y(0.3);
    let p2 = base + Vec3::build(width, 0., depth).rotate_y(0.3);
    let p3 = base + Vec3::build(0., 0., depth).rotate_y(0.3);
    let p4 = p0 + Vec3::build(0., height, 0.).rotate_y(0.3);
    let p5 = p1 + Vec3::build(0., height, 0.).rotate_y(0.3);
    let p6 = p2 + Vec3::build(0., height, 0.).rotate_y(0.3);
    let p7 = p3 + Vec3::build(0., height, 0.).rotate_y(0.3);
    // bottom
    scene.add_triangle(Triangle::build(p0, p1, p2, lambertian_white));
    scene.add_triangle(Triangle::build(p0, p2, p3, lambertian_white));
    // top
    scene.add_triangle(Triangle::build(p4, p5, p6, lambertian_white));
    scene.add_triangle(Triangle::build(p4, p6, p7, lambertian_white));
    // front
    scene.add_triangle(Triangle::build(p3, p2, p6, lambertian_white));
    scene.add_triangle(Triangle::build(p3, p6, p7, lambertian_white));
    // back
    scene.add_triangle(Triangle::build(p0, p1, p5, lambertian_white));
    scene.add_triangle(Triangle::build(p0, p5, p4, lambertian_white));
    // left
    scene.add_triangle(Triangle::build(p0, p3, p7, lambertian_white));
    scene.add_triangle(Triangle::build(p0, p7, p4, lambertian_white));
    // right
    scene.add_triangle(Triangle::build(p1, p2, p6, lambertian_white));
    scene.add_triangle(Triangle::build(p1, p6, p5, lambertian_white));

    // glass pyramid
    let base_center = Vec3::build(0., 0., 0.) * scaling;
    let base_size = 1.8 * scaling;
    let height = 2. * scaling;
    let gp0 = base_center + Vec3::build(-base_size / 2., 0., -base_size / 2.).rotate_y(-0.1);
    let gp1 = base_center + Vec3::build(base_size / 2., 0., -base_size / 2.).rotate_y(-0.2);
    let gp2 = base_center + Vec3::build(base_size / 2., 0., base_size / 2.).rotate_y(-0.2);
    let gp3 = base_center + Vec3::build(-base_size / 2., 0., base_size / 2.).rotate_y(-0.2);
    let apex = base_center + Vec3::build(0., height, 0.).rotate_y(-0.2);
    // base
    scene.add_triangle(Triangle::build(gp0, gp1, gp2, glass));
    scene.add_triangle(Triangle::build(gp0, gp2, gp3, glass));
    // sides
    scene.add_triangle(Triangle::build(gp0, gp1, apex, glass));
    scene.add_triangle(Triangle::build(gp1, gp2, apex, glass));
    scene.add_triangle(Triangle::build(gp2, gp3, apex, glass));
    scene.add_triangle(Triangle::build(gp3, gp0, apex, glass));

    // glass pillar near the small prism
    let glass_base = Vec3::build(-1.6, 0., 2.5) * scaling;
    let glass_width = 0.9 * scaling;
    let glass_height = 2.1 * scaling;
    let glass_depth = 0.9 * scaling;
    let g0 = glass_base;
    let g1 = glass_base + Vec3::build(glass_width, 0., 0.).rotate_y(-0.5);
    let g2 = glass_base + Vec3::build(glass_width, 0., glass_depth).rotate_y(-0.5);
    let g3 = glass_base + Vec3::build(0., 0., glass_depth).rotate_y(-0.5);
    let g4 = g0 + Vec3::build(0., glass_height, 0.).rotate_y(-0.5);
    let g5 = g1 + Vec3::build(0., glass_height, 0.).rotate_y(-0.5);
    let g6 = g2 + Vec3::build(0., glass_height, 0.).rotate_y(-0.5);
    let g7 = g3 + Vec3::build(0., glass_height, 0.).rotate_y(-0.5);
    // bottom
    scene.add_triangle(Triangle::build(g0, g1, g2, blue_glass));
    scene.add_triangle(Triangle::build(g0, g2, g3, blue_glass));
    // top
    scene.add_triangle(Triangle::build(g4, g5, g6, blue_glass));
    scene.add_triangle(Triangle::build(g4, g6, g7, blue_glass));
    // front
    scene.add_triangle(Triangle::build(g3, g2, g6, blue_glass));
    scene.add_triangle(Triangle::build(g3, g6, g7, blue_glass));
    // back
    scene.add_triangle(Triangle::build(g0, g1, g5, blue_glass));
    scene.add_triangle(Triangle::build(g0, g5, g4, blue_glass));
    // left
    scene.add_triangle(Triangle::build(g0, g3, g7, blue_glass));
    scene.add_triangle(Triangle::build(g0, g7, g4, blue_glass));
    // right
    scene.add_triangle(Triangle::build(g1, g2, g6, blue_glass));
    scene.add_triangle(Triangle::build(g1, g6, g5, blue_glass));

    // prism tall
    let base2 = Vec3::build(2.1, 0., 3.5) * scaling;
    let width2 = 1.2 * scaling;
    let height2 = 4. * scaling;
    let depth2 = 1.2 * scaling;
    let q0 = base2;
    let q1 = base2 + Vec3::build(width2, 0., 0.).rotate_y(-0.2);
    let q2 = base2 + Vec3::build(width2, 0., depth2).rotate_y(-0.2);
    let q3 = base2 + Vec3::build(0., 0., depth2).rotate_y(-0.2);
    let q4 = q0 + Vec3::build(0., height2, 0.).rotate_y(-0.2);
    let q5 = q1 + Vec3::build(0., height2, 0.).rotate_y(-0.2);
    let q6 = q2 + Vec3::build(0., height2, 0.).rotate_y(-0.2);
    let q7 = q3 + Vec3::build(0., height2, 0.).rotate_y(-0.2);
    // bottom
    scene.add_triangle(Triangle::build(q0, q1, q2, lambertian_white));
    scene.add_triangle(Triangle::build(q0, q2, q3, lambertian_white));
    // top
    scene.add_triangle(Triangle::build(q4, q5, q6, lambertian_white));
    scene.add_triangle(Triangle::build(q4, q6, q7, lambertian_white));
    // front
    scene.add_triangle(Triangle::build(q3, q2, q6, lambertian_white));
    scene.add_triangle(Triangle::build(q3, q6, q7, lambertian_white));
    // back
    scene.add_triangle(Triangle::build(q0, q1, q5, lambertian_white));
    scene.add_triangle(Triangle::build(q0, q5, q4, lambertian_white));
    // left
    scene.add_triangle(Triangle::build(q0, q3, q7, lambertian_white));
    scene.add_triangle(Triangle::build(q0, q7, q4, lambertian_white));
    // right
    scene.add_triangle(Triangle::build(q1, q2, q6, lambertian_white));
    scene.add_triangle(Triangle::build(q1, q6, q5, lambertian_white));

    // spheres in the scene
    scene.add_sphere(Sphere::build(Vec3::build(-2.5, 1., -1.) * scaling, 1. * scaling, ideal_mirror));
}

pub fn cornell_basic(scene: &mut Scene) {
    let red = Material::Lambertian(Lambertian::build(Vec3::build(0.65, 0.05, 0.05)));
    let green = Material::Lambertian(Lambertian::build(Vec3::build(0.12, 0.45, 0.15)));
    let white = Material::Lambertian(Lambertian::build(Vec3::build(0.73, 0.73, 0.73)));
    let light = Material::Source(Source::build(Vec3::build(15., 15., 15.)));

    let width = 5.0;

    let floor = 0.0;
    let ceiling = 5.5;
    let z_min = -5.0;
    let z_max = 5.0;

    scene.add_triangle(Triangle::build(
        Vec3::build(-width, floor, z_min),
        Vec3::build(width, floor, z_min),
        Vec3::build(width, floor, z_max),
        white,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(-width, floor, z_min),
        Vec3::build(width, floor, z_max),
        Vec3::build(-width, floor, z_max),
        white,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(-width, ceiling, z_min),
        Vec3::build(width, ceiling, z_min),
        Vec3::build(width, ceiling, z_max),
        white,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(-width, ceiling, z_min),
        Vec3::build(width, ceiling, z_max),
        Vec3::build(-width, ceiling, z_max),
        white,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(-width, floor, z_min),
        Vec3::build(width, floor, z_min),
        Vec3::build(width, ceiling, z_min),
        white,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(-width, floor, z_min),
        Vec3::build(width, ceiling, z_min),
        Vec3::build(-width, ceiling, z_min),
        white,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(-width, floor, z_min),
        Vec3::build(-width, floor, z_max),
        Vec3::build(-width, ceiling, z_max),
        red,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(-width, floor, z_min),
        Vec3::build(-width, ceiling, z_max),
        Vec3::build(-width, ceiling, z_min),
        red,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(width, floor, z_min),
        Vec3::build(width, floor, z_max),
        Vec3::build(width, ceiling, z_max),
        green,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(width, floor, z_min),
        Vec3::build(width, ceiling, z_max),
        Vec3::build(width, ceiling, z_min),
        green,
    ));

    scene.add_triangle(Triangle::build(
        Vec3::build(-1.0, ceiling - 0.01, -1.0),
        Vec3::build(1.0, ceiling - 0.01, -1.0),
        Vec3::build(1.0, ceiling - 0.01, 1.0),
        light,
    ));
    scene.add_triangle(Triangle::build(
        Vec3::build(-1.0, ceiling - 0.01, -1.0),
        Vec3::build(1.0, ceiling - 0.01, 1.0),
        Vec3::build(-1.0, ceiling - 0.01, 1.0),
        light,
    ));

    let p0 = Vec3::build(-2.5, floor, 2.5);
    let p1 = Vec3::build(-0.5, floor, 2.5);
    let p2 = Vec3::build(-0.5, floor, 0.5);
    let p3 = Vec3::build(-2.5, floor, 0.5);
    let height1 = 1.5;
    let p4 = p0 + Vec3::build(0., height1, 0.);
    let p5 = p1 + Vec3::build(0., height1, 0.);
    let p6 = p2 + Vec3::build(0., height1, 0.);
    let p7 = p3 + Vec3::build(0., height1, 0.);
    scene.add_triangle(Triangle::build(p0, p1, p2, white));
    scene.add_triangle(Triangle::build(p0, p2, p3, white));
    scene.add_triangle(Triangle::build(p4, p5, p6, white));
    scene.add_triangle(Triangle::build(p4, p6, p7, white));
    scene.add_triangle(Triangle::build(p3, p2, p6, white));
    scene.add_triangle(Triangle::build(p3, p6, p7, white));
    scene.add_triangle(Triangle::build(p0, p1, p5, white));
    scene.add_triangle(Triangle::build(p0, p5, p4, white));
    scene.add_triangle(Triangle::build(p0, p3, p7, white));
    scene.add_triangle(Triangle::build(p0, p7, p4, white));
    scene.add_triangle(Triangle::build(p1, p2, p6, white));
    scene.add_triangle(Triangle::build(p1, p6, p5, white));

    let q0 = Vec3::build(0.5, floor, -2.0);
    let q1 = Vec3::build(2.3, floor, -2.0);
    let q2 = Vec3::build(2.3, floor, -0.2);
    let q3 = Vec3::build(0.5, floor, -0.2);
    let height2 = 3.0;
    let q4 = q0 + Vec3::build(0., height2, 0.);
    let q5 = q1 + Vec3::build(0., height2, 0.);
    let q6 = q2 + Vec3::build(0., height2, 0.);
    let q7 = q3 + Vec3::build(0., height2, 0.);
    scene.add_triangle(Triangle::build(q0, q1, q2, white));
    scene.add_triangle(Triangle::build(q0, q2, q3, white));
    scene.add_triangle(Triangle::build(q4, q5, q6, white));
    scene.add_triangle(Triangle::build(q4, q6, q7, white));
    scene.add_triangle(Triangle::build(q3, q2, q6, white));
    scene.add_triangle(Triangle::build(q3, q6, q7, white));
    scene.add_triangle(Triangle::build(q0, q1, q5, white));
    scene.add_triangle(Triangle::build(q0, q5, q4, white));
    scene.add_triangle(Triangle::build(q0, q3, q7, white));
    scene.add_triangle(Triangle::build(q0, q7, q4, white));
    scene.add_triangle(Triangle::build(q1, q2, q6, white));
    scene.add_triangle(Triangle::build(q1, q6, q5, white));
}
