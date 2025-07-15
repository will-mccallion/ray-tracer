use std::f64::INFINITY;

use crate::shapes::{Intersects, Shapes};
use crate::vector::Vec3;
use image::{Rgb, RgbImage};

const T_MIN: f64 = 0.0001;
const SHADOW_EPSILON: f64 = 0.001;

struct LightPoint {
    pub position: Vec3<f64>,
    pub intensity: f64,
}

impl LightPoint {
    pub fn new(position: Vec3<f64>, intensity: f64) -> Self {
        LightPoint {
            position,
            intensity,
        }
    }
}

pub struct Image {
    image_buffer: RgbImage,
    camera_pos: Vec3<f64>,
    lights: Vec<LightPoint>,
    shapes: Vec<Shapes>,
    viewport_center: Vec3<f64>,
    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,
    background_color: Rgb<u8>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        let aspect_ratio = width as f64 / height as f64;

        let camera_pos = Vec3::new(0.0, 0.0, 0.0);
        let viewport_center = Vec3::new(0.0, 0.0, -1.0);
        let focal_length = (camera_pos - viewport_center).length();

        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;

        Self {
            image_buffer: RgbImage::new(width, height),
            camera_pos,
            lights: vec![LightPoint::new(Vec3::new(0.0, 10.0, -10.0), 1.0)],
            shapes: vec![],
            viewport_center,
            focal_length,
            viewport_height,
            viewport_width,
            background_color: Rgb([0, 0, 0]),
        }
    }

    pub fn change_background_colour(&mut self, colour: Rgb<u8>) {
        self.background_color = colour;
    }

    pub fn add_shape(&mut self, shape: Shapes) {
        self.shapes.push(shape);
    }

    pub fn draw_image(&mut self) {
        let width = self.image_buffer.width();
        let height = self.image_buffer.height();
        let i_width = width as isize;
        let i_height = height as isize;

        let ratio = self.viewport_height / height as f64;

        let primary_light = self
            .lights
            .get(0)
            .expect("Scene requires at least one light");

        for px in 0..width {
            for py in 0..height {
                let x = px as isize - i_width / 2;
                let y = i_height / 2 - py as isize - 1;

                let ray_finish =
                    Vec3::new(x as f64 * ratio, y as f64 * ratio, self.viewport_center.z);
                let ray_dir = (ray_finish - self.camera_pos).normalize();

                let closest_hit = self
                    .shapes
                    .iter()
                    .enumerate()
                    .filter_map(|(index, shape)| {
                        let intersection_t = match shape {
                            Shapes::Spheres(sphere_data) => {
                                sphere_data.does_intersect(&self.camera_pos, &ray_dir)
                            }
                        };
                        intersection_t.map(|t| (index, t))
                    })
                    .filter(|&(_index, t)| t > T_MIN)
                    .min_by(|&(_, t1), &(_, t2)| t1.total_cmp(&t2));

                let colour = match closest_hit {
                    None => self.background_color,
                    Some((hit_index, t)) => {
                        let hit_point = self.camera_pos + ray_dir * t;
                        let hit_object = &self.shapes[hit_index];

                        let surface_normal = match hit_object {
                            Shapes::Spheres(sphere_data) => sphere_data.get_normal(&hit_point),
                        };

                        let base_colour = match hit_object {
                            Shapes::Spheres(sphere_data) => sphere_data.get_colour(),
                        };

                        let light_dir = (primary_light.position - hit_point).normalize();
                        let dist_to_light = (primary_light.position - hit_point).length();

                        let shadow_ray_origin = hit_point + surface_normal * SHADOW_EPSILON;

                        let in_shadow = self
                            .shapes
                            .iter()
                            .enumerate()
                            .filter_map(|(_index, shape)| match shape {
                                Shapes::Spheres(sphere_data) => {
                                    sphere_data.does_intersect(&shadow_ray_origin, &light_dir)
                                }
                            })
                            .any(|shadow_t| shadow_t > T_MIN && shadow_t < dist_to_light);

                        if in_shadow {
                            let shadow_factor = 0.2;
                            Rgb([
                                (base_colour[0] as f32 * shadow_factor) as u8,
                                (base_colour[1] as f32 * shadow_factor) as u8,
                                (base_colour[2] as f32 * shadow_factor) as u8,
                            ])
                        } else {
                            base_colour
                        }
                    }
                };

                self.image_buffer.put_pixel(px, py, colour);
            }
        }
    }

    pub fn create_image(&mut self, output_path: &str) -> Option<()> {
        println!("Creating image...");
        if !output_path.ends_with(".png") {
            println!("Invalid path.");
            let output = output_path.to_owned() + ".png";
            println!("Saving in {}", output);
            let _ = self.image_buffer.save(output);
            return Some(());
        }
        let _ = self.image_buffer.save(output_path).unwrap();
        println!("Output path: {}", output_path);
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_image() {
        let mut image = Image::new(100, 100);
        image.create_image("src/image.png").unwrap();
        let _ = std::fs::remove_file("src/image.png");
    }

    #[test]
    #[should_panic]
    fn create_image_invalid_path() {
        let mut image = Image::new(300, 300);
        image.create_image("src/image.pngdt").unwrap();
    }
}
