use crate::cube::unit_cube;
use crate::math;

use super::*;
use super::{Face, Vec3};

/// ## Quads per Face
///
/// In this voxel meshing strategy, each voxel can contain 1 quad per face. Quads are four-sided polygons commonly used in computer graphics.
///
/// ## Culling
///
/// culling will occur on a per face basis.
///
pub struct Culling {
    pub width: usize,
    pub height: usize,
    pub depth: usize,

    pub faces: [Vec3<i32>; Face::SIZE],
}

impl Culling {
    pub fn new(w: usize, h: usize, d: usize, faces: [Vec3<i32>; Face::SIZE]) -> Self {
        Culling {
            width: w,
            height: h,
            depth: d,
            faces: faces,
        }
    }

    fn is_in_bounds(&self, x: i32, y: i32, z: i32) -> bool {
        1.max(x).min(self.width as i32) == x
            && 1.max(y).min(self.height as i32) == y
            && 1.max(z).min(self.depth as i32) == z
    }

    /// Finds the visibility state of each face of a voxel at the given coordinates. It returns two arrays: one indicating whether each face is within the bounds of the voxel volume, and another indicating whether each face is visible (based on the visibility state of the neighboring voxels).
    fn find_bounds(
        &self,
        volume: &[impl Voxel],
        x: usize,
        y: usize,
        z: usize,
    ) -> ([bool; Face::SIZE], [bool; Face::SIZE]) {
        let vert = Vec3::new(x.clone() as i32, y.clone() as i32, z.clone() as i32);
        let verts = [
            vert + self.faces[Face::Right as usize],
            vert + self.faces[Face::Back as usize],
            vert + self.faces[Face::Up as usize],
            vert + self.faces[Face::Left as usize],
            vert + self.faces[Face::Front as usize],
            vert + self.faces[Face::Down as usize],
        ];

        let bound_w: usize = self.width + 2;
        let bound_h: usize = self.height + 2;
        let mut vert_in_bounds = [false; Face::SIZE];
        let mut vert_state = [false; Face::SIZE];

        for i in 0..Face::SIZE {
            let v = verts[i];
            let id = v.x as usize + v.y as usize * bound_w + v.z as usize * (bound_w * bound_h);

            vert_in_bounds[i] = self.is_in_bounds(v.x, v.y, v.z);
            vert_state[i] = volume[id].visibility().is_visible();
        }

        (vert_in_bounds, vert_state)
    }
}

impl Mesher for Culling {
    fn eval_append_border<V>(&self, volume: &[V]) -> Option<MesherResult>
    where
        V: Voxel + Clone,
    {
        let border_w = self.width + 2;
        let border_h = self.height + 2;
        let border_d = self.depth + 2;

        let mut extended_volume: Vec<Option<V>> = vec![None; border_w * border_h * border_d];

        for z in 0..self.depth {
            for y in 0..self.height {
                for x in 0..self.width {
                    let idx_new = (z + 1) * border_w * border_h + (y + 1) * border_w + (x + 1);
                    let idx_org = z * self.width * self.height + y * self.width + x;
                    extended_volume[idx_new] = Some(volume[idx_org].clone());
                }
            }
        }

        self.eval(&extended_volume)
    }

    fn eval(&self, volume: &[impl Voxel]) -> Option<MesherResult> {
        if self.width == 0 || self.height == 0 || self.depth == 0 {
            None
        } else {
            let border_w = self.width + 2;
            let border_h = self.height + 2;
            let border_d = self.depth + 2;

            let cube_faces = unit_cube(&self.faces);

            let mut result: MesherResult = MesherResult {
                quads: Vec::with_capacity(self.width * self.height * self.depth * 6),
            };

            for z in 1..(border_d - 1) {
                for y in 0..border_h {
                    for x in 0..border_w {
                        if !self.is_in_bounds(x as i32, y as i32, z as i32) {
                            continue;
                        }

                        let idx = z * border_h * border_w + y * border_w + x;
                        let voxel = &volume[idx];
                        let state = voxel.visibility().is_visible();
                        //println!("state[{x}, {y}, {z}]: {state}");

                        if !state {
                            continue;
                        }

                        // Remove the border from the coordinates
                        let vert = Vec3::new((x - 1) as f32, (y - 1) as f32, (z - 1) as f32);

                        let (_, volume_state) = self.find_bounds(volume, x, y, z);

                        for face_id in 0..Face::SIZE {
                            if state == volume_state[face_id] {
                                // No change in state means there is no need to draw a quad
                                continue;
                            }

                            let mut face = cube_faces[face_id].clone();

                            for p in face.points.iter_mut() {
                                *p = *p + vert;
                            }

                            let uv_space = [
                                math::lerp(face.uv[0], face.uv[2], Vec2::new(0.0, 0.0)),
                                math::lerp(face.uv[0], face.uv[2], Vec2::new(1.0, 1.0)),
                            ];
                            for uv in face.uv.iter_mut() {
                                *uv = math::lerp(uv_space[0], uv_space[1], *uv);
                            }

                            result.quads.push(face);
                        }
                    }
                }
            }

            Some(result)
        }
    }
}
