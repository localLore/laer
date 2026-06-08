//! 区块的实现：16x16x16 的方块集合及相关操作方法。

#[cfg(target_os = "macos")]
use std::process::Command;

use crate::concept::{Cube, CubeBlock, CubeBlockImage};
use crate::data::RGBAColor;

type CubeImage = [[RGBAColor; 13]; 13];
type CubeSlot = Option<Box<dyn Cube<Target = CubeImage>>>;

impl CubeBlock {
    /// 创建一个空的区块，所有槽位为 None
    pub fn new() -> Self {
        CubeBlock {
            cubes: std::array::from_fn(|_| std::array::from_fn(|_| std::array::from_fn(|_| None))),
        }
    }

    /// 在指定位置放置方块
    ///
    /// # Panics
    /// 如果坐标超出 0..16 范围
    pub fn place(&mut self, x: usize, y: usize, z: usize, cube: Box<dyn Cube<Target = CubeImage>>) {
        assert!(x < 16, "x 坐标超出范围: {x}");
        assert!(y < 16, "y 坐标超出范围: {y}");
        assert!(z < 16, "z 坐标超出范围: {z}");
        self.cubes[x][y][z] = Some(cube);
    }

    /// 移除指定位置的方块，返回被移除的方块（如果有）
    ///
    /// # Panics
    /// 如果坐标超出 0..16 范围
    pub fn remove(&mut self, x: usize, y: usize, z: usize) -> CubeSlot {
        assert!(x < 16, "x 坐标超出范围: {x}");
        assert!(y < 16, "y 坐标超出范围: {y}");
        assert!(z < 16, "z 坐标超出范围: {z}");
        self.cubes[x][y][z].take()
    }

    /// 获取指定位置方块的引用
    ///
    /// # Panics
    /// 如果坐标超出 0..16 范围
    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<&dyn Cube<Target = CubeImage>> {
        assert!(x < 16, "x 坐标超出范围: {x}");
        assert!(y < 16, "y 坐标超出范围: {y}");
        assert!(z < 16, "z 坐标超出范围: {z}");
        self.cubes[x][y][z].as_deref()
    }

    /// 获取指定位置方块的可变引用
    ///
    /// # Panics
    /// 如果坐标超出 0..16 范围
    pub fn get_mut(
        &mut self,
        x: usize,
        y: usize,
        z: usize,
    ) -> Option<&mut dyn Cube<Target = CubeImage>> {
        assert!(x < 16, "x 坐标超出范围: {x}");
        assert!(y < 16, "y 坐标超出范围: {y}");
        assert!(z < 16, "z 坐标超出范围: {z}");
        if let Some(ref mut b) = self.cubes[x][y][z] {
            Some(b.as_mut())
        } else {
            None
        }
    }

    /// 检查指定位置是否有方块
    ///
    /// # Panics
    /// 如果坐标超出 0..16 范围
    pub fn occupied(&self, x: usize, y: usize, z: usize) -> bool {
        assert!(x < 16, "x 坐标超出范围: {x}");
        assert!(y < 16, "y 坐标超出范围: {y}");
        assert!(z < 16, "z 坐标超出范围: {z}");
        self.cubes[x][y][z].is_some()
    }

    /// 清空整个区块
    pub fn clear(&mut self) {
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    self.cubes[x][y][z] = None;
                }
            }
        }
    }

    /// 统计区块中的方块数量
    pub fn count(&self) -> usize {
        self.cubes
            .iter()
            .flat_map(|layer| layer.iter())
            .flat_map(|column| column.iter())
            .filter(|slot| slot.is_some())
            .count()
    }

    /// 判断区块是否为空
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// 将区块渲染为 193x193 的图像
    pub fn render(&self) -> CubeBlockImage {
        let mut image = CubeBlockImage::default();

        // 区块定位点：底面最后面的方块 (x=0, y=0, z=0)
        // 区块图形定位点：左上角
        // x+1 → 向左下：屏幕 x-6, y+3
        // y+1 → 向右下：屏幕 x+6, y+3
        // z+1 → 向上：屏幕 y-8
        //
        // 方块 (0,0,0) 的屏幕位置：画布左上角 (0, 0)
        // z=15 时最高，y 偏移 = -15*8 = -120，需要 offset_y = 120
        // x=15 时最左，x 偏移 = -15*6 = -90，需要 offset_x = 90
        let offset_x: isize = 90;
        let offset_y: isize = 120;

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    if let Some(cube) = self.get(x, y, z) {
                        let cube_pixels: &CubeImage = cube.deref();

                        let screen_x = offset_x - (x as isize * 6) + (y as isize * 6);
                        let screen_y =
                            offset_y + (x as isize * 3) + (y as isize * 3) - (z as isize * 8);

                        for py in 0..13 {
                            for px in 0..13 {
                                let sx = screen_x + px as isize;
                                let sy = screen_y + py as isize;

                                if sx >= 0 && sx < 193 && sy >= 0 && sy < 193 {
                                    let src = &cube_pixels[py][px];
                                    if src.a > 0 {
                                        image.image.pixels[sy as usize][sx as usize] = src.clone();
                                    }
                                }
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

/// 显示 CubeBlockImage
impl CubeBlockImage {
    /// 调用系统默认图片查看器显示区块图像
    pub fn show(&self) {
        let tmp_path = std::env::temp_dir().join("cube_block_preview.png");
        self.save(&tmp_path).expect("无法保存预览图像");

        #[cfg(target_os = "macos")]
        Command::new("open").arg(&tmp_path).spawn().ok();

        #[cfg(target_os = "linux")]
        Command::new("xdg-open").arg(&tmp_path).spawn().ok();

        #[cfg(target_os = "windows")]
        Command::new("cmd")
            .args(["/c", "start", tmp_path.to_str().unwrap()])
            .spawn()
            .ok();
    }
}
