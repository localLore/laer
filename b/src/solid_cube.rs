use std::{ops::Deref, path::PathBuf};

use crate::{concept::Cube, data::RGBAColor};

type CubeImage = [[RGBAColor; 13]; 13];

/// 图形构成的方块, 暂时从 PNG 文件加载
pub struct SolidCube {
    pub src_path: PathBuf,
    image: CubeImage, // 缓存加载后的图像数据
}

impl SolidCube {
    pub fn new(src_path: impl Into<PathBuf>) -> image::ImageResult<Self> {
        let path = src_path.into();
        let img = image::open(&path)?.to_rgba8();
        let (w, h) = img.dimensions();

        assert_eq!(w, 13, "SolidCube 图像宽度必须为 13");
        assert_eq!(h, 13, "SolidCube 图像高度必须为 13");

        let mut cube_image = [[RGBAColor::default(); 13]; 13];
        for y in 0..13 {
            for x in 0..13 {
                let p = img.get_pixel(x, y);
                cube_image[y as usize][x as usize] = RGBAColor {
                    r: p[0],
                    g: p[1],
                    b: p[2],
                    a: p[3],
                };
            }
        }

        Ok(SolidCube {
            src_path: path,
            image: cube_image,
        })
    }
}

impl Deref for SolidCube {
    type Target = CubeImage;

    fn deref(&self) -> &Self::Target {
        &self.image
    }
}

use std::process::Command;

impl Cube for SolidCube {
    fn show(&self) {
        // 将 13x13 图像数据转换为可保存的格式
        let data: Vec<u8> = self
            .image
            .iter()
            .flat_map(|row| row.iter())
            .flat_map(|p| [p.r, p.g, p.b, p.a])
            .collect();

        let img =
            image::RgbaImage::from_raw(13, 13, data).expect("pixel buffer must match dimensions");

        // 保存到临时文件
        let tmp_path = std::env::temp_dir().join("cube_preview.png");
        img.save(&tmp_path).expect("无法保存预览图像");

        // 调用系统默认图片查看器
        #[cfg(target_os = "macos")]
        {
            Command::new("open")
                .arg(&tmp_path)
                .spawn()
                .expect("无法打开预览");
        }

        #[cfg(target_os = "linux")]
        {
            Command::new("xdg-open")
                .arg(&tmp_path)
                .spawn()
                .expect("无法打开预览");
        }

        #[cfg(target_os = "windows")]
        {
            Command::new("cmd")
                .args(["/c", "start", tmp_path.to_str().unwrap()])
                .spawn()
                .expect("无法打开预览");
        }
    }
}
