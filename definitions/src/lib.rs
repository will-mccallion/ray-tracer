use serde::{Deserialize, Serialize};

// This Vec3 will be used by both projects for serialization.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// The JSON definition for a mesh object, including normals.
#[derive(Serialize, Deserialize)]
pub struct MeshDef {
    #[serde(rename = "type")]
    pub obj_type: String,
    pub vertices: Vec<Vec3>,
    pub indices: Vec<[usize; 3]>,
    pub normals: Vec<Vec3>,
    pub material: MaterialDef,
}

#[derive(Serialize, Deserialize)]
pub struct MaterialDef {
    #[serde(rename = "type")]
    pub mat_type: String,
    pub color: [u8; 3],
}
