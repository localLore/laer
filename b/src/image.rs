//! Fixed-size RGBA image with pixel-level operations.

use crate::color::RGBAColor;

/// A fixed-size RGBA image with width `W` and height `H`.
///
/// Pixels are stored in row-major order: `pixels[y][x]`.
pub struct RGBAImage<const W: usize, const H: usize> {
    pub pixels: [[RGBAColor; W]; H],
}

impl<const W: usize, const H: usize> RGBAImage<W, H> {
    /// Create an image filled with `background_color`.
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

impl<const W: usize, const H: usize> RGBAImage<W, H> {
    /// Blend a source pixel at screen coordinates `(x, y)` using the "over" operator.
    ///
    /// Silently does nothing when:
    /// - coordinates are out of bounds, or
    /// - `src` is fully transparent.
    pub fn blend_pixel(&mut self, x: isize, y: isize, src: &RGBAColor) {
        if x < 0 || y < 0 {
            return;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= W || y >= H || src.a == 0 {
            return;
        }
        let blended = src.blend_over(&self.pixels[y][x]);
        self.pixels[y][x] = blended;
    }
}
