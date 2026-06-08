use crate::data::{RGBAColor, RGBAImage};

pub trait Cube {
    fn show(&self);
}

pub struct CubeBlock {
    cubes: Box<[[[Option<Box<dyn Cube>>; 16]; 16]; 16]>,
}

pub struct CubeBlockImage {
    pub image: RGBAImage,
}

impl CubeBlockImage {
    pub fn new(background_color: RGBAColor) -> Self {
        CubeBlockImage {
            image: RGBAImage::new(49, 49, background_color),
        }
    }
}

impl Default for CubeBlockImage {
    fn default() -> Self {
        CubeBlockImage::new(RGBAColor::default())
    }
}
