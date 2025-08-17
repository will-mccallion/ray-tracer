// src/renderer.rs

use crate::math::vec3::Vec3;
use crate::scene::Scene;
use image::{Rgb, RgbImage};
// --- NEW: Import the necessary components from indicatif ---
use indicatif::{ProgressBar, ProgressStyle};

pub struct Renderer {
    pub samples_per_pixel: u32,
}

impl Renderer {
    pub fn new(samples_per_pixel: u32) -> Self {
        Self { samples_per_pixel }
    }

    pub fn render(&self, scene: &Scene) -> RgbImage {
        let mut image_buffer = RgbImage::new(scene.camera.width, scene.camera.height);
        let total_pixels = (scene.camera.width * scene.camera.height) as u64;

        let pb = ProgressBar::new(total_pixels);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} Rendering: [{bar:40.cyan/blue}] {percent}% | {pos}/{len}px ({eta})")
            .expect("Failed to create progress bar template")
            .progress_chars("#>-"));

        for py in 0..scene.camera.height {
            for px in 0..scene.camera.width {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

                for _s in 0..self.samples_per_pixel {
                    let u = (px as f64 + rand::random::<f64>()) / (scene.camera.width - 1) as f64;
                    let v = (py as f64 + rand::random::<f64>()) / (scene.camera.height - 1) as f64;
                    let ray = scene.camera.get_ray(u, 1.0 - v);
                    pixel_color = pixel_color + self.trace_ray(&ray, scene);
                }

                pixel_color = pixel_color / self.samples_per_pixel as f64;

                let r = pixel_color.x.powf(0.5);
                let g = pixel_color.y.powf(0.5);
                let b = pixel_color.z.powf(0.5);

                let final_color = Rgb([
                    (r.clamp(0.0, 0.999) * 256.0) as u8,
                    (g.clamp(0.0, 0.999) * 256.0) as u8,
                    (b.clamp(0.0, 0.999) * 256.0) as u8,
                ]);

                image_buffer.put_pixel(px, py, final_color);

                pb.inc(1);
            }
        }

        pb.finish_with_message("Render complete!");
        image_buffer
    }

    fn trace_ray(&self, ray: &crate::math::ray::Ray, scene: &Scene) -> Vec3<f64> {
        const T_MIN: f64 = 0.001;
        const T_MAX: f64 = f64::INFINITY;

        let closest_hit = scene
            .hittables
            .iter()
            .filter_map(|hittable| hittable.hit(ray, T_MIN, T_MAX))
            .min_by(|a, b| a.t.total_cmp(&b.t));

        if let Some(hit) = closest_hit {
            return hit.material.shade(&hit, scene);
        }

        Vec3::new(
            scene.background_color[0] as f64 / 255.0,
            scene.background_color[1] as f64 / 255.0,
            scene.background_color[2] as f64 / 255.0,
        )
    }
}
