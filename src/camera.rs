use crate::math::{ray::Ray, vec3::Vec3};

pub struct Camera {
    origin: Vec3<f64>,
    lower_left_corner: Vec3<f64>,
    horizontal: Vec3<f64>,
    vertical: Vec3<f64>,
    pub width: u32,
    pub height: u32,
}

impl Camera {
    pub fn new(
        width: u32,
        height: u32,
        lookfrom: Vec3<f64>,
        lookat: Vec3<f64>,
        vup: Vec3<f64>,    // View Up vector, defines the camera's "roll"
        vfov_degrees: f64, // Vertical field-of-view in degrees
    ) -> Self {
        let aspect_ratio = width as f64 / height as f64;
        let theta = vfov_degrees.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;

        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            width,
            height,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction =
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin;
        Ray::new(self.origin, direction.normalize())
    }
}
