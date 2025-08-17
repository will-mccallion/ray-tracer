use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    math::{ray::Ray, vec3::Vec3},
};
use std::sync::Arc;

pub struct Triangle {
    v0: Vec3<f64>,
    v1: Vec3<f64>,
    v2: Vec3<f64>,
    n0: Vec3<f64>,
    n1: Vec3<f64>,
    n2: Vec3<f64>,
    material: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(
        v0: Vec3<f64>,
        v1: Vec3<f64>,
        v2: Vec3<f64>,
        n0: Vec3<f64>,
        n1: Vec3<f64>,
        n2: Vec3<f64>,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            v0,
            v1,
            v2,
            n0,
            n1,
            n2,
            material,
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        const EPSILON: f64 = 0.000001;
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let h = ray.direction.cross(&edge2);
        let a = edge1.dot(&h);

        if a > -EPSILON && a < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - self.v0;
        let u = f * s.dot(&h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * ray.direction.dot(&q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(&q);

        if t > t_min && t < t_max {
            let point = ray.at(t);

            let w = 1.0 - u - v;
            let interpolated_normal = (self.n0 * w + self.n1 * u + self.n2 * v).normalize();

            return Some(HitRecord {
                t,
                point,
                normal: interpolated_normal,
                material: Arc::clone(&self.material),
            });
        }

        None
    }
}
