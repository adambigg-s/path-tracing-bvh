use crate::geometry::Geometry;
use crate::geometry::Sphere;
use crate::geometry::Triangle;
use crate::ray_hit::HitRecord;
use crate::ray_hit::Hittable;
use crate::ray_hit::Ray;
use crate::ray_hit::ScatterRecord;
use crate::utils::sky_gradient;
use crate::utils::Int;
use crate::vector::Vec3;

pub struct Scene {
    hittables: Vec<Geometry>,
}

impl Scene {
    pub fn new() -> Self {
        Scene { hittables: Vec::new() }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.hittables.push(Geometry::Sphere(sphere));
    }

    pub fn add_triangle(&mut self, triangle: Triangle) {
        self.hittables.push(Geometry::Triangle(triangle));
    }

    pub fn get_color(&self, ray: &Ray, depth: Int) -> Vec3 {
        if depth.is_negative() {
            return Vec3::zeros();
        }

        let mut record = HitRecord::new();
        if !self.hit(ray, &mut record) {
            return sky_gradient(ray);
        }

        let mut scatter = ScatterRecord::new();
        if record.material.scatter(&record, &mut scatter) {
            return self.get_color(&scatter.scattered, depth - 1).mul_component(&scatter.attenuation);
        }

        scatter.attenuation
    }

    pub fn hit(&self, ray: &Ray, record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut temp_record = HitRecord::new();
        for hittable in &self.hittables {
            if hittable.hit(ray, &mut temp_record) {
                hit_anything = true;
                *record = temp_record;
            }
        }

        hit_anything
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
