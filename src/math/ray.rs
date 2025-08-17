use crate::math::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3<f64>,
    pub direction: Vec3<f64>,
}

impl Ray {
    pub fn new(origin: Vec3<f64>, direction: Vec3<f64>) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3<f64> {
        self.origin + self.direction * t
    }
}
