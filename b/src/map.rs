//! Indexed block map — a size-generic 3D grid of cube indices, plus a bridge
//! to convert into the existing [`CubeBlock`] render pipeline.

use crate::block::CubeBlock;
use crate::block_image::CubeBlockImage;
use crate::owned_cube::OwnedCube;
use crate::resource::ResourcePack;

/// N×N×N grid. Each cell is `None` (empty) or `Some(index)` into a [`ResourcePack`].
pub struct BlockMap<const N: usize> {
    cells: [[[Option<usize>; N]; N]; N],
}

impl<const N: usize> BlockMap<N> {
    pub fn new() -> Self {
        BlockMap {
            cells: [[[None; N]; N]; N],
        }
    }

    // ── cell access ──

    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<usize> {
        self.cells[z][y][x]
    }

    /// # Panics if coordinate ≥ N.
    pub fn place(&mut self, x: usize, y: usize, z: usize, index: usize) {
        assert!(x < N && y < N && z < N);
        self.cells[z][y][x] = Some(index);
    }

    /// Place by name. Returns the resolved index.
    ///
    /// # Panics if name not found or coordinate out of range.
    pub fn place_named(
        &mut self,
        x: usize,
        y: usize,
        z: usize,
        name: &str,
        pack: &ResourcePack,
    ) -> usize {
        let index = pack
            .index_of(name)
            .unwrap_or_else(|| panic!("cube not found in pack: {name}"));
        self.place(x, y, z, index);
        index
    }

    /// # Panics if coordinate ≥ N.
    pub fn remove(&mut self, x: usize, y: usize, z: usize) {
        assert!(x < N && y < N && z < N);
        self.cells[z][y][x] = None;
    }

    pub fn count(&self) -> usize {
        self.cells
            .iter()
            .flat_map(|layer| layer.iter())
            .flat_map(|row| row.iter())
            .filter(|c| c.is_some())
            .count()
    }

    /// Iterate over occupied cells: `(x, y, z, cube_index)`.
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, usize, usize)> + '_ {
        self.cells.iter().enumerate().flat_map(|(z, layer)| {
            layer.iter().enumerate().flat_map(move |(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(x, cell)| cell.map(|idx| (x, y, z, idx)))
            })
        })
    }

    // ── bridge ──

    /// Convert into a [`CubeBlock`] (16³), placing each occupied cell as an
    /// [`OwnedCube`] looked up from `pack`. Cubes go in the first N×N×N
    /// corner; after this you can `.render()`, `.show()`, `.save()`, etc.
    pub fn to_cube_block(&self, pack: &ResourcePack) -> CubeBlock {
        let mut block = CubeBlock::new();
        for (x, y, z, index) in self.iter() {
            if let Some(image) = pack.get(index) {
                let name = pack
                    .iter()
                    .find(|(i, _, _)| *i == index)
                    .map(|(_, n, _)| n.to_string())
                    .unwrap_or_else(|| format!("_{index}"));
                block.place_cube(x, y, z, Box::new(OwnedCube::new(name, *image)));
            }
        }
        block
    }

    pub fn render(&self, pack: &ResourcePack) -> CubeBlockImage {
        self.to_cube_block(pack).render_to_rgba_image()
    }
}

impl<const N: usize> Default for BlockMap<N> {
    fn default() -> Self {
        BlockMap::new()
    }
}
