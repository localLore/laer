//! A named collection of [`CubeImage`]s — the data-oriented counterpart of
//! the [`Cube`] trait hierarchy.
//!
//! # Two ways to build
//!
//! 1. **From a sprite sheet** — slice a PNG grid:
//!    ```ignore
//!    let pack = ResourcePack::from_sprite_sheet("sprites.png", 16, &[
//!        "red_cube", "blue_cube", ...
//!    ])?;
//!    ```
//!
//! 2. **Imperatively** — add cubes one by one:
//!    ```ignore
//!    let mut pack = ResourcePack::new();
//!    pack.add("red_cube", red_pixels);
//!    pack.add_solid("invisible", RGBAColor::default());
//!    ```
//!
//! [`Cube`]: crate::cube::Cube

use std::collections::HashMap;
use std::path::Path;

use crate::color::RGBAColor;
use crate::cube::CubeImage;

/// A collection of 13×13 cube images, indexed by name and position.
pub struct ResourcePack {
    cubes: Vec<CubeImage>,
    name_to_index: HashMap<String, usize>,
}

impl ResourcePack {
    // ── lookup ──

    /// Number of cubes in the pack.
    pub fn len(&self) -> usize {
        self.cubes.len()
    }

    /// Get a cube by its integer index.
    pub fn get(&self, index: usize) -> Option<&CubeImage> {
        self.cubes.get(index)
    }

    /// Get a cube by name.
    pub fn get_named(&self, name: &str) -> Option<&CubeImage> {
        self.name_to_index
            .get(name)
            .and_then(|&i| self.cubes.get(i))
    }

    /// Look up the index for a name.
    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.name_to_index.get(name).copied()
    }

    /// Iterate over all (index, &name, &CubeImage) triples.
    pub fn iter(&self) -> impl Iterator<Item = (usize, &str, &CubeImage)> {
        self.cubes.iter().enumerate().filter_map(|(i, img)| {
            // Find the name for this index (reverse lookup).
            // Linear scan is fine since packs are small (~64 entries).
            self.name_to_index
                .iter()
                .find(|(_, v)| **v == i)
                .map(|(k, _)| (i, k.as_str(), img))
        })
    }

    // ── mutable builder ──

    /// Create an empty pack.
    pub fn new() -> Self {
        ResourcePack {
            cubes: Vec::new(),
            name_to_index: HashMap::new(),
        }
    }

    /// Add a named cube image. Returns its index.
    pub fn add(&mut self, name: impl Into<String>, image: CubeImage) -> usize {
        let name = name.into();
        let index = self.cubes.len();
        self.name_to_index.insert(name, index);
        self.cubes.push(image);
        index
    }

    /// Add a solid-color cube. Returns its index.
    pub fn add_solid(&mut self, name: impl Into<String>, color: RGBAColor) -> usize {
        self.add(name, [[color; 13]; 13])
    }

    // ── sprite sheet loader ──

    /// Load a sprite sheet and build a pack.
    ///
    /// The sheet is a PNG of `(columns * 16)` × `(rows * 16)` pixels. Each
    /// 16×16 cell yields a 13×13 cube from its top-left corner.
    ///
    /// `names` lists one name per cell in row-major order. Pass `""` to skip
    /// a cell.
    pub fn from_sprite_sheet(
        path: impl AsRef<Path>,
        columns: u32,
        names: &[&str],
    ) -> image::ImageResult<Self> {
        let img = image::open(path)?.to_rgba8();
        let (sheet_w, sheet_h) = img.dimensions();
        let cols = columns;
        let rows = sheet_h / 16;
        let cell_count = (cols * rows) as usize;

        assert!(
            names.len() >= cell_count,
            "names slice has {} entries but sprite sheet has {cell_count} cells",
            names.len()
        );

        let mut pack = ResourcePack::new();

        for i in 0..cell_count {
            let name = names[i];
            if name.is_empty() {
                continue;
            }
            let col = (i as u32) % cols;
            let row = (i as u32) / cols;
            let ox = col * 16;
            let oy = row * 16;

            let mut image = [[RGBAColor::default(); 13]; 13];
            for py in 0..13 {
                for px in 0..13 {
                    if ox + px >= sheet_w || oy + py >= sheet_h {
                        continue; // out of bounds → stays transparent
                    }
                    let p = img.get_pixel(ox + px, oy + py);
                    image[py as usize][px as usize] = RGBAColor {
                        r: p[0],
                        g: p[1],
                        b: p[2],
                        a: p[3],
                    };
                }
            }
            pack.add(name, image);
        }

        Ok(pack)
    }
}

impl Default for ResourcePack {
    fn default() -> Self {
        ResourcePack::new()
    }
}
