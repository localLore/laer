//! A 16×16×16 grid of cubes — the core world data structure.

use crate::block_image::CubeBlockImage;
use crate::cube::{Cube, CubeImage};

/// An optional cube slot: `None` means empty, `Some(boxed cube)` means occupied.
pub type CubeSlot = Option<Box<dyn Cube<Target = CubeImage>>>;

/// A 16×16×16 grid of optional cube slots.
pub struct CubeBlock {
    pub cubes: [[[CubeSlot; 16]; 16]; 16],
}

impl CubeBlock {
    /// Create an empty block with all slots set to `None`.
    pub fn new() -> Self {
        CubeBlock {
            cubes: std::array::from_fn(|_| std::array::from_fn(|_| std::array::from_fn(|_| None))),
        }
    }

    /// Place a cube at the given coordinates.
    ///
    /// # Panics
    /// Panics if any coordinate is ≥ 16.
    pub fn place(&mut self, x: usize, y: usize, z: usize, cube: Box<dyn Cube<Target = CubeImage>>) {
        assert!(x < 16, "x out of range: {x}");
        assert!(y < 16, "y out of range: {y}");
        assert!(z < 16, "z out of range: {z}");
        self.cubes[x][y][z] = Some(cube);
    }

    /// Remove the cube at the given coordinates, returning it if present.
    ///
    /// # Panics
    /// Panics if any coordinate is ≥ 16.
    pub fn remove(&mut self, x: usize, y: usize, z: usize) -> CubeSlot {
        assert!(x < 16, "x out of range: {x}");
        assert!(y < 16, "y out of range: {y}");
        assert!(z < 16, "z out of range: {z}");
        self.cubes[x][y][z].take()
    }

    /// Get a shared reference to the cube at the given coordinates.
    ///
    /// # Panics
    /// Panics if any coordinate is ≥ 16.
    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<&dyn Cube<Target = CubeImage>> {
        assert!(x < 16, "x out of range: {x}");
        assert!(y < 16, "y out of range: {y}");
        assert!(z < 16, "z out of range: {z}");
        self.cubes[x][y][z].as_deref()
    }

    /// Get a mutable reference to the cube at the given coordinates.
    ///
    /// # Panics
    /// Panics if any coordinate is ≥ 16.
    pub fn get_mut(
        &mut self,
        x: usize,
        y: usize,
        z: usize,
    ) -> Option<&mut dyn Cube<Target = CubeImage>> {
        assert!(x < 16, "x out of range: {x}");
        assert!(y < 16, "y out of range: {y}");
        assert!(z < 16, "z out of range: {z}");
        if let Some(ref mut b) = self.cubes[x][y][z] {
            Some(b.as_mut())
        } else {
            None
        }
    }

    /// Check whether the given coordinates are occupied.
    ///
    /// # Panics
    /// Panics if any coordinate is ≥ 16.
    pub fn occupied(&self, x: usize, y: usize, z: usize) -> bool {
        assert!(x < 16, "x out of range: {x}");
        assert!(y < 16, "y out of range: {y}");
        assert!(z < 16, "z out of range: {z}");
        self.cubes[x][y][z].is_some()
    }

    /// Remove all cubes from the block.
    pub fn clear(&mut self) {
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    self.cubes[x][y][z] = None;
                }
            }
        }
    }

    /// Return the number of occupied slots.
    pub fn count(&self) -> usize {
        self.cubes
            .iter()
            .flat_map(|layer| layer.iter())
            .flat_map(|column| column.iter())
            .filter(|slot| slot.is_some())
            .count()
    }

    /// Return `true` if the block contains no cubes.
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Render the block into a 193×193 isometric image.
    ///
    /// Cube coordinates map to screen pixels as follows:
    ///
    /// ```text
    /// x+1 → left-down:   screen_x - 6, screen_y + 3
    /// y+1 → right-down:  screen_x + 6, screen_y + 3
    /// z+1 → up:          screen_y - 8
    /// ```
    ///
    /// Cube (0,0,0) anchors at the canvas top-left.
    pub fn render(&self) -> CubeBlockImage {
        let mut image = CubeBlockImage::default();

        let offset_x: isize = 90;
        let offset_y: isize = 120;

        for z in 0..16 {
            for y in 0..16 {
                for x in 0..16 {
                    if let Some(cube) = self.get(x, y, z) {
                        let cube_pixels: &CubeImage = cube.deref();

                        let screen_x = offset_x - (x as isize * 6) + (y as isize * 6);
                        let screen_y =
                            offset_y + (x as isize * 3) + (y as isize * 3) - (z as isize * 8);

                        for py in 0..13 {
                            for px in 0..13 {
                                let sx = screen_x + px as isize;
                                let sy = screen_y + py as isize;
                                image.image.blend_pixel(sx, sy, &cube_pixels[py][px]);
                            }
                        }
                    }
                }
            }
        }

        image
    }
}

impl Default for CubeBlock {
    fn default() -> Self {
        CubeBlock::new()
    }
}
