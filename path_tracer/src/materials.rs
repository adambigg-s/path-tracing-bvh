use crate::ray_hit::HitRecord;
use crate::ray_hit::Ray;
use crate::ray_hit::Scatter;
use crate::ray_hit::ScatterRecord;
use crate::utils::random;
use crate::utils::Float;
use crate::vector::Vec3;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Glass(Glass),
    Source(Source),
}

impl Material {
    pub fn scatter(&self, record: &HitRecord, scatter_record: &mut ScatterRecord) -> bool {
        match self {
            Material::Lambertian(lambertian) => lambertian.scatter(record, scatter_record),
            Material::Metal(metal) => metal.scatter(record, scatter_record),
            Material::Glass(glass) => glass.scatter(record, scatter_record),
            Material::Source(source) => source.scatter(record, scatter_record),
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian(Lambertian::build(Vec3::build(0., 1., 1.)))
    }
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn build(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, hitrecord: &HitRecord, record: &mut ScatterRecord) -> bool {
        let mut scatter_direction = hitrecord.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hitrecord.normal;
        }
        record.scattered = Ray::build(hitrecord.point, scatter_direction);
        record.attenuation = self.albedo;

        true
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzzy: Float,
}

impl Metal {
    pub fn build(albedo: Vec3, fuzzy: Float) -> Self {
        Metal { albedo, fuzzy }
    }
}

impl Scatter for Metal {
    fn scatter(&self, hitrecord: &HitRecord, record: &mut ScatterRecord) -> bool {
        let mut reflected = hitrecord.ray_in.reflect_around(&hitrecord.normal).norm();
        reflected += Vec3::random_unit_vector() * self.fuzzy;
        record.scattered = Ray::build(hitrecord.point, reflected);
        record.attenuation = self.albedo;

        true
    }
}

#[derive(Clone, Copy)]
pub struct Glass {
    pub albedo: Vec3,
    pub refraction_index: Float,
}

impl Glass {
    pub fn build(albedo: Vec3, refraction_index: Float) -> Self {
        Glass { albedo, refraction_index }
    }

    pub fn reflectance(&self, cos: Float) -> Float {
        let re0 = ((1. - self.refraction_index) / (1. + self.refraction_index)).powi(2);
        re0 + (1. - re0) * (1. - cos).powi(5)
    }
}

impl Scatter for Glass {
    fn scatter(&self, hitrecord: &HitRecord, record: &mut ScatterRecord) -> bool {
        let refraction_index = if hitrecord.front_face {
            1. / self.refraction_index
        }
        else {
            self.refraction_index
        };
        let unit_direction = hitrecord.ray_in.norm();
        let cos = -unit_direction.inner_product(&hitrecord.normal).min(1.);
        let sin = (1. - cos * cos).sqrt();
        let direction = if refraction_index * sin > 1. || self.reflectance(cos) > random() {
            unit_direction.reflect_around(&hitrecord.normal)
        }
        else {
            unit_direction.refract_around(&hitrecord.normal, refraction_index)
        };

        record.scattered = Ray::build(hitrecord.point, direction);
        record.attenuation = self.albedo;

        true
    }
}

#[derive(Clone, Copy)]
pub struct Source {
    pub albedo: Vec3,
}

impl Source {
    pub fn build(albedo: Vec3) -> Self {
        Source { albedo }
    }
}

impl Scatter for Source {
    fn scatter(&self, _hitrecord: &HitRecord, record: &mut ScatterRecord) -> bool {
        record.attenuation = self.albedo;
        false
    }
}
