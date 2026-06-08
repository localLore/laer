//! RGBA color value — the fundamental pixel type used throughout the crate.

/// A single RGBA pixel with 8 bits per channel.
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

impl RGBAColor {
    /// Standard "over" alpha compositing: blend `self` (source / foreground)
    /// onto `dst` (destination / background), returning the resulting color.
    ///
    /// Uses straight (non-premultiplied) alpha with 16-bit intermediates
    /// to avoid overflow and preserve precision.
    pub fn blend_over(&self, dst: &RGBAColor) -> RGBAColor {
        let sa = self.a as u16;
        let da = dst.a as u16;

        // Fast paths for common edge cases.
        if sa == 255 {
            return *self;
        }
        if sa == 0 {
            return *dst;
        }
        if da == 0 {
            return *self;
        }

        let inv_sa = 255 - sa;
        let out_a = sa + da * inv_sa / 255;

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
