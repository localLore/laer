//! File I/O for RGBAImage: load from / save to disk.

use std::path::Path;

use image::RgbaImage as ImgBuf;

use crate::color::RGBAColor;
use crate::image::RGBAImage;

impl<const W: usize, const H: usize> RGBAImage<W, H> {
    /// Load an RGBA image from file. Dimensions must match `W`×`H` exactly.
    ///
    /// # Panics
    /// Panics if the image dimensions don't match.
    pub fn load(path: impl AsRef<Path>) -> image::ImageResult<Self> {
        let img = image::open(path)?.to_rgba8();
        let (w, h) = img.dimensions();

        assert_eq!(w as usize, W, "image width must be {W}, got {w}");
        assert_eq!(h as usize, H, "image height must be {H}, got {h}");

        let mut pixels = [[RGBAColor::default(); W]; H];
        for y in 0..H {
            for x in 0..W {
                let p = img.get_pixel(x as u32, y as u32);
                pixels[y][x] = RGBAColor {
                    r: p[0],
                    g: p[1],
                    b: p[2],
                    a: p[3],
                };
            }
        }
        Ok(RGBAImage { pixels })
    }

    /// Save the image to file. Format is inferred from the extension
    /// (`.png`, `.jpg`, `.bmp`, `.tiff`, etc.).
    pub fn save(&self, path: impl AsRef<Path>) -> image::ImageResult<()> {
        let data: Vec<u8> = self
            .pixels
            .iter()
            .flat_map(|row| row.iter())
            .flat_map(|p| [p.r, p.g, p.b, p.a])
            .collect();
        let buf =
            ImgBuf::from_raw(W as u32, H as u32, data).expect("pixel buffer must match dimensions");
        buf.save(path)
    }
}
