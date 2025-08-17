use crate::math::vec3::Vec3;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PointLight {
    pub position: Vec3<f64>,
    pub intensity: f64,
}

impl PointLight {
    pub fn new(position: Vec3<f64>, intensity: f64) -> Self {
        Self {
            position,
            intensity,
        }
    }
}
