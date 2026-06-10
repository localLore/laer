//! The rendered isometric image of a [`CubeBlock`], plus display/save helpers.
//!
//! [`CubeBlock`]: crate::block::CubeBlock

use std::path::Path;

use crate::color::RGBAColor;
use crate::image::RGBAImage;

/// A 193×193 image produced by rendering a [`CubeBlock`].
///
/// [`CubeBlock`]: crate::block::CubeBlock
pub struct CubeBlockImage {
    pub image: RGBAImage<193, 193>,
}

impl CubeBlockImage {
    /// Create a block image filled with `background_color`.
    pub fn new(background_color: RGBAColor) -> Self {
        CubeBlockImage {
            image: RGBAImage::new(background_color),
        }
    }
}

impl Default for CubeBlockImage {
    fn default() -> Self {
        CubeBlockImage::new(RGBAColor::default())
    }
}

impl CubeBlockImage {
    /// Save to file (delegates to [`RGBAImage::save`]).
    pub fn save_as_png_file(&self, path: impl AsRef<Path>) -> image::ImageResult<()> {
        self.image.save(path)
    }

    /// Open the block image in the system default image viewer.
    pub fn show_self(&self) {
        let tmp_path = std::env::temp_dir().join("cube_block_preview.png");
        self.save_as_png_file(&tmp_path)
            .expect("failed to save preview image");

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
