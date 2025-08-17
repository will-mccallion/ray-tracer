use clap::Parser;
use ray_tracer::{definitions::load_scene_from_file, renderer::Renderer};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    scene_path: String,

    output_path: String,

    #[arg(short, long, default_value_t = 100)]
    samples: u32,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    println!("Loading scene from: '{}'...", &args.scene_path);

    let scene = match load_scene_from_file(&args.scene_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error loading scene: {}", e);
            std::process::exit(1);
        }
    };

    // --- Rendering ---
    let renderer = Renderer::new(args.samples);

    println!("Rendering with {} samples per pixel...", args.samples);
    let image_buffer = renderer.render(&scene);

    println!("Saving image to {}...", &args.output_path);
    image_buffer
        .save(&args.output_path)
        .expect("Failed to save image.");

    println!("Done.");

    Ok(())
}
