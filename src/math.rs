mod numeric;
mod vector2;
mod vector3;

use std::ops::{Sub, Add, Mul};

pub use vector2::Vec2;
pub use vector3::Vec3;
pub fn lerp<T>(a: T, b: T, alpha: T) -> T
where
	T: Sub<Output = T> + Add<Output = T> + Mul<Output = T> + Clone {
	let c = b - a.clone();
	a + (c * alpha)
}