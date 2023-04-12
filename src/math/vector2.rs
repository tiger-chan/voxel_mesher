use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, PartialEq, PartialOrd)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}

impl<T> Clone for Vec2<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Vec2::new(self.x.clone(), self.y.clone())
    }
}

impl<T> Add for Vec2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> Sub for Vec2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> Mul for Vec2<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        Vec2::new(self.x * rhs.x, self.y * rhs.y)
    }
}
