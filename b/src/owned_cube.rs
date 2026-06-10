//! A simple owned [`Cube`] implementation that holds a [`CubeImage`] in memory.
//!
//! This is the bridge between data-oriented representations (resource packs,
//! indexed maps) and the trait-based [`CubeBlock`] render pipeline.

use std::ops::Deref;

use crate::color::RGBAColor;
use crate::cube::{Cube, CubeImage};

/// A cube backed by an owned 13×13 image buffer and an optional display name.
pub struct OwnedCube {
    pub name: String,
    image: CubeImage,
}

impl OwnedCube {
    /// Create a cube from a name and its 13×13 pixel data.
    pub fn new(name: impl Into<String>, image: CubeImage) -> Self {
        OwnedCube {
            name: name.into(),
            image,
        }
    }
}

impl Deref for OwnedCube {
    type Target = CubeImage;

    fn deref(&self) -> &Self::Target {
        &self.image
    }
}

impl Cube for OwnedCube {
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
            .ok();
        #[cfg(target_os = "linux")]
        std::process::Command::new("xdg-open")
            .arg(&tmp_path)
            .spawn()
            .ok();
        #[cfg(target_os = "windows")]
        std::process::Command::new("cmd")
            .args(["/c", "start", tmp_path.to_str().unwrap()])
            .spawn()
            .ok();
    }
}

// ── Re-exported convenience constructors ──

impl OwnedCube {
    /// Create a solid-color cube.
    pub fn solid(name: impl Into<String>, color: RGBAColor) -> Self {
        OwnedCube::new(name, [[color; 13]; 13])
    }
}
