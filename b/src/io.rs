use std::path::Path;

use image::RgbaImage as ImgBuf;

use crate::concept::CubeBlockImage;
use crate::data::{RGBAColor, RGBAImage};

impl RGBAImage {
    /// 从文件加载 RGBA 图像
    pub fn load(path: impl AsRef<Path>) -> image::ImageResult<Self> {
        let img = image::open(path)?.to_rgba8();
        let (w, h) = img.dimensions();
        let mut pixels = Vec::with_capacity(h as usize);
        for y in 0..h {
            let mut row = Vec::with_capacity(w as usize);
            for x in 0..w {
                let p = img.get_pixel(x, y);
                row.push(RGBAColor {
                    r: p[0],
                    g: p[1],
                    b: p[2],
                    a: p[3],
                });
            }
            pixels.push(row);
        }
        Ok(RGBAImage { pixels })
    }

    /// 导出到文件（格式由扩展名自动决定：.png / .jpg / .bmp / .tiff 等）
    pub fn save(&self, path: impl AsRef<Path>) -> image::ImageResult<()> {
        let w = self.pixels[0].len() as u32;
        let h = self.pixels.len() as u32;
        let data: Vec<u8> = self
            .pixels
            .iter()
            .flat_map(|row| row.iter())
            .flat_map(|p| [p.r, p.g, p.b, p.a])
            .collect();
        let buf = ImgBuf::from_raw(w, h, data).expect("pixel buffer must match dimensions");
        buf.save(path)
    }
}

impl CubeBlockImage {
    /// 导出到文件，委托给内部 RGBAImage
    pub fn save(&self, path: impl AsRef<Path>) -> image::ImageResult<()> {
        self.image.save(path)
    }
}
