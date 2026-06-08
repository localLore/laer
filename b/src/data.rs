#[derive(Clone)]
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
pub struct RGBAImage {
    pub pixels: Vec<Vec<RGBAColor>>,
}

impl RGBAImage {
    pub fn new(x: usize, y: usize, background_color: RGBAColor) -> Self {
        RGBAImage {
            pixels: vec![vec![background_color; x]; y],
        }
    }
}

impl Default for RGBAImage {
    fn default() -> Self {
        RGBAImage::new(16, 16, RGBAColor::default())
    }
}
