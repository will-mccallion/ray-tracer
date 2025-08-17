use definitions::{MaterialDef, MeshDef, Vec3};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input.obj> <output.json>", args[0]);
        return;
    }

    let obj_path = &args[1];
    let json_path = &args[2];

    println!("Loading OBJ file from: {}", obj_path);

    let (models, _materials) =
        tobj::load_obj(obj_path, &tobj::GPU_LOAD_OPTIONS).expect("Failed to load OBJ file");

    let model = &models[0];
    let mesh = &model.mesh;

    let vertices: Vec<Vec3> = mesh
        .positions
        .chunks_exact(3)
        .map(|v| Vec3 {
            x: v[0] as f64,
            y: v[1] as f64,
            z: -v[2] as f64,
        })
        .collect();

    let normals: Vec<Vec3> = mesh
        .normals
        .chunks_exact(3)
        .map(|n| Vec3 {
            x: n[0] as f64,
            y: n[1] as f64,
            z: -n[2] as f64,
        })
        .collect();

    let indices: Vec<[usize; 3]> = mesh
        .indices
        .chunks_exact(3)
        .map(|i| [i[0] as usize, i[1] as usize, i[2] as usize])
        .collect();

    println!(
        "Found {} vertices, {} normals, and {} triangles.",
        vertices.len(),
        normals.len(),
        indices.len()
    );

    let material = MaterialDef {
        mat_type: "Lambertian".to_string(),
        color: [180, 180, 180],
    };

    let mesh_def = MeshDef {
        obj_type: "Mesh".to_string(),
        vertices,
        indices,
        normals,
        material,
    };

    let json_output = serde_json::to_string_pretty(&mesh_def).expect("Failed to serialize to JSON");
    std::fs::write(json_path, json_output).expect("Failed to write JSON file");

    println!("Successfully converted mesh to {}", json_path);
}
