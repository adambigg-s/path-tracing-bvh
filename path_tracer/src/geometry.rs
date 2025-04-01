use crate::{Float, vector::Vec3};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn build(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at_time(&self, time: Float) -> Vec3 {
        self.origin + self.direction * time
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float, hitrecord: &mut HitRecord) -> bool;
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub time: Float,
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: Float,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float, hitrecord: &mut HitRecord) -> bool {
        let rel_center = self.center - ray.origin;
        let alpha = ray.direction.inner_product(&ray.direction);
        let eta = ray.direction.inner_product(&rel_center);
        let gamma = rel_center.inner_product(&rel_center) - self.radius * self.radius;

        let discrim = eta * eta - alpha * gamma;
        if discrim < 0. {
            return false;
        }

        let sqrt = discrim.sqrt();

        let mut root = (eta - sqrt) / alpha;
        if root <= t_min || t_max <= root {
            root = (eta + sqrt) / alpha;
            if root <= t_min || t_max <= root {
                return false;
            }
        }

        hitrecord.time = root;
        hitrecord.point = ray.at_time(root);
        hitrecord.normal = (hitrecord.point - self.center) / self.radius;
        true
    }
}

pub struct Triangle {
    p1: Vec3,
    p2: Vec3,
    p3: Vec3,
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float, hitrecord: &mut HitRecord) -> bool {
        false
    }
}

pub enum Geometry {
    Sphere { sphere: Sphere },
    Triangle { triangle: Triangle },
}

impl Hittable for Geometry {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float, hitrecord: &mut HitRecord) -> bool {
        match self {
            Geometry::Sphere { sphere } => sphere.hit(ray, t_min, t_max, hitrecord),
            Geometry::Triangle { triangle } => triangle.hit(ray, t_min, t_max, hitrecord),
        }
    }
}
