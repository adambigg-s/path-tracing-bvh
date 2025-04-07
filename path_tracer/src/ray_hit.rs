use crate::materials::Material;
use crate::utils::Float;
use crate::utils::Interval;
use crate::utils::INFIN;
use crate::Vec3;

pub trait Hittable {
    fn hit(&self, ray: &Ray, record: &mut HitRecord) -> bool;
}

pub trait Scatter {
    fn scatter(&self, record: &HitRecord, scatter: &mut ScatterRecord) -> bool;
}

#[derive(Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub const fn build(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at_time(&self, time: Float) -> Vec3 {
        self.origin + self.direction * time
    }
}

#[derive(Clone, Copy, Default)]
pub struct HitRecord {
    pub point: Vec3,
    pub ray_in: Vec3,
    pub normal: Vec3,
    pub intersection_time: Float,
    pub interval: Interval,
    pub material: Material,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord { intersection_time: -INFIN, ..Default::default() }
    }

    pub fn set_face_normal(&mut self, outward_normal: &Vec3) {
        self.front_face = self.ray_in.inner_product(outward_normal) < 0.;
        if self.front_face {
            self.normal = *outward_normal;
        }
        else {
            self.normal = -(*outward_normal);
        }
    }
}

#[derive(Default)]
pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

impl ScatterRecord {
    pub fn new() -> Self {
        ScatterRecord { attenuation: Vec3::zeros(), scattered: Ray::default() }
    }
}
