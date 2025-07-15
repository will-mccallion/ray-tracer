use crate::shapes::Intersects;
use crate::vector::Vec3;
use image::Rgb;

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Vec3<f64>,
    radius: f64,
    colour: Rgb<u8>,
}

pub trait Shape {
    fn name(&self) -> &'static str;
}

impl Sphere {
    pub fn new(center: Vec3<f64>, radius: f64) -> Self {
        Self {
            center,
            radius,
            colour: Rgb([255, 255, 255]),
        }
    }

    pub fn get_normal(&self, point: &Vec3<f64>) -> Vec3<f64> {
        (*point - self.center).normalize()
    }

    pub fn change_colour(&mut self, colour: Rgb<u8>) {
        self.colour = colour;
    }

    pub fn get_colour(&self) -> Rgb<u8> {
        self.colour
    }
}

impl Intersects for Sphere {
    fn does_intersect(&self, ray_pos: &Vec3<f64>, ray_dir: &Vec3<f64>) -> Option<f64> {
        let v = *ray_pos - self.center;
        let a = ray_dir.dot(&ray_dir);
        let b = 2.0 * ray_dir.dot(&v);
        let c = v.dot(&v) - (self.radius * self.radius);
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        match super::solve_quadratic(a, b, c) {
            Some(t) => return Some(t),
            _ => return None,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersects_sphere() {
        let s1 = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 2.0);

        let ray_pos = Vec3::new(0.0, 0.0, 0.0);
        let ray_dir = Vec3::new(0.0, 0.0, -1.0);
        let x = s1.does_intersect(&ray_pos, &ray_dir);
        assert!(x.is_some(), "Incorrect sphere miss");
    }

    #[test]
    #[should_panic]
    fn not_intersecting_sphere() {
        let s1 = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 2.0);

        let ray_pos = Vec3::new(0.0, 0.0, 0.0);
        let ray_dir = Vec3::new(0.0, 0.0, 1.0);
        let x = s1.does_intersect(&ray_pos, &ray_dir);
        assert!(x.is_some(), "Incorrect sphere intersection");
    }
}
