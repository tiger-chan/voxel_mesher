use crate::{math::Vec3, Face};

#[allow(dead_code)]
pub const RIGHT_HAND_Y_UP: [Vec3<i32>; Face::SIZE] = [
    Vec3::new(1, 0, 0),  // Right,
    Vec3::new(0, 0, -1),  // Back,
    Vec3::new(0, 1, 0),  // Up,
    Vec3::new(-1, 0, 0), // Left,
    Vec3::new(0, 0, 1), // Front,
    Vec3::new(0, -1, 0), // Down,
];

#[allow(dead_code)]
pub const LEFT_HAND_Y_UP: [Vec3<i32>; Face::SIZE] = [
	Vec3::new(1, 0, 0),  // Right,
    Vec3::new(0, 0, 1),  // Back,
    Vec3::new(0, 1, 0),  // Up,
    Vec3::new(-1, 0, 0), // Left,
    Vec3::new(0, 0, -1), // Front,
    Vec3::new(0, -1, 0), // Down,
];
