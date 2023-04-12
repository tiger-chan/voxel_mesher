mod complex;
mod culling;

use super::face::*;
use super::math::*;
use super::quad::*;
use super::voxel::Voxel;

pub use culling::Culling;

pub struct MesherResult {
    pub quads: Vec<Quad>,
}

pub trait Mesher {
    fn eval_append_border<V>(&self, volume: &[V]) -> Option<MesherResult>
    where
        V: Voxel + Clone;
    fn eval(&self, volume: &[impl Voxel]) -> Option<MesherResult>;
}

impl<V> Voxel for Option<V>
where
    V: Voxel,
{
    type Id = V::Id;
    fn id(&self) -> Self::Id {
        match self {
            None => panic!("Cannot retrive id from None"),
            Some(v) => v.id(),
        }
    }

    fn visibility(&self) -> crate::Visibility {
        match self {
            None => crate::Visibility::Hidden,
            Some(v) => v.visibility(),
        }
    }
}
