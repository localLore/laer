#[derive(Clone, Copy)]
pub struct RGBAColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for RGBAColor {
    fn default() -> Self {
        RGBAColor {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
}

/// 固定尺寸的 RGBA 图像, 宽 W, 高 H
pub struct RGBAImage<const W: usize, const H: usize> {
    pub pixels: [[RGBAColor; W]; H],
}

impl<const W: usize, const H: usize> RGBAImage<W, H> {
    pub fn new(background_color: RGBAColor) -> Self {
        RGBAImage {
            pixels: [[background_color; W]; H],
        }
    }
}

impl<const W: usize, const H: usize> Default for RGBAImage<W, H> {
    fn default() -> Self {
        RGBAImage::new(RGBAColor::default())
    }
}
