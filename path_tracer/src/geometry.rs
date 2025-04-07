use crate::materials::Material;
use crate::ray_hit::HitRecord;
use crate::ray_hit::Hittable;
use crate::ray_hit::Ray;
use crate::utils::Float;
use crate::utils::Interval;
use crate::vector::Vec3;

pub enum Geometry {
    Sphere(Sphere),
    Triangle(Triangle),
}

impl Hittable for Geometry {
    fn hit(&self, ray: &Ray, record: &mut HitRecord) -> bool {
        match self {
            Geometry::Sphere(sphere) => sphere.hit(ray, record),
            Geometry::Triangle(triangle) => triangle.hit(ray, record),
        }
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: Float,
    pub material: Material,
}

impl Sphere {
    pub const fn build(center: Vec3, radius: Float, material: Material) -> Self {
        Sphere { center, radius, material }
    }

    fn get_normal(&self, at: &Vec3) -> Vec3 {
        (*at - self.center) / self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, record: &mut HitRecord) -> bool {
        let center = self.center - ray.origin;
        let alpha = ray.direction.inner_product(&ray.direction);
        let eta = ray.direction.inner_product(&center);
        let gamma = center.inner_product(&center) - self.radius * self.radius;

        let discrim = eta * eta - alpha * gamma;
        if discrim < 0. {
            return false;
        }

        let sqrt = discrim.sqrt();
        let mut root = (eta - sqrt) / alpha;
        if !record.interval.contains(root) {
            root = (eta + sqrt) / alpha;
            if !record.interval.contains(root) {
                return false;
            }
        }

        record.point = ray.at_time(root);
        record.ray_in = ray.direction;
        record.set_face_normal(&self.get_normal(&record.point));
        record.intersection_time = root;
        record.interval.max = record.intersection_time;
        record.material = self.material;

        true
    }
}

pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    material: Material,
}

impl Triangle {
    pub const fn build(a: Vec3, b: Vec3, c: Vec3, material: Material) -> Self {
        Triangle { a, b, c, material }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, record: &mut HitRecord) -> bool {
        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;

        let pvector = ray.direction.cross_product(&edge2);
        let determinant = edge1.inner_product(&pvector);

        if Interval::near_zero().contains(determinant) {
            return false;
        }

        let tvector = ray.origin - self.a;
        let u = tvector.inner_product(&pvector) / determinant;
        if !Interval::build(0., 1.).contains(u) {
            return false;
        }

        let qvector = tvector.cross_product(&edge1);
        let v = ray.direction.inner_product(&qvector) / determinant;
        if !Interval::build(0., 1.).contains(v) {
            return false;
        }

        if u + v > 1. {
            return false;
        }

        let time = edge2.inner_product(&qvector) / determinant;
        if !record.interval.contains(time) {
            return false;
        }

        record.point = ray.at_time(time);
        record.ray_in = ray.direction;
        record.set_face_normal(&edge1.cross_product(&edge2).normalized());
        record.intersection_time = time;
        record.interval.max = record.intersection_time;
        record.material = self.material;

        true
    }
}
