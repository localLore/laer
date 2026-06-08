#![allow(dead_code)]
//! 定义方块和区块的概念.
//!
//! 目前只考虑静态的方块.
//!
//! 不同类型的方块的信息构成不同, 例如有些是纯色有些是流体有些是位图, 有些有内部参数有些没有, 但都可以实现 13x13 的图形, 占用区块中的一个位置.
use crate::data::{RGBAColor, RGBAImage};
use std::ops::Deref;

// 一个方块的图形, 固定为 13x13
type CubeImage = [[RGBAColor; 13]; 13];

/// 一个方块, 可以通过某种方式确定其图形
pub trait Cube: Deref<Target = CubeImage> {
    fn show(&self);
}

// 一个方块在区块上的占位
type CubeSlot = Option<Box<dyn Cube<Target = CubeImage>>>;

/// 区块, 包含 16x16x16 个方块, 每个方块的图形固定为 13x13
pub struct CubeBlock {
    pub cubes: [[[CubeSlot; 16]; 16]; 16],
}

/// 区块的图形, 固定为 193x193, 用于渲染整个区块的外观
pub struct CubeBlockImage {
    pub image: RGBAImage<193, 193>,
}

// 用某个背景颜色初始化区块的图形
impl CubeBlockImage {
    pub fn new(background_color: RGBAColor) -> Self {
        CubeBlockImage {
            image: RGBAImage::new(background_color),
        }
    }
}

/// 区块的图形的默认值是没有方块的虚空
impl Default for CubeBlockImage {
    fn default() -> Self {
        CubeBlockImage::new(RGBAColor::default())
    }
}
