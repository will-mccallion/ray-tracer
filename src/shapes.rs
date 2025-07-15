pub mod sphere;

use crate::vector::Vec3;
use image::Rgb;
use sphere::Sphere;

#[derive(Debug)]
pub enum Shapes {
    Spheres(Sphere),
}

pub trait Intersects {
    fn does_intersect(&self, ray_pos: &Vec3<f64>, ray_dir: &Vec3<f64>) -> Option<f64>;
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<f64> {
    if a == 0.0 {
        return None;
    }

    let discriminant = ((b * b) - (4.0 * a * c)).sqrt();
    let (t1, t2) = (
        (-b + discriminant) / (2.0 * a),
        (-b - discriminant) / (2.0 * a),
    );

    if t1 < 0.0 && t2 < 0.0 {
        return None;
    }

    if t1 < 0.0 {
        return Some(t1);
    }

    Some(t2)
}

#[cfg(test)]
mod tests {
    use crate::shapes::solve_quadratic;

    #[test]
    fn test_solve_simple_quadratic() {
        assert!(
            solve_quadratic(1.0, 1.0, 1.0).is_some(),
            "Simple solve_quadratic"
        );
    }

    #[test]
    #[should_panic]
    fn test_a_is_0_quadratic() {
        assert!(
            solve_quadratic(0.0, 1.0, 1.0).is_some(),
            "Invalid a parameter"
        );
    }
}
