use definitions::MeshDef as MeshDataFile;

use crate::{
    camera::Camera,
    hittable::Hittable,
    light::PointLight,
    material::{Lambertian, Material},
    math::vec3::Vec3,
    scene::Scene,
    shapes::{mesh::Mesh, sphere::Sphere},
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
struct SceneDef {
    camera: CameraDef,
    background_color: RgbDef,
    ambient_light: Vec3<f64>,
    lights: Vec<PointLight>,
    objects: Vec<ObjectDef>,
}

#[derive(Deserialize)]
struct CameraDef {
    width: u32,
    height: u32,
    lookfrom: Vec3<f64>,
    lookat: Vec3<f64>,
    vup: Vec3<f64>,
    vfov: f64,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum ObjectDef {
    Sphere(SphereDef),
    Mesh(MeshDef),
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum MaterialDef {
    Lambertian(LambertianDef),
}

#[derive(Deserialize)]
struct SphereDef {
    center: Vec3<f64>,
    radius: f64,
    material: MaterialDef,
}

#[derive(Deserialize)]
struct MeshDef {
    path: String,
    material: MaterialDef,
}

#[derive(Deserialize)]
struct LambertianDef {
    color: RgbDef,
}

#[derive(Deserialize, Clone, Copy)]
struct RgbDef([u8; 3]);

pub fn load_scene_from_file(path: &str) -> Result<Scene, Box<dyn std::error::Error>> {
    let scene_data = std::fs::read_to_string(path)?;
    let scene_def: SceneDef = serde_json::from_str(&scene_data)?;
    Ok(scene_def.build())
}

impl SceneDef {
    fn build(self) -> Scene {
        let camera = self.camera.build();
        let hittables = self
            .objects
            .into_iter()
            .map(|obj_def| obj_def.build())
            .collect();

        Scene::new(
            camera,
            self.lights,
            hittables,
            self.background_color.into(),
            self.ambient_light,
        )
    }
}

impl CameraDef {
    fn build(self) -> Camera {
        Camera::new(
            self.width,
            self.height,
            self.lookfrom,
            self.lookat,
            self.vup,
            self.vfov,
        )
    }
}

impl ObjectDef {
    fn build(self) -> Box<dyn Hittable> {
        match self {
            ObjectDef::Sphere(s) => Box::new(s.build()),
            ObjectDef::Mesh(m) => Box::new(m.build()),
        }
    }
}

impl MeshDef {
    fn build(self) -> Mesh {
        let mesh_data_str = std::fs::read_to_string(&self.path)
            .unwrap_or_else(|_| panic!("Failed to load mesh data file: {}", self.path));

        let mesh_data: MeshDataFile = serde_json::from_str(&mesh_data_str)
            .unwrap_or_else(|_| panic!("Failed to parse mesh data from: {}", self.path));

        let vertices = mesh_data
            .vertices
            .into_iter()
            .map(|v| Vec3::new(v.x, v.y, v.z))
            .collect();
        let normals = mesh_data
            .normals
            .into_iter()
            .map(|n| Vec3::new(n.x, n.y, n.z))
            .collect();

        Mesh::new(vertices, mesh_data.indices, normals, self.material.build())
    }
}

impl SphereDef {
    fn build(self) -> Sphere {
        Sphere::new(self.center, self.radius, self.material.build())
    }
}

impl MaterialDef {
    fn build(self) -> Arc<dyn Material> {
        // Changed to return dyn Material
        match self {
            MaterialDef::Lambertian(m) => Arc::new(m.build()),
        }
    }
}

impl LambertianDef {
    fn build(self) -> Lambertian {
        Lambertian::new(self.color.into())
    }
}

impl From<RgbDef> for image::Rgb<u8> {
    fn from(rgb: RgbDef) -> Self {
        image::Rgb(rgb.0)
    }
}
