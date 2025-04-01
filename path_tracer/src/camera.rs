use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use crate::{Float, Int, geometry::Ray, vector::Vec3};

pub struct Camera {
    pub height: Int,
    pub width: Int,
    pub aspect_ratio: Float,
    pub fov: Float,
    pub center: Vec3,
    pub pixel_top_left: Vec3,
    pub pixel_du: Vec3,
    pub pixel_dv: Vec3,
}

impl Camera {
    pub fn build_default(width: Int, height: Int) -> Self {
        let aspect_ratio = width as Float / height as Float;
        let viewport_height = 2.;
        let viewport_width = viewport_height * aspect_ratio;

        let focal_length = 1.;
        let camera_center = Vec3::build(0., 0., 0.);
        let viewport_u = Vec3::build(viewport_width, 0., 0.);
        let viewport_v = Vec3::build(0., -viewport_height, 0.);

        let pixel_delta_u = viewport_u / width as f32;
        let pixel_delta_v = viewport_v / height as f32;

        let viewport_upper_left =
            camera_center - Vec3::build(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel_center_location = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Camera {
            height,
            width,
            aspect_ratio,
            fov: Float::default(),
            center: camera_center,
            pixel_top_left: pixel_center_location,
            pixel_du: pixel_delta_u,
            pixel_dv: pixel_delta_v,
        }
    }

    pub fn get_color(&self, ray: Ray) -> Vec3 {
        let time = hit_sphere(Vec3::build(0., 0., -1.), 0.5, &ray);
        if time > 0. {
            let normal = (ray.at_time(time) - Vec3::build(0., 0., -1.)).norm();
            return Vec3::build(normal.x + 1., normal.y + 1., normal.z + 1.) * 0.5;
        }
        let unit = ray.direction.norm();
        let alpha = 0.5 * (unit.y + 1.);
        Vec3::build(1., 1., 1.) * (1. - alpha) + Vec3::build(0.5, 0.7, 1.) * alpha
    }

    pub fn render(&self, file: &mut File) -> io::Result<()> {
        let printerval = self.height / 100;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "P3\n{} {}\n255", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel_center = self.pixel_top_left
                    + (self.pixel_du * x as Float)
                    + (self.pixel_dv * y as Float);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::build(self.center, ray_direction);
                let pixel_color = self.get_color(ray);
                self.write_pixel(&mut writer, pixel_color)?;
            }
            writeln!(writer)?;
            if y % printerval == 0 {
                println!("progress: {:.1}%", y as f32 / self.height as f32 * 100.);
            }
        }
        writer.flush()?;
        Ok(())
    }

    pub fn write_pixel(&self, writer: &mut BufWriter<&mut File>, color: Vec3) -> io::Result<()> {
        let red = (color.x * 255.) as u8;
        let green = (color.y * 255.) as u8;
        let blue = (color.z * 255.) as u8;
        write!(writer, "{} {} {} ", red, green, blue)?;
        Ok(())
    }
}

pub fn hit_sphere(center: Vec3, radius: Float, ray: &Ray) -> Float {
    let rel_center = center - ray.origin;
    let a = ray.direction.inner_product(&ray.direction);
    let h = ray.direction.inner_product(&rel_center);
    let c = rel_center.inner_product(&rel_center) - radius * radius;
    let discrim = h * h - a * c;

    if discrim < 0. {
        -1.
    } else {
        (h - discrim.sqrt()) / a
    }
}
