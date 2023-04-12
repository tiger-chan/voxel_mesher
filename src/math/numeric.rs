use std::ops::{Add, Sub};

pub trait Numeric: Default + PartialEq + PartialOrd + Copy + Clone + Add + Sub {}
