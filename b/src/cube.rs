//! The `Cube` abstraction — any object that occupies one slot in a [`CubeBlock`].
//!
//! [`CubeBlock`]: crate::block::CubeBlock

use std::ops::Deref;

use crate::color::RGBAColor;

/// A 13×13 image representing one cube's appearance.
pub type CubeImage = [[RGBAColor; 13]; 13];

/// A cube is any type that derefs to a [`CubeImage`] and can be previewed.
pub trait Cube: Deref<Target = CubeImage> {
    /// Open the cube's image in the system default image viewer.
    fn show(&self);
}
