use std::ops::{Add, Mul, Sub, Div};

#[derive(Debug, Copy, PartialEq, PartialOrd)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Clone for Vec3<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Vec3::new(self.x.clone(), self.y.clone(), self.z.clone())
    }
}

impl<T> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> Div<i32> for Vec3<T>
where
    T: Div<i32, Output = T>,
{
    type Output = Self;
    fn div(self, rhs: i32) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T> Div<f32> for Vec3<T>
where
    T: Div<f32, Output = T>,
{
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl From<Vec3<i32>> for Vec3<f32> {
	fn from(value: Vec3<i32>) -> Self {
		Vec3::new(
			value.x as f32,
			value.y as f32,
			value.z as f32,
		)
	}
}

impl<T> Vec3<T>
where
    T: Sub<Output = T> + Mul<Output = T> + Copy,
{
    pub fn cross(&self, b: Vec3<T>) -> Vec3<T> {
        let mut r: Vec3<T> = self.clone();
        r.x = (self.y * b.z) - (self.z * b.y);
        r.y = (self.x * b.z) - (self.z * b.x);
        r.z = (self.x * b.y) - (self.y * b.x);
        return r;
    }
}

impl<T> std::fmt::Display for Vec3<T>
where
	T: std::fmt::Display {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
	}
}