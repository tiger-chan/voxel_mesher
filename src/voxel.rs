#[allow(dead_code)]
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum Visibility {
    Hidden,
    Visible,
}

impl Visibility {
    pub fn is_visible(&self) -> bool {
        matches!(self, Visibility::Visible)
    }
}

pub trait Voxel {
    type Id: Eq;
    fn id(&self) -> Self::Id;

    fn visibility(&self) -> Visibility;
}