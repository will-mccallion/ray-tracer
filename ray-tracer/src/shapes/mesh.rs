use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    math::{ray::Ray, vec3::Vec3},
    shapes::triangle::Triangle,
};
use std::sync::Arc;

pub struct Mesh {
    triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new(
        vertices: Vec<Vec3<f64>>,
        indices: Vec<[usize; 3]>,
        normals: Vec<Vec3<f64>>,
        material: Arc<dyn Material>,
    ) -> Self {
        let triangles = indices
            .into_iter()
            .map(|[i0, i1, i2]| {
                Triangle::new(
                    vertices[i0],
                    vertices[i1],
                    vertices[i2],
                    normals[i0],
                    normals[i1],
                    normals[i2],
                    Arc::clone(&material),
                )
            })
            .collect();
        Self { triangles }
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for triangle in &self.triangles {
            if let Some(hit) = triangle.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                closest_hit = Some(hit);
            }
        }
        closest_hit
    }
}
