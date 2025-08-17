use ray_tracer::{definitions::load_scene_from_file, renderer::Renderer};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <path/to/scene.json> <output_path.png>", args[0]);
        std::process::exit(1);
    }
    let scene_path = &args[1];
    let output_path = &args[2];

    // --- Render Quality Settings ---
    const SAMPLES_PER_PIXEL: u32 = 100;

    println!("Loading scene from: '{}'...", scene_path);

    // --- Scene Loading ---
    let scene = match load_scene_from_file(scene_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error loading scene: {}", e);
            std::process::exit(1);
        }
    };

    // --- Rendering ---
    let renderer = Renderer::new(SAMPLES_PER_PIXEL);

    println!("Rendering with {} samples per pixel...", SAMPLES_PER_PIXEL);
    let image_buffer = renderer.render(&scene);

    println!("Saving image to {}...", output_path);
    image_buffer
        .save(output_path)
        .expect("Failed to save image.");

    println!("Done.");

    Ok(())
}
