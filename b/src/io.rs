use image::RgbaImage as ImgBuf;
use std::path::Path;

use crate::concept::CubeBlockImage;
use crate::data::{RGBAColor, RGBAImage};

impl<const W: usize, const H: usize> RGBAImage<W, H> {
    /// 从文件加载 RGBA 图像，尺寸必须完全匹配
    pub fn load(path: impl AsRef<Path>) -> image::ImageResult<Self> {
        let img = image::open(path)?.to_rgba8();
        let (w, h) = img.dimensions();

        assert_eq!(w as usize, W, "图像宽度必须为 {W}");
        assert_eq!(h as usize, H, "图像高度必须为 {H}");

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

    /// 导出到文件（格式由扩展名自动决定：.png / .jpg / .bmp / .tiff 等）
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

impl CubeBlockImage {
    /// 导出到文件，委托给内部 RGBAImage
    pub fn save(&self, path: impl AsRef<Path>) -> image::ImageResult<()> {
        self.image.save(path)
    }
}
