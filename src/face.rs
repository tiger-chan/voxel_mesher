#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum Face {
    Right,
    Back,
    Up,
    Left,
    Front,
    Down,
}

impl Face {
    pub const SIZE: usize = 6;
}
