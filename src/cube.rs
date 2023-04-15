use crate::math::*;

use super::{face::Face, quad::*};

fn configure_quad(q: &mut Quad, uvs: [Vec2<f32>; 4]) {
    let cb = q.points[2] - q.points[1];
    let ab = q.points[0] - q.points[1];
    let normal = cb.cross(ab);

    q.normal = normal;
    q.uv = uvs;
}

#[cfg(feature = "top-left-uv")]
fn gen_uvs() -> [Vec2<f32>; 4] {
	[
		Vec2::new(0.0, 0.0), // UV Bottom Left
		Vec2::new(1.0, 0.0), // UV Bottom Right
		Vec2::new(1.0, 1.0), // UV Top Right
		Vec2::new(0.0, 1.0), // UV Top Left
	]
}

#[cfg(feature = "bottom-left-uv")]
fn gen_uvs() -> [Vec2<f32>; 4] {
	[
		Vec2::new(0.0, 1.0), // UV Bottom Left
		Vec2::new(1.0, 1.0), // UV Bottom Right
		Vec2::new(1.0, 0.0), // UV Top Right
		Vec2::new(0.0, 0.0), // UV Top Left
	]
}

pub fn unit_cube(faces: &[Vec3<i32>; Face::SIZE]) -> [Quad; Face::SIZE] {
    // Update and normalize in to the positive space
    let unit_dir: [Vec3<f32>; Face::SIZE] = faces
        .map(|d| Vec3::new(d.x + 1, d.y + 1, d.z + 1) / 2)
        .map(|x| Vec3::from(x));

    //println!("Unit Directions {unit_dir:?}");

    const RIGHT: usize = Face::Right as usize;
    const BACK: usize = Face::Back as usize;
    const UP: usize = Face::Up as usize;
    const LEFT: usize = Face::Left as usize;
    const FRONT: usize = Face::Front as usize;
    const DOWN: usize = Face::Down as usize;

    let a = unit_dir[LEFT] + unit_dir[DOWN] + unit_dir[FRONT];
    let b = unit_dir[RIGHT] + unit_dir[DOWN] + unit_dir[FRONT];
    let c = unit_dir[LEFT] + unit_dir[UP] + unit_dir[FRONT];
    let d = unit_dir[RIGHT] + unit_dir[UP] + unit_dir[FRONT];

    let e = unit_dir[LEFT] + unit_dir[DOWN] + unit_dir[BACK];
    let f = unit_dir[RIGHT] + unit_dir[DOWN] + unit_dir[BACK];
    let g = unit_dir[LEFT] + unit_dir[UP] + unit_dir[BACK];
    let z = unit_dir[RIGHT] + unit_dir[UP] + unit_dir[BACK];

    let mut quads: [Quad; Face::SIZE] = [
        Quad::new(d, z, f, b), // Right face
        Quad::new(z, g, e, f), // Back face
        Quad::new(g, z, d, c), // Top face
        Quad::new(g, c, a, e), // Left face
        Quad::new(c, d, b, a), // Front face
        Quad::new(f, e, a, b), // Bottom face
    ];

    let uvs = gen_uvs();

    for q in quads.iter_mut() {
        configure_quad(q, uvs.clone());
    }

    quads
}

#[cfg(test)]
mod tests {
    use crate::RIGHT_HAND_Y_UP;

    use super::*;

    #[test]
    fn generate_cube() {
        let result = unit_cube(&RIGHT_HAND_Y_UP);
        assert_eq!(result.len(), 6);
        assert_eq!(result[0].normal, Vec3::new(1.0, 0.0, 0.0));
    }
}
