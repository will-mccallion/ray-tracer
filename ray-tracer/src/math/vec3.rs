use num_traits::{Float, Zero};
use serde::Deserialize;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }
}

impl<T> Vec3<T>
where
    T: Float,
{
    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        if length.is_zero() {
            Vec3::new(T::zero(), T::zero(), T::zero())
        } else {
            Vec3::new(self.x / length, self.y / length, self.z / length)
        }
    }
}

impl<T, E> Vec3<T>
where
    T: Mul<Output = E> + Copy,
    E: Add<Output = E>,
{
    pub fn dot(&self, other: &Self) -> E {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
}

impl<T> Vec3<T>
where
    T: Mul<Output = T> + Sub<Output = T> + Copy,
{
    pub fn cross(self, other: &Self) -> Self {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> Mul<Vec3<T>> for Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T, E> Div<E> for Vec3<T>
where
    T: Div<E, Output = T> + Copy,
    E: Copy + Zero + PartialEq,
{
    type Output = Self;

    fn div(self, rhs: E) -> Self::Output {
        if rhs.is_zero() {
            panic!("Attempting to divide by 0");
        }
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let v1 = Vec3::new(0.0, 1.0, 2.0);
        let v2 = Vec3::new(0.0, 1.0, 2.0);
        let v3 = v1 + v2;
        assert_eq!(v3, Vec3::new(0.0, 2.0, 4.0));
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(0.0, 1.0, 2.0);
        let v3 = v1 - v2;
        assert_eq!(v3, Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_scalar_mult() {
        let v1 = Vec3::new(2.0, 3.0, 4.0);
        let v2 = v1 * 2.0;
        assert_eq!(v2, Vec3::new(4.0, 6.0, 8.0));
    }

    #[test]
    fn test_vec_mult() {
        let v1 = Vec3::new(2.0, 3.0, 4.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        let v3 = v1 * v2;
        assert_eq!(v3, Vec3::new(4.0, 9.0, 16.0));
    }

    #[test]
    fn test_div() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = v1 / 2.0;
        assert_eq!(v2, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    #[should_panic]
    fn test_div_by_0() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let _v2 = v1 / 0.0;
    }

    #[test]
    fn test_normalize() {
        let v1 = Vec3::new(1.0, 1.0, 1.0);
        let v2 = v1.normalize();
        let v3 = Vec3::new(1.0 / 3.0.sqrt(), 1.0 / 3.0.sqrt(), 1.0 / 3.0.sqrt());
        assert_eq!(v3, v2);
    }
}
