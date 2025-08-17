use crate::material::Material;
use crate::math::{ray::Ray, vec3::Vec3};
use std::sync::Arc;

pub struct HitRecord {
    pub t: f64,
    pub point: Vec3<f64>,
    pub normal: Vec3<f64>,
    pub material: Arc<dyn Material>,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
