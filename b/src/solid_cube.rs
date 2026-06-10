//! A cube whose appearance is loaded from a 13×13 PNG file.

use std::ops::Deref;
use std::path::PathBuf;

use crate::color::RGBAColor;
use crate::cube::{Cube, CubeImage};

/// A cube backed by a PNG image file.
#[derive(Clone)]
pub struct SolidCube {
    pub src_path: PathBuf,
    image: CubeImage,
}

impl SolidCube {
    /// Load a 13×13 PNG as a [`SolidCube`].
    ///
    /// # Panics
    /// Panics if the image dimensions are not exactly 13×13.
    pub fn new(src_path: impl Into<PathBuf>) -> image::ImageResult<Self> {
        let path = src_path.into();
        let img = image::open(&path)?.to_rgba8();
        let (w, h) = img.dimensions();

        assert_eq!(w, 13, "SolidCube image width must be 13, got {w}");
        assert_eq!(h, 13, "SolidCube image height must be 13, got {h}");

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

impl Cube for SolidCube {
    fn show(&self) {
        let data: Vec<u8> = self
            .image
            .iter()
            .flat_map(|row| row.iter())
            .flat_map(|p| [p.r, p.g, p.b, p.a])
            .collect();

        let img =
            image::RgbaImage::from_raw(13, 13, data).expect("pixel buffer must match dimensions");

        let tmp_path = std::env::temp_dir().join("cube_preview.png");
        img.save(&tmp_path).expect("failed to save preview image");

        #[cfg(target_os = "macos")]
        std::process::Command::new("open")
            .arg(&tmp_path)
            .spawn()
            .expect("failed to open preview");
        #[cfg(target_os = "linux")]
        std::process::Command::new("xdg-open")
            .arg(&tmp_path)
            .spawn()
            .expect("failed to open preview");
        #[cfg(target_os = "windows")]
        std::process::Command::new("cmd")
            .args(["/c", "start", tmp_path.to_str().unwrap()])
            .spawn()
            .expect("failed to open preview");
    }
}
