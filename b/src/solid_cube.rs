use std::{ops::Deref, path::PathBuf};

use crate::{concept::Cube, data::RGBAColor};

/// 图形构成的方块, 暂时从 PNG 文件加载, 因为没有设计资源包上下文
pub struct SolidCube {
    pub src_path: PathBuf,
}

impl Deref for SolidCube {
    type Target = [[RGBAColor; 13]; 13];

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl Cube for SolidCube {
    fn show(&self) {
        todo!()
    }
}
