// src/scene.rs

use crate::{camera::Camera, hittable::Hittable, light::PointLight, math::vec3::Vec3};
use image::Rgb;

pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<PointLight>,
    pub hittables: Vec<Box<dyn Hittable>>,
    pub background_color: Rgb<u8>,
    pub ambient_light: Vec3<f64>,
}

impl Scene {
    pub fn new(
        camera: Camera,
        lights: Vec<PointLight>,
        hittables: Vec<Box<dyn Hittable>>,
        background_color: Rgb<u8>,
        ambient_light: Vec3<f64>,
    ) -> Self {
        Self {
            camera,
            lights,
            hittables,
            background_color,
            ambient_light,
        }
    }
}
