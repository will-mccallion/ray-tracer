use crate::math::vec3::Vec3;
use crate::scene::Scene;
use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;

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

        let pixels: Vec<(u32, u32)> = (0..scene.camera.height)
            .flat_map(|y| (0..scene.camera.width).map(move |x| (x, y)))
            .collect();

        let rendered_pixels: Vec<Rgb<u8>> = pixels
            .into_par_iter()
            .map(|(px, py)| {
                pb.inc(1);

                let mut rng = rand::thread_rng();
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

                let samples_sqrt = (self.samples_per_pixel as f64).sqrt() as u32;
                let samples_per_side = if samples_sqrt == 0 { 1 } else { samples_sqrt };

                for i in 0..samples_per_side {
                    for j in 0..samples_per_side {
                        let u = (px as f64
                            + (i as f64 + rng.r#gen::<f64>()) / samples_per_side as f64)
                            / (scene.camera.width - 1) as f64;

                        let v = (py as f64
                            + (j as f64 + rng.r#gen::<f64>()) / samples_per_side as f64)
                            / (scene.camera.height - 1) as f64;

                        let ray = scene.camera.get_ray(u, 1.0 - v);
                        pixel_color = pixel_color + self.trace_ray(&ray, scene);
                    }
                }

                let total_samples = (samples_per_side * samples_per_side) as f64;
                pixel_color = pixel_color / total_samples;

                // Gamma Correction
                let r = pixel_color.x.powf(0.5);
                let g = pixel_color.y.powf(0.5);
                let b = pixel_color.z.powf(0.5);

                Rgb([
                    (r.clamp(0.0, 0.999) * 256.0) as u8,
                    (g.clamp(0.0, 0.999) * 256.0) as u8,
                    (b.clamp(0.0, 0.999) * 256.0) as u8,
                ])
            })
            .collect();

        pb.finish_with_message("Parallel rendering complete. Writing to image...");

        for (i, pixel) in rendered_pixels.into_iter().enumerate() {
            let x = i as u32 % scene.camera.width;
            let y = i as u32 / scene.camera.width;
            image_buffer.put_pixel(x, y, pixel);
        }

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
