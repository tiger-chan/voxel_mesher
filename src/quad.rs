use crate::math::{Vec2, Vec3};

const QUAD_SIZE: usize = 4;

#[derive(Debug, Clone, Copy)]
pub struct Quad {
    pub points: [Vec3<f32>; QUAD_SIZE],
    pub normal: Vec3<f32>,
    pub uv: [Vec2<f32>; 4],
}

impl Quad {
    pub fn new(a: Vec3<f32>, b: Vec3<f32>, c: Vec3<f32>, d: Vec3<f32>) -> Self {
        Quad {
            points: [a, b, c, d],
            normal: Vec3::new(0.0, 0.0, 0.0),
            uv: [Vec2::new(0.0, 0.0); 4],
        }
    }

    #[cfg(feature = "ccw-winding")]
    pub fn triangles(&self) -> [usize; 6] {
        [
            2, 1, 0, // first triangle
            3, 2, 0, // second triangle
        ]
    }

    #[cfg(feature = "cw-winding")]
    pub fn triangles(&self) -> [usize; 6] {
        [
            0, 1, 2, // first triangle
            0, 2, 3, // second triangle
        ]
    }
}
