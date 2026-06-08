//! Pixel blending operations ("over" alpha compositing) for RGBAImage.

use crate::data::{RGBAColor, RGBAImage};

impl RGBAColor {
    /// Standard "over" alpha compositing: blend `self` (source / foreground)
    /// onto `dst` (destination / background), returning the resulting color.
    ///
    /// Uses straight (non-premultiplied) alpha with 16-bit intermediates
    /// to avoid overflow and preserve precision.
    pub fn blend_over(&self, dst: &RGBAColor) -> RGBAColor {
        let sa = self.a as u16;
        let da = dst.a as u16;

        // Fully opaque source → source wins
        if sa == 255 {
            return *self;
        }
        // Fully transparent source → destination wins
        if sa == 0 {
            return *dst;
        }
        // Fully transparent destination → source wins
        if da == 0 {
            return *self;
        }

        let inv_sa = 255 - sa;
        let out_a = sa + da * inv_sa / 255;
        // The result is at least as opaque as source, so out_a > 0

        let r = (self.r as u16 * sa + dst.r as u16 * da * inv_sa / 255) / out_a;
        let g = (self.g as u16 * sa + dst.g as u16 * da * inv_sa / 255) / out_a;
        let b = (self.b as u16 * sa + dst.b as u16 * da * inv_sa / 255) / out_a;

        RGBAColor {
            r: r as u8,
            g: g as u8,
            b: b as u8,
            a: out_a as u8,
        }
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
